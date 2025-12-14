#!/usr/bin/env python3
"""
Fusion AI Training Pipeline - GROUP 2: The Oracle
Development Assistant Training Script with LoRA

This script trains the Fusion Oracle model using instruction fine-tuning with LoRA.
"""

import os
import yaml
import torch
from transformers import (
    AutoTokenizer,
    AutoModelForCausalLM,
    BitsAndBytesConfig,
    TrainingArguments
)
from peft import LoraConfig, get_peft_model, prepare_model_for_kbit_training
from trl import SFTTrainer
from datasets import load_dataset
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class FusionOracleTrainer:
    def __init__(self, config_path="training_config.yaml"):
        with open(config_path, 'r') as f:
            self.config = yaml.safe_load(f)
        
        self.base_model_name = self.config['base_model']
        self.tokenizer = None
        self.model = None
        
    def load_model_with_quantization(self):
        """Load model with 4-bit quantization"""
        logger.info(f"Loading model: {self.base_model_name}")
        
        # Quantization config
        quant_config = self.config['quantization']
        bnb_config = BitsAndBytesConfig(
            load_in_4bit=quant_config['load_in_4bit'],
            bnb_4bit_quant_type=quant_config['bnb_4bit_quant_type'],
            bnb_4bit_compute_dtype=getattr(torch, quant_config['bnb_4bit_compute_dtype']),
            bnb_4bit_use_double_quant=quant_config['bnb_4bit_use_double_quant']
        )
        
        # Load tokenizer
        self.tokenizer = AutoTokenizer.from_pretrained(self.base_model_name)
        if self.tokenizer.pad_token is None:
            self.tokenizer.pad_token = self.tokenizer.eos_token
        self.tokenizer.padding_side = self.config['tokenizer_padding_side']
        
        # Load model
        self.model = AutoModelForCausalLM.from_pretrained(
            self.base_model_name,
            quantization_config=bnb_config,
            device_map="auto",
            trust_remote_code=True
        )
        
        # Prepare for k-bit training
        self.model = prepare_model_for_kbit_training(self.model)
        
        return self.model, self.tokenizer
    
    def setup_lora(self):
        """Configure LoRA adapters"""
        logger.info("Setting up LoRA configuration")
        
        lora_config_dict = self.config['lora']
        lora_config = LoraConfig(
            r=lora_config_dict['r'],
            lora_alpha=lora_config_dict['lora_alpha'],
            target_modules=lora_config_dict['target_modules'],
            lora_dropout=lora_config_dict['lora_dropout'],
            bias=lora_config_dict['bias'],
            task_type=lora_config_dict['task_type']
        )
        
        self.model = get_peft_model(self.model, lora_config)
        self.model.print_trainable_parameters()
        
        return self.model
    
    def load_instruction_dataset(self):
        """Load and format instruction dataset"""
        dataset_path = self.config['training']['dataset']
        logger.info(f"Loading instruction dataset: {dataset_path}")
        
        dataset = load_dataset("json", data_files=dataset_path, split="train")
        
        # Format with prompt template
        prompt_template = self.config['prompt_template']
        
        def format_instruction(example):
            """Format example using prompt template"""
            text = prompt_template.format(
                instruction=example['instruction'],
                input=example.get('input', ''),
                output=example['output']
            )
            return {"text": text}
        
        dataset = dataset.map(format_instruction, remove_columns=dataset.column_names)
        
        # Split train/val
        split_ratio = self.config['data']['train_val_split']
        seed = self.config['data']['seed']
        split = dataset.train_test_split(test_size=1-split_ratio, seed=seed)
        
        return split['train'], split['test']
    
    def train(self):
        """Execute instruction fine-tuning with LoRA"""
        logger.info("=== Starting Fusion Oracle Training ===")
        
        # Load model with quantization
        self.load_model_with_quantization()
        
        # Setup LoRA
        self.setup_lora()
        
        # Load dataset
        train_dataset, eval_dataset = self.load_instruction_dataset()
        
        # Training configuration
        train_config = self.config['training']
        
        # Training arguments
        training_args = TrainingArguments(
            output_dir=train_config['output_dir'],
            num_train_epochs=train_config['epochs'],
            per_device_train_batch_size=train_config['batch_size'],
            gradient_accumulation_steps=train_config['gradient_accumulation_steps'],
            learning_rate=train_config['learning_rate'],
            warmup_ratio=train_config['warmup_ratio'],
            logging_steps=train_config['logging_steps'],
            save_steps=train_config['save_steps'],
            eval_steps=train_config['eval_steps'],
            evaluation_strategy="steps",
            save_strategy="steps",
            fp16=False,
            bf16=True if self.config['hardware']['mixed_precision'] == 'bf16' else False,
            optim=train_config['optimizer'],
            lr_scheduler_type=train_config['scheduler'],
            weight_decay=train_config['weight_decay'],
            gradient_checkpointing=self.config['hardware']['gradient_checkpointing'],
            load_best_model_at_end=self.config['logging']['load_best_model_at_end'],
            metric_for_best_model=self.config['logging']['metric_for_best_model'],
            greater_is_better=self.config['logging']['greater_is_better'],
            report_to=self.config['logging'].get('report_to', []),
            group_by_length=self.config['advanced'].get('group_by_length', False),
            ddp_find_unused_parameters=self.config['advanced'].get('ddp_find_unused_parameters', False)
        )
        
        # SFT Trainer for instruction tuning
        trainer = SFTTrainer(
            model=self.model,
            args=training_args,
            train_dataset=train_dataset,
            eval_dataset=eval_dataset,
            tokenizer=self.tokenizer,
            dataset_text_field=self.config['sft_config']['dataset_text_field'],
            max_seq_length=self.config['sft_config']['max_seq_length'],
            packing=self.config['sft_config']['packing']
        )
        
        # Train
        logger.info("Starting training...")
        trainer.train()
        
        # Save final model
        logger.info(f"Saving model to {train_config['output_dir']}")
        trainer.save_model(train_config['output_dir'])
        self.tokenizer.save_pretrained(train_config['output_dir'])
        
        # Merge LoRA adapters if configured
        if self.config['post_training']['merge_adapters']:
            self.merge_and_save_model(trainer)
        
        logger.info("Training complete!")
    
    def merge_and_save_model(self, trainer):
        """Merge LoRA adapters back into base model"""
        logger.info("Merging LoRA adapters...")
        
        merged_dir = self.config['post_training']['output_merged_dir']
        
        # Merge adapters
        merged_model = trainer.model.merge_and_unload()
        
        # Save merged model
        merged_model.save_pretrained(merged_dir)
        self.tokenizer.save_pretrained(merged_dir)
        
        logger.info(f"Merged model saved to {merged_dir}")
    
    def test_generation(self, prompt):
        """Test model generation"""
        logger.info("Testing model generation...")
        
        inputs = self.tokenizer(prompt, return_tensors="pt").to(self.model.device)
        
        gen_config = self.config['generation']
        outputs = self.model.generate(
            **inputs,
            max_new_tokens=gen_config['max_new_tokens'],
            temperature=gen_config['temperature'],
            top_p=gen_config['top_p'],
            top_k=gen_config['top_k'],
            repetition_penalty=gen_config['repetition_penalty'],
            do_sample=gen_config['do_sample']
        )
        
        result = self.tokenizer.decode(outputs[0], skip_special_tokens=True)
        logger.info(f"Generated: {result}")
        return result


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Fusion Oracle Training Pipeline")
    parser.add_argument("--config", default="training_config.yaml", help="Path to config file")
    parser.add_argument("--test", type=str, help="Test generation with prompt")
    
    args = parser.parse_args()
    
    trainer = FusionOracleTrainer(args.config)
    
    if args.test:
        trainer.load_model_with_quantization()
        trainer.test_generation(args.test)
    else:
        trainer.train()
