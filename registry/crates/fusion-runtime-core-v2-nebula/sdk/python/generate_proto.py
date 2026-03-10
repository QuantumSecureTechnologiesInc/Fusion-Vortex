#!/usr/bin/env python3
"""
Protobuf Code Generation Script for Fusion Runtime Core v2.0
Generates Python protobuf client bindings from the proto definitions.

Usage:
    python generate_proto.py

Requirements:
    pip install grpcio-tools
"""

import os
import sys
from pathlib import Path
from grpc_tools import protoc

def main():
    # Determine paths
    script_dir = Path(__file__).parent
    proto_dir = script_dir.parent.parent / "proto"
    output_dir = script_dir
    
    proto_file = proto_dir / "fusion_core_v2.proto"
    
    if not proto_file.exists():
        print(f"Error: Proto file not found at {proto_file}")
        sys.exit(1)
    
    print(f"Generating Python protobuf client bindings from {proto_file}...")
    print(f"Output directory: {output_dir}")
    
    # Run protoc to generate Python code
    result = protoc.main([
        'grpc_tools.protoc',
        f'--proto_path={proto_dir}',
        f'--python_out={output_dir}',
        f'--grpc_python_out={output_dir}',
        str(proto_file)
    ])
    
    if result == 0:
        print("✓ Successfully generated Python protobuf files:")
        print(f"  - {output_dir / 'fusion_core_v2_pb2.py'}")
        print(f"  - {output_dir / 'fusion_core_v2_pb2_grpc.py'}")
    else:
        print(f"✗ Failed to generate protobuf files (exit code: {result})")
        sys.exit(result)

if __name__ == "__main__":
    main()
