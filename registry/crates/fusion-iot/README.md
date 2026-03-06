# fusion-iot

`fusion-iot` provides compiler-safe IoT protocol helpers for Fusion v2.0 Vortex.

## Implemented APIs

- `default_broker_port(use_tls: bool) -> int`
- `qos_from_priority(priority: int) -> int`
- `should_send_now(priority: int, battery_percent: int, critical: bool) -> bool`
- `classify_payload_size(payload_bytes: int) -> int`
- `encode_mqtt_fixed_header(packet_type: int, qos: int, retain: bool, dup: bool) -> int`
- `decode_packet_type(header_byte: int) -> int`
- `decode_qos(header_byte: int) -> int`
- `bool_as_int(flag: bool) -> int`

## Build

From the repository root on Windows:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/build_fusion_iot.ps1
```

This runs parse, semantic analysis, and object generation through the stable WSL `bin/fuc` compiler path and writes:

- `artifacts/fusion-iot/fusion_iot.o`
