# Fusion Runtime Core v2.0 - Python SDK

Official Python client SDK for interacting with the Fusion Runtime Core v2.0 (Nebula).

## Installation

```bash
pip install -r requirements.txt
```text

## Generating Protobuf Client Bindings

Before using the SDK, you need to generate the Python protobuf client bindings from the proto definitions:

```bash
python generate_proto.py
```text

This will create:
- `fusion_core_v2_pb2.py` - Protocol buffer message definitions
- `fusion_core_v2_pb2_grpc.py` - gRPC service clients

## Usage

```python
from fusion_client import FusionClient

# Create a client instance

client = FusionClient(host="localhost", port=50051)

# Check runtime health

health = client.check_health()
print(f"Runtime status: {health}")

# Execute a WASM plugin

result = client.execute_wasm(
    plugin_name="my-plugin",
    wasm_file_path="path/to/plugin.wasm",
    input_data="optional input data"
)
print(f"Plugin result: {result}")
```text

## API Reference

### FusionClient

#### `__init__(host="localhost", port=50051, client_id="python-sdk-v1")`

Creates a new client instance.

#### `check_health()`

Performs a health check on the runtime. Returns dict with:
- `status`: Runtime status string
- `version`: Runtime version
- `load`: Current load index

#### `execute_wasm(plugin_name, wasm_file_path, input_data="")`

Uploads and executes a WASM plugin. Returns dict with:
- `exit_code`: Plugin exit code
- `output`: Plugin output data
- `error`: Error message (if any)
- `time_ms`: Execution time in milliseconds

## Development

To regenerate protobuf files after updating the proto definitions:

```bash
python generate_proto.py
```text

## Requirements

- Python 3.7+
- grpcio >= 1.50.0
- grpcio-tools >= 1.50.0
- protobuf >= 4.21.0
