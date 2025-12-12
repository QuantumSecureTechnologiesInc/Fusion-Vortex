# Fusion Package Registry - Batch Crate Generator
# Generates 50 crates from source files into registry/crates/

$ErrorActionPreference = "Stop"

$SourceDir = "C:\Projects\Fusion - Programming Language\Source Files\Ecosystem\Fusion Crates"
$RegistryDir = "C:\Projects\Fusion - Programming Language\registry\crates"

# Ensure registry directory exists
New-Item -ItemType Directory -Force -Path $RegistryDir | Out-Null

# Define the 50 crates to create
$CrateDefinitions = @(
    @{Name = "q-measurement-opt"; Config = "Measurement Opt Config.txt"; Lib = "crates_q-measurement-opt_src_lib.rs.txt" },
    @{Name = "q-optimizer-hybrid"; Config = "Optimization Config.txt"; Lib = "crates_q-optimizer-hybrid_src_lib.rs.txt" },
    @{Name = "q-pulse-seq"; Config = "Pulse Seq Config.txt"; Lib = "crates_q-pulse-seq_src_lib.rs.txt" },
    @{Name = "q-algo"; Config = ""; Lib = "crates_q-algo_src_qft.rs.txt" },
    @{Name = "q-sim"; Config = ""; Lib = "crates_q-sim_src_noise.rs.txt" },
    @{Name = "q-visualization"; Config = "Quantum Vis Config.txt"; Lib = "crates_q-visualization_src_lib.rs.txt" },
    @{Name = "sec-forensics"; Config = ""; Lib = "crates_sec-forensics_src_analyzer.rs.txt" },
    @{Name = "sec-incident-response"; Config = "Incident Response Config.txt"; Lib = "crates_sec-incident-response_src_lib.rs.txt" },
    @{Name = "sec-network-segmentation"; Config = "Network Segmentation Config.txt"; Lib = "crates_sec-network-segmentation_src_lib.rs.txt" },
    @{Name = "sec-os-hardener"; Config = "OS Hardener Config.txt"; Lib = "crates_sec-os-hardener_src_lib.rs.txt" },
    @{Name = "sec-penetration"; Config = ""; Lib = "crates_sec-penetration_src_lib.rs.txt" },
    @{Name = "sec-policy-compiler"; Config = "Policy Compiler Config.txt"; Lib = "crates_sec-policy-compiler_src_lib.rs.txt" },
    @{Name = "sec-runtime-policy"; Config = ""; Lib = "crates_sec-runtime-policy_src_lib.rs.txt" },
    @{Name = "sec-secrets-auditor"; Config = "Secrets Auditor Config.txt"; Lib = "crates_sec-secrets-auditor_src_lib.rs.txt" },
    @{Name = "sec-threat-intel"; Config = "Threat Intel Config.txt"; Lib = "crates_sec-threat-intel_src_lib.rs.txt" },
    @{Name = "cloud-aws"; Config = ""; Lib = "crates_cloud-aws_src_s3.rs.txt" },
    @{Name = "cloud-azure"; Config = ""; Lib = "crates_cloud-azure_src_blob.rs.txt" },
    @{Name = "cloud-gcp"; Config = ""; Lib = "crates_cloud-gcp_src_storage.rs.txt" },
    @{Name = "interop-java"; Config = ""; Lib = "crates_interop_java_src_lib.rs.txt" },
    @{Name = "interop-js"; Config = ""; Lib = "crates_interop_js_src_lib.rs.txt" },
    @{Name = "interop-python"; Config = ""; Lib = "crates_interop_python_src_lib.rs.txt" },
    @{Name = "llm-llama"; Config = ""; Lib = "crates_llm-llama_src_attention.rs.txt" },
    @{Name = "llm-quantization"; Config = "Quantization Fusion Config.txt"; Lib = "" },
    @{Name = "llm-beam-search"; Config = "Beam Search Config.txt"; Lib = "" },
    @{Name = "llm-prompt-tuning"; Config = "Prompt Tuning Config.txt"; Lib = "Prompt Tuning Logic.txt" },
    @{Name = "llm-distillation"; Config = "Distill Config.txt"; Lib = "Distillation Logic.txt" },
    @{Name = "llm-rlhf"; Config = ""; Lib = "RLHF Framework.txt" },
    @{Name = "llm-distributed-training"; Config = ""; Lib = "Distributed Training Framework.txt" },
    @{Name = "llm-lora-manager"; Config = "LORA Manager Config.txt"; Lib = "LORA Manager.txt" },
    @{Name = "nn-layer-norm"; Config = "Norm Crate Config.txt"; Lib = "Layer Normalization.txt" },
    @{Name = "nn-maxpool"; Config = "Pooling Crate Config.txt"; Lib = "MaxPool Implementation.txt" },
    @{Name = "nn-rnn"; Config = "RNN Crate Config.txt"; Lib = "" },
    @{Name = "nn-gnn"; Config = "GNN Crate Config.txt"; Lib = "GCN Implementation.txt" },
    @{Name = "nn-lstm"; Config = ""; Lib = "LSTM Implementation.txt" },
    @{Name = "nn-pooling"; Config = "Pooling Crate Config.txt"; Lib = "" },
    @{Name = "event-bus"; Config = "Event Bus Config.txt"; Lib = "Event Bus Implementation.txt" },
    @{Name = "faas"; Config = "FaaS Config.txt"; Lib = "FaaS Implementation.txt" },
    @{Name = "graphql"; Config = "GraphQL Config.txt"; Lib = "GraphQL Implementation.txt" },
    @{Name = "rest-server"; Config = "REST Server Config.txt"; Lib = "REST Server Implementation.txt" },
    @{Name = "grpc"; Config = "gRPC Config.txt"; Lib = "gRPC Implementation.txt" },
    @{Name = "rate-limiter"; Config = "Rate Limiter Config.txt"; Lib = "Rate Limiter Implementation.txt" },
    @{Name = "schema-validator"; Config = "Schema Validator Config.txt"; Lib = "Schema Validator Implementation.txt" },
    @{Name = "router-mesh"; Config = "Router Mesh Config.txt"; Lib = "" },
    @{Name = "sbom-generator"; Config = ""; Lib = "SBOM Generator.txt" },
    @{Name = "sdk-generator"; Config = "SDK Generator Config.txt"; Lib = "SDK Generator Implementation.txt" },
    @{Name = "sandbox-manager"; Config = "Sandbox Config.txt"; Lib = "Sandbox Manager.txt" },
    @{Name = "safety-monitor"; Config = ""; Lib = "Safety Monitor.txt" },
    @{Name = "telemetry-ingestor"; Config = ""; Lib = "Telemetry Ingestor.txt" },
    @{Name = "vram-scheduler"; Config = ""; Lib = "VRAM Scheduler.txt" },
    @{Name = "offload"; Config = "Offload Config.txt"; Lib = "" }
)

