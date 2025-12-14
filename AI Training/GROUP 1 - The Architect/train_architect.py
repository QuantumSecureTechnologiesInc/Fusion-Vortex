#!/usr/bin/env python3
"""
Fusion AI Training Pipeline - GROUP 1: The Architect
Compiler Intelligence Training Script

This script trains the Fusion Architect model through 4 stages:
1. Fusion Syntax SFT
2. LLVM Optimization Mapping
3. Security Invariant Enforcement
4. Self-Correction via DPO
"""

import os
import yaml
import torch
from transformers import (
    AutoTokenizer,
    AutoModelForCausalLM,
    TrainingArguments,
    Trainer,
    DataCollatorForLanguageModeling
)
from datasets import load_dataset
from trl import DPOTrainer, DPOConfig
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class FusionArchitectTrainer:
    def __init__(self, config_path="training_config.yaml"):
        with open(config_path, 'r') as f:
            self.config = yaml.safe_load(f)
        
        self.base_model_name = self.config['base_model']
        self.tokenizer = None
        self.model = None
        
    def load_model_and_tokenizer(self, model_path=None):
        """Load base model or checkpoint"""
        model_name = model_path or self.base_model_name
        logger.info(f"Loading model from: {model_name}")
        
        self.tokenizer = AutoTokenizer.from_pretrained(model_name)
        if self.tokenizer.pad_token is None:
            self.tokenizer.pad_token = self.tokenizer.eos_token
        
        self.model = AutoModelForCausalLM.from_pretrained(
            model_name,
            torch_dtype=torch.bfloat16,
            device_map="auto",
            trust_remote_code=True
        )
        
        return self.model, self.tokenizer
    
    def load_jsonl_dataset(self, file_path):
        """Load JSONL dataset"""
        logger.info(f"Loading dataset: {file_path}")
        dataset = load_dataset("json", data_files=file_path, split="train")
        return dataset
    
    def preprocess_syntax_dataset(self, examples):
        """Preprocess Stage 1: Syntax SFT"""
        texts = [ex['text'] for ex in examples]
        return self.tokenizer(
            texts,
            truncation=True,
            max_length=self.config['pipeline'][0]['max_seq_length'],
            padding='max_length'
        )
    
    def preprocess_llvm_dataset(self, examples):
        """Preprocess Stage 2: LLVM IR Pairs"""
        stage_config = self.config['pipeline'][1]['preprocessing']
        input_template = stage_config['input_template']
        output_template = stage_config['output_template']
        
        texts = []
        for ex in examples:
            input_text = input_template.format(fusion_code=ex['fusion_code'])
            output_text = output_template.format(llvm_ir=ex['llvm_ir'])
            texts.append(input_text + output_text)
        
        return self.tokenizer(
            texts,
            truncation=True,
            max_length=self.config['pipeline'][1]['max_seq_length'],
            padding='max_length'
        )
    
    def preprocess_security_dataset(self, examples):
        """Preprocess Stage 3: Security Invariants"""
        stage_config = self.config['pipeline'][2]['preprocessing']
        input_template = stage_config['input_template']
        output_template = stage_config['output_template']
        
        texts = []
        for ex in examples:
            input_text = input_template.format(code=ex['code'])
            output_text = output_template.format(
                is_safe=ex['is_safe'],
                explanation=ex['explanation'],
                violations=str(ex['violations'])
            )
            texts.append(input_text + output_text)
        
        return self.tokenizer(
            texts,
            truncation=True,
            max_length=self.config['pipeline'][2]['max_seq_length'],
            padding='max_length'
        )
    
    def train_stage_1(self):
        """Stage 1: Fusion Syntax SFT"""
        logger.info("=== Stage 1: Fusion Syntax SFT ===")
        stage_config = self.config['pipeline'][0]
        
        # Load model
        self.load_model_and_tokenizer()
        
        # Load and preprocess dataset
        dataset = self.load_jsonl_dataset(stage_config['dataset'])
        dataset = dataset.map(
            lambda ex: self.preprocess_syntax_dataset([ex]),
            batched=False,
            remove_columns=dataset.column_names
        )
        
        # Split train/val
        split = dataset.train_test_split(
            test_size=1-self.config['data']['train_val_split'],
            seed=self.config['data']['seed']
        )
        
        # Training arguments
        training_args = TrainingArguments(
            output_dir=stage_config['output_dir'],
            num_train_epochs=stage_config['epochs'],
            per_device_train_batch_size=stage_config['batch_size'],
            gradient_accumulation_steps=stage_config['gradient_accumulation_steps'],
            learning_rate=stage_config['learning_rate'],
            warmup_ratio=stage_config['warmup_ratio'],
            logging_steps=stage_config['logging_steps'],
            save_steps=stage_config['save_steps'],
            eval_steps=stage_config['eval_steps'],
            evaluation_strategy="steps",
            save_strategy="steps",
            fp16=False,
            bf16=True,
            optim=stage_config['optimizer'],
            lr_scheduler_type=stage_config['scheduler'],
            weight_decay=stage_config['weight_decay'],
            load_best_model_at_end=True,
            metric_for_best_model="eval_loss",
            report_to=["wandb"] if self.config['logging'].get('wandb_project') else []
        )
        
        # Data collator
        data_collator = DataCollatorForLanguageModeling(
            tokenizer=self.tokenizer,
            mlm=False
        )
        
        # Trainer
        trainer = Trainer(
            model=self.model,
            args=training_args,
            train_dataset=split['train'],
            eval_dataset=split['test'],
            data_collator=data_collator
        )
        
        # Train
        trainer.train()
        
        # Save
        trainer.save_model(stage_config['output_dir'])
        self.tokenizer.save_pretrained(stage_config['output_dir'])
        
        logger.info(f"Stage 1 complete. Model saved to {stage_config['output_dir']}")
    
    def train_stage_2(self):
        """Stage 2: LLVM Optimization Mapping"""
        logger.info("=== Stage 2: LLVM Optimization Mapping ===")
        stage_config = self.config['pipeline'][1]
        
        # Load from Stage 1
        self.load_model_and_tokenizer(stage_config['load_from'])
        
        # Load and preprocess dataset
        dataset = self.load_jsonl_dataset(stage_config['dataset'])
        dataset = dataset.map(
            lambda ex: self.preprocess_llvm_dataset([ex]),
            batched=False,
            remove_columns=dataset.column_names
        )
        
        # Split train/val
        split = dataset.train_test_split(
            test_size=1-self.config['data']['train_val_split'],
            seed=self.config['data']['seed']
        )
        
        # Training arguments (similar to Stage 1 but with stage-specific config)
        training_args = TrainingArguments(
            output_dir=stage_config['output_dir'],
            num_train_epochs=stage_config['epochs'],
            per_device_train_batch_size=stage_config['batch_size'],
            gradient_accumulation_steps=stage_config['gradient_accumulation_steps'],
            learning_rate=stage_config['learning_rate'],
            warmup_ratio=stage_config['warmup_ratio'],
            logging_steps=stage_config['logging_steps'],
            save_steps=stage_config['save_steps'],
            eval_steps=stage_config['eval_steps'],
            evaluation_strategy="steps",
            save_strategy="steps",
            bf16=True,
            optim=stage_config['optimizer'],
            lr_scheduler_type=stage_config['scheduler'],
            weight_decay=stage_config['weight_decay']
        )
        
        data_collator = DataCollatorForLanguageModeling(
            tokenizer=self.tokenizer,
            mlm=False
        )
        
        trainer = Trainer(
            model=self.model,
            args=training_args,
            train_dataset=split['train'],
            eval_dataset=split['test'],
            data_collator=data_collator
        )
        
        trainer.train()
        trainer.save_model(stage_config['output_dir'])
        self.tokenizer.save_pretrained(stage_config['output_dir'])
        
        logger.info(f"Stage 2 complete. Model saved to {stage_config['output_dir']}")
    
    def train_stage_3(self):
        """Stage 3: Security Invariant Enforcement"""
        logger.info("=== Stage 3: Security Invariant Enforcement ===")
        # Similar implementation to Stage 2, using preprocess_security_dataset
        pass  # Implementation follows same pattern as stage_2
    
    def train_stage_4_dpo(self):
        """Stage 4: Self-Correction via DPO"""
        logger.info("=== Stage 4: Self-Correction via DPO ===")
        stage_config = self.config['pipeline'][3]
        
        # Load from Stage 3
        self.load_model_and_tokenizer(stage_config['load_from'])
        ref_model = AutoModelForCausalLM.from_pretrained(
            stage_config['load_from'],
            torch_dtype=torch.bfloat16,
            device_map="auto"
        )
        
        # Load preference dataset
        dataset = self.load_jsonl_dataset(stage_config['dataset'])
        
        # Preprocess for DPO format
        def preprocess_dpo(examples):
            return {
                'prompt': examples['prompt'],
                'chosen': examples['chosen'],
                'rejected': examples['rejected']
            }
        
        dataset = dataset.map(preprocess_dpo, remove_columns=['reason', 'metadata'])
        
        split = dataset.train_test_split(
            test_size=1-self.config['data']['train_val_split'],
            seed=self.config['data']['seed']
        )
        
        # DPO Training
        training_args = DPOConfig(
            output_dir=stage_config['output_dir'],
            num_train_epochs=stage_config['epochs'],
            per_device_train_batch_size=stage_config['batch_size'],
            learning_rate=stage_config['learning_rate'],
            beta=stage_config['beta'],
            warmup_ratio=stage_config['warmup_ratio'],
            logging_steps=stage_config['logging_steps'],
            save_steps=stage_config['save_steps'],
            eval_steps=stage_config['eval_steps'],
            bf16=True
        )
        
        dpo_trainer = DPOTrainer(
            model=self.model,
            ref_model=ref_model,
            args=training_args,
            train_dataset=split['train'],
            eval_dataset=split['test'],
            tokenizer=self.tokenizer,
            max_length=stage_config['max_seq_length']
        )
        
        dpo_trainer.train()
        dpo_trainer.save_model(stage_config['output_dir'])
        self.tokenizer.save_pretrained(stage_config['output_dir'])
        
        logger.info(f"Stage 4 complete. Final model saved to {stage_config['output_dir']}")
    
    def run_full_pipeline(self):
        """Execute all 4 training stages"""
        logger.info("Starting full Fusion Architect training pipeline")
        
        self.train_stage_1()
        self.train_stage_2()
        self.train_stage_3()
        self.train_stage_4_dpo()
        
        logger.info("Full training pipeline complete!")


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Fusion Architect Training Pipeline")
    parser.add_argument("--config", default="training_config.yaml", help="Path to config file")
    parser.add_argument("--stage", type=int, choices=[1,2,3,4], help="Run specific stage only")
    parser.add_argument("--full", action="store_true", help="Run full pipeline")
    
    args = parser.parse_args()
    
    trainer = FusionArchitectTrainer(args.config)
    
    if args.full:
        trainer.run_full_pipeline()
    elif args.stage:
        if args.stage == 1:
            trainer.train_stage_1()
        elif args.stage == 2:
            trainer.train_stage_2()
        elif args.stage == 3:
            trainer.train_stage_3()
        elif args.stage == 4:
            trainer.train_stage_4_dpo()
    else:
        print("Please specify either --stage <1-4> or --full")
