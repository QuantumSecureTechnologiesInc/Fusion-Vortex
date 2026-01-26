# Vortex v2.0 - Monitoring Dashboards

## Overview
Two dashboards available for monitoring Vortex chaos health in real-time.

---

## 1. Terminal Dashboard

**File**: `tools/vortex_dashboard.c`  
**Type**: Native C application with color-coded terminal output

### Features:
- ✅ Real-time Lyapunov exponent display
- ✅ Color-coded status (GREEN/YELLOW/RED)
- ✅ ASCII bar graphs
- ✅ Live metrics:
  - Phase shifts counter
  - Collapse warnings
  - Self-heal operations
  - Entropy generation stats
- ✅ Updates every 500ms
- ✅ Clean, professional terminal UI

### Compile:
```bash
cd tools
gcc -o vortex_dashboard vortex_dashboard.c \
    -I../include \
    -L../build \
    -lhypercycle_vacuum \
    -lpthread -lssl -lcrypto -lm -mavx512f
```

### Run:
```bash
./vortex_dashboard
```

### Example Output:
```
╔════════════════════════════════════════════════════════════════════╗
║          VORTEX v2.0 - CHAOS MONITORING DASHBOARD                  ║
╚════════════════════════════════════════════════════════════════════╝

┌─ SYSTEM STATUS ────────────────────────────────────────────────────┐
│ Iteration: 42                                                      │
│ Status:    ● HEALTHY   (Chaos strong)                              │
└────────────────────────────────────────────────────────────────────┘

┌─ LYAPUNOV EXPONENT (Chaos Indicator) ──────────────────────────────┐
│ Current: 0.087432                                                  │
│ Target:  0.050000 (minimum for stable chaos)                      │
│ Visual:  [████████████████████░░░░░░░░░░░░░░░░░░░░]              │
└────────────────────────────────────────────────────────────────────┘

┌─ RECOVERY & HEALING METRICS ───────────────────────────────────────┐
│ Phase Shifts:      2          (ergodic phase space jumps)         │
│ Collapse Warnings: 0          (near-collapse detections)          │
│ Self-Heal Count:   0          (automatic recovery operations)     │
└────────────────────────────────────────────────────────────────────┘
```

---

## 2. Web Dashboard

**File**: `tools/dashboard.html`  
**Type**: Modern HTML/CSS/JavaScript web application

### Features:
- ✅ Stunning visual design with glassmorphism
- ✅ Real-time chart of Lyapunov exponent history
- ✅ Color-coded status indicators with animations
- ✅ Gradient backgrounds and smooth transitions
- ✅ Responsive layout
- ✅ Auto-updates every 500ms
- ✅ Professional metrics display

### Run:
```bash
# Option 1: Open directly in browser
open tools/dashboard.html

# Option 2: Serve with simple HTTP server
cd tools
python -m http.server 8080
# Then navigate to http://localhost:8080/dashboard.html
```

### Features Displayed:
1. **System Status Card**
   - Health indicator (HEALTHY/MARGINAL/CRITICAL)
   - Pulsing status dot
   - System uptime

2. **Lyapunov Exponent Card**
   - Current LLE value
   - Progress bar visualization
   - Target threshold indicator

3. **Recovery Metrics Card**
   - Phase shifts count
   - Collapse warnings
   - Self-heal operations

4. **Entropy Generation Card**
   - Total keys generated
   - Total batches
   - Last batch processing time

5. **Historical Chart**
   - Real-time line graph of LLE
   - 50-sample rolling window
   - Threshold line overlay
   - Smooth animations

---

## How to Access Monitoring Data

### Where to Call:
```c
#include "vortex_monitoring_api.h"

// In your application main loop or monitoring thread
hc_vac_context_t ctx;
hc_vacuum_init_context(&ctx, NULL);

while (monitoring) {
    // Generate entropy
    uint8_t seed[32];
    hc_vacuum_generate_seed_safe(ctx, seed);
    
    // GET TELEMETRY HERE ← This is where you call it
    hc_telemetry_extended_t stats;
    hc_vacuum_get_telemetry(ctx, (hc_telemetry_t*)&stats);
    
    // Use the data
    printf("LLE: %.6f\n", stats.lyapunov_exponent);
    printf("Phase Shifts: %llu\n", stats.phase_shifts);
    
    sleep(1);
}
```

### Where to Read:
The telemetry data is read from:
1. **Context structure** - `ctx->lyapunov.current_lle`
2. **Telemetry API** - `hc_vacuum_get_telemetry()` populates the struct
3. **Updated by** - Background worker thread (every 100µs)

### Data Flow:
```
Background Worker Thread (hc_vacuum_engine.c:179-225)
├─ Generates entropy sample
├─ Calls update_lyapunov_monitor()     ← Updates ctx->lyapunov
├─ Calls check_lyapunov_horizon()      ← Checks collapse
└─ Updates ctx->phase_shifts           ← Tracks recovery

User Thread (your application)
└─ Calls hc_vacuum_get_telemetry()     ← Reads ctx->lyapunov
   └─ Returns hc_telemetry_extended_t  ← Your monitoring data
```

---

## Integration Example

### C Application with Terminal Dashboard:
```bash
# 1. Build Vortex library
cd v2.0\ Vortex
mkdir build && cd build
cmake ..
make

# 2. Compile dashboard
cd ../tools
gcc -o vortex_dashboard vortex_dashboard.c ../build/libhypercycle.a \
    -I../include -lpthread -lssl -lcrypto -lm

# 3. Run
./vortex_dashboard
```

### Web Dashboard with Data Bridge:
```bash
# 1. Open dashboard in browser
open tools/dashboard.html

# 2. Dashboard shows simulated data
# 3. To connect to real Vortex data, create API bridge:
#    - Run vortex_dashboard in background
#    - Pipe telemetry to JSON file
#    - Fetch JSON from web dashboard via AJAX
```

---

## Summary

**Call**: `hc_vacuum_get_telemetry(ctx, &stats)`  
**Read**: `stats.lyapunov_exponent`, `stats.phase_shifts`, etc.  
**Display**: Terminal dashboard OR Web dashboard  
**Update Rate**: Every 500ms (configurable)

Both dashboards provide real-time visibility into Vortex v2.0's advanced chaos monitoring!