Write-Host "=== Fusion Registry Crate Generator ===" -ForegroundColor Cyan
Write-Host "Generating 50 crates..." -ForegroundColor Yellow

$successCount = 0
$errorCount = 0

foreach ($crate in $CrateDefinitions) {
    $crateName = $crate.Name
    $crateDir = Join-Path $RegistryDir $crateName
    $srcDir = Join-Path $crateDir "src"
    
    try {
        Write-Host "`nProcessing: $crateName" -ForegroundColor Green
        
        # Create directories
        New-Item -ItemType Directory -Force -Path $srcDir | Out-Null
        
        # Generate or read Cargo.toml
        $cargoPath = Join-Path $crateDir "Cargo.toml"
        $packageName = "fusion_" + ($crateName -replace '-', '_')
        
        if ($crate.Config -and (Test-Path (Join-Path $SourceDir $crate.Config))) {
            $configContent = Get-Content (Join-Path $SourceDir $crate.Config) -Raw
            # Fix paths
            $configContent = $configContent -replace 'path = "\.\./core"', 'path = "../../crates/core"'
            $configContent = $configContent -replace 'path = "\.\./ai-core"', 'path = "../../ecosystem/crates/ai-core"'
            Set-Content -Path $cargoPath -Value $configContent -NoNewline
        }
        else {
            $cargoContent = "[package]`nname = `"$packageName`"`nversion = `"0.1.0`"`nedition = `"2021`"`nlicense = `"MIT`"`n`n[dependencies]`nfusion_core = { path = `"../../crates/core`" }`n"
            Set-Content -Path $cargoPath -Value $cargoContent -NoNewline
        }
        
        # Generate or read lib.rs
        $libPath = Join-Path $srcDir "lib.rs"
        if ($crate.Lib -and (Test-Path (Join-Path $SourceDir $crate.Lib))) {
            $libContent = Get-Content (Join-Path $SourceDir $crate.Lib) -Raw
            Set-Content -Path $libPath -Value $libContent -NoNewline
        }
        else {
            $libContent = "/// $crateName implementation`n`npub struct ${crateName}Module;`n"
            Set-Content -Path $libPath -Value $libContent -NoNewline
        }
        
        Write-Host "  Created $crateName" -ForegroundColor DarkGreen
        $successCount++
        
    }
    catch {
        Write-Host "  Error: $_" -ForegroundColor Red
        $errorCount++
    }
}

Write-Host "`n=== Summary ===" -ForegroundColor Cyan
Write-Host "Created: $successCount / $($CrateDefinitions.Count) crates" -ForegroundColor Green
Write-Host "Location: $RegistryDir" -ForegroundColor Yellow
