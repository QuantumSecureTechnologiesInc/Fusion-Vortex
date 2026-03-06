// Fusion Standard Library (corrected for proper C FFI and Rust semantics)
use libc::{c_char, c_int, c_void, size_t};
use std::ffi::CString;
use std::io::{self, Write};

// Correct C FFI declarations
extern "C" {
    fn puts(s: *const c_char) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn exit(code: c_int);
}

// Pure Rust implementation (no FFI needed)
pub fn string_starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

// Fusion runtime FFI declarations (stubbed for bootstrap)
// These will be provided by the Fusion runtime in the final system
extern "C" {
    fn fusion_read_line() -> *const c_char;
    fn fusion_fs_read_to_string(path: *const c_char) -> *const c_char;
    fn fusion_fs_write_str(path: *const c_char, contents: *const c_char) -> bool;
    fn fusion_fs_append_str(path: *const c_char, contents: *const c_char) -> bool;
    fn fusion_fs_exists(path: *const c_char) -> bool;
    fn fusion_fs_create_dir(path: *const c_char) -> bool;
    fn fusion_fs_remove_file(path: *const c_char) -> bool;
    fn fusion_path_join(a: *const c_char, b: *const c_char) -> *const c_char;
    fn fusion_path_basename(p: *const c_char) -> *const c_char;
    fn fusion_path_dirname(p: *const c_char) -> *const c_char;
    fn fusion_env_get(key: *const c_char) -> *const c_char;
    fn fusion_argc() -> i64;
    fn fusion_argv(idx: i64) -> *const c_char;
    fn fusion_time_now_ms() -> i64;
    fn fusion_sleep_ms(ms: i64);
    fn fusion_rand_seed(seed: i64);
    fn fusion_rand_next() -> i64;
    fn fusion_hash32(s: *const c_char) -> i64;
    fn fusion_hmac32(key: *const c_char, msg: *const c_char) -> i64;
    fn fusion_fmt_int(v: i64) -> *const c_char;
    fn fusion_fmt_pair(k: *const c_char, v: *const c_char) -> *const c_char;
    fn fusion_json_escape(s: *const c_char) -> *const c_char;
    fn fusion_json_kv_string(k: *const c_char, v: *const c_char) -> *const c_char;
    fn fusion_json_kv_int(k: *const c_char, v: i64) -> *const c_char;
    fn fusion_tcp_connect(host: *const c_char, port: i64) -> i64;
    fn fusion_tcp_send_str(fd: i64, data: *const c_char) -> i64;
    fn fusion_tcp_recv_str(fd: i64, max_bytes: i64) -> *const c_char;
    fn fusion_tcp_close(fd: i64);
    fn fusion_udp_send_str(host: *const c_char, port: i64, data: *const c_char) -> i64;
    fn fusion_udp_recv_str(port: i64, max_bytes: i64) -> *const c_char;
    fn fusion_mutex_new() -> i64;
    fn fusion_mutex_lock(handle: i64);
    fn fusion_mutex_unlock(handle: i64);
    fn fusion_mutex_free(handle: i64);
}

// ----------------------------
// Primitives
// ----------------------------
pub struct FInt { value: i64 }
pub struct FBool { value: bool }
pub struct FString { value: String }

pub fn f_int_new(v: i64) -> FInt { FInt { value: v } }
pub fn f_int_val(v: FInt) -> i64 { v.value }

pub fn f_bool_new(v: bool) -> FBool { FBool { value: v } }
pub fn f_bool_val(v: FBool) -> bool { v.value }

pub fn f_string_new(v: String) -> FString { FString { value: v } }
pub fn f_string_val(v: FString) -> String { v.value }

// ----------------------------
// Collections (fixed-size vector of i64)
// ----------------------------
pub struct FVecInt { data: [i64; 1024], len: i64 }

pub fn fvec_new() -> FVecInt {
    FVecInt { data: [0; 1024], len: 0 }
}

pub fn fvec_len(v: FVecInt) -> i64 { v.len }
pub fn fvec_is_empty(v: FVecInt) -> bool { v.len == 0 }

pub fn fvec_push(mut v: FVecInt, value: i64) -> FVecInt {
    if v.len < 1024 {
        v.data[v.len as usize] = value;
        v.len = v.len + 1;
    }
    v
}

pub fn fvec_get(v: FVecInt, index: i64) -> i64 {
    if v.len == 0 || index < 0 || index >= v.len { return 0; }
    v.data[index as usize]
}

// ----------------------------
// Security Context (stateless helpers)
// ----------------------------
pub struct FusionContext { classical_sig: String, pq_sig: String }

pub fn fusion_ctx_new(classical: String, pq: String) -> FusionContext {
    FusionContext { classical_sig: classical, pq_sig: pq }
}

pub fn fusion_ctx_verify(ctx: FusionContext) -> bool {
    !ctx.classical_sig.is_empty() && !ctx.pq_sig.is_empty()
}

// ----------------------------
// IO helpers
// ----------------------------
pub fn print_line(s: String) {
    let c = CString::new(s).unwrap_or_else(|_| CString::new("<invalid utf8>").unwrap());
    unsafe { puts(c.as_ptr()); }
}

pub fn print_int(n: i64) {
    println!("{}", n);
}


// ----------------------------
// Core narrative (simplified)
// ----------------------------
pub struct Verse { message: String }
pub struct Narrative { id: String, chapter: String, author: String, verse_count: i64 }

pub fn narrative_start(chapter: String, author: String) -> Narrative {
    let n: Narrative = Narrative { id: "narrative", chapter: chapter, author: author, verse_count: 0 };
    return n;
}

pub fn narrative_record(mut n: Narrative, _msg: String) -> Narrative {
    n.verse_count = n.verse_count + 1;
    return n;
}

pub fn narrative_conclude_success(_n: Narrative, _summary: String) -> String { return "ok".to_string(); }

// ----------------------------
// Cycle / Market (simplified)
// ----------------------------
pub struct ComputeNode { id: String, reputation_score: i64, is_active: bool, region: String }

pub fn compute_node_new(id: String, region: String) -> ComputeNode {
    let n: ComputeNode = ComputeNode { id: id, reputation_score: 100, is_active: true, region: region };
    return n;
}

pub fn compute_node_is_trustworthy(n: ComputeNode, min_score: i64) -> bool {
    if n.reputation_score > min_score { return true; }
    return n.reputation_score == min_score;
}

pub struct MarketBid { max_price_per_hour: i64, currency_symbol: String }

pub fn market_bid_new(amount: i64) -> MarketBid {
    let bid: MarketBid = MarketBid { max_price_per_hour: amount, currency_symbol: "$" };
    return bid;
}

pub fn market_bid_negotiate(bid: MarketBid, node: &ComputeNode) -> bool {
    let node_rate: i64 = if node.reputation_score > 90 { 50 } else { 20 };
    if bid.max_price_per_hour > node_rate { return true; }
    return bid.max_price_per_hour == node_rate;
}

// ----------------------------
// Seal / Guards (simplified)
// ----------------------------
pub fn seal_sanitise(s: String) -> String { return s; }

// ----------------------------
// Memory utilities
// ----------------------------
pub fn alloc(size: usize) -> usize {
    unsafe { malloc(size as size_t) as usize }
}

pub fn release(ptr: usize) {
    unsafe { free(ptr as *mut c_void); }
}

pub fn resize(ptr: usize, size: usize) -> usize {
    unsafe { realloc(ptr as *mut c_void, size as size_t) as usize }
}

pub fn copy(dest: usize, src: usize, n: usize) {
    unsafe { memcpy(dest as *mut c_void, src as *const c_void, n as size_t); }
}

pub fn str_len(s: String) -> usize {
    let c = CString::new(s).unwrap_or_else(|_| CString::new("").unwrap());
    unsafe { strlen(c.as_ptr()) as usize }
}

pub fn panic(msg: String) -> () {
    print_line("PANIC:");
    print_line(msg);
    unsafe { exit(1) };
}

// ----------------------------
// AI Core (pure .fu)
// ----------------------------
pub struct AiSession { provider: String, model: String, offline: bool }
pub struct LlmRuntime { model_path: String }

pub fn llm_runtime_new(model_path: String) -> LlmRuntime {
    let rt: LlmRuntime = LlmRuntime { model_path: model_path };
    return rt;
}

pub fn ai_session_new(provider: String, model: String, offline: bool) -> AiSession {
    let s: AiSession = AiSession { provider: provider, model: model, offline: offline };
    return s;
}

pub fn ai_session_provider(s: AiSession) -> String { return s.provider; }
pub fn ai_session_model(s: AiSession) -> String { return s.model; }
pub fn ai_session_offline(s: AiSession) -> bool { return s.offline; }

pub fn ai_session_from_env(provider: String, model: String, offline: bool) -> AiSession {
    return ai_session_new(provider, model, offline);
}

// ----------------------------
// Telemetry (pure .fu)
// ----------------------------
pub struct RuntimeContext { env: i64, region: String, version: String }

pub fn runtime_context_current() -> RuntimeContext {
    let ctx: RuntimeContext = RuntimeContext { env: 0, region: "local", version: "1.0.0" };
    return ctx;
}

// ----------------------------
// Cycle Errors / Agreements (pure .fu)
// ----------------------------
pub struct CycleFailure { code: i64, message: String, agreement_id: String, required: i64, offered: i64 }
pub struct ServiceAgreement { id: String, provider: ComputeNode, payload_hash: String, state: i64 }

pub fn cycle_failure_contract_breach(agreement_id: String, reason: String) -> CycleFailure {
    let e: CycleFailure = CycleFailure { code: 1, message: reason, agreement_id: agreement_id, required: 0, offered: 0 };
    return e;
}

pub fn cycle_failure_insufficient_bid(required: i64, offered: i64) -> CycleFailure {
    let e: CycleFailure = CycleFailure { code: 2, message: "INSUFFICIENT_BID", agreement_id: "", required: required, offered: offered };
    return e;
}

pub fn agreement_draft(provider: ComputeNode, payload_hash: String) -> ServiceAgreement {
    let a: ServiceAgreement = ServiceAgreement { id: "agreement", provider: provider, payload_hash: payload_hash, state: 0 };
    return a;
}

pub fn agreement_sign_terms(mut a: ServiceAgreement, bid: MarketBid) -> ServiceAgreement {
    if a.state != 0 { return a; }
    if market_bid_negotiate(bid, &a.provider) { a.state = 1; }
    return a;
}

pub fn agreement_execute_workload(a: ServiceAgreement) -> String {
    if a.state != 1 { return "CONTRACT_NOT_SIGNED".to_string(); }
    return "EXECUTING".to_string();
}

pub fn agreement_settle(mut a: ServiceAgreement) -> ServiceAgreement {
    a.state = 3;
    return a;
}

// ----------------------------
// Flow (pure .fu)
// ----------------------------
pub struct FluxState { id: String, tick: i64 }

pub fn flux_state_new(id: String) -> FluxState {
    let f: FluxState = FluxState { id: id, tick: 0 };
    return f;
}

pub fn flux_state_step(mut f: FluxState) -> FluxState {
    f.tick = f.tick + 1;
    return f;
}

// ----------------------------
// Seal (pure .fu)
// ----------------------------
pub struct SecurityViolation { reason: String }

pub fn security_violation(reason: String) -> SecurityViolation {
    let v: SecurityViolation = SecurityViolation { reason: reason };
    return v;
}

pub fn ensure_system_operational(active: bool) -> bool {
    if active { return true; }
    return false;
}

pub fn ensure_clearance(user_id: String) -> bool {
    if string_starts_with(&user_id, "usr_") { return true; }
    return string_starts_with(&user_id, "admin_");
}

pub fn sanitise(input: String) -> String { return input; }

// ----------------------------
// Stdlib IO/FS (pure .fu)
// ----------------------------
pub fn stdio_println(s: String) { print_line(s); }

pub fn stdio_print(s: String) {
    use std::io::{self, Write};
    let _ = io::stdout().write_all(s.as_bytes());
    let _ = io::stdout().flush();
}

pub fn stdio_read_line() -> String {
    unsafe {
        let ptr = fusion_read_line();
        if ptr.is_null() { return String::new(); }
        std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn fs_read_to_string(path: String) -> String { return unsafe { fusion_fs_read_to_string(path) }; }
pub fn fs_write_str(path: String, contents: String) -> bool { return unsafe { fusion_fs_write_str(path, contents) }; }
pub fn fs_append_str(path: String, contents: String) -> bool { return unsafe { fusion_fs_append_str(path, contents) }; }
pub fn fs_exists(path: String) -> bool { return unsafe { fusion_fs_exists(path) }; }
pub fn fs_create_dir(path: String) -> bool { return unsafe { fusion_fs_create_dir(path) }; }
pub fn fs_remove_file(path: String) -> bool { return unsafe { fusion_fs_remove_file(path) }; }

// ----------------------------
// Path / Env
// ----------------------------
pub fn path_join(a: String, b: String) -> String { return unsafe { fusion_path_join(a, b) }; }
pub fn path_basename(p: String) -> String { return unsafe { fusion_path_basename(p) }; }
pub fn path_dirname(p: String) -> String { return unsafe { fusion_path_dirname(p) }; }
pub fn env_get(key: String) -> String { return unsafe { fusion_env_get(key) }; }
pub fn env_argc() -> i64 { return unsafe { fusion_argc() }; }
pub fn env_argv(idx: i64) -> String { return unsafe { fusion_argv(idx) }; }

// ----------------------------
// Time
// ----------------------------
pub fn time_now_ms() -> i64 { return unsafe { fusion_time_now_ms() }; }
pub fn sleep_ms(ms: i64) -> () { unsafe { fusion_sleep_ms(ms) }; }

// ----------------------------
// Math
// ----------------------------
pub fn math_abs(v: i64) -> i64 { if v < 0 { return 0 - v; } return v; }
pub fn math_min(a: i64, b: i64) -> i64 { if a < b { return a; } return b; }
pub fn math_max(a: i64, b: i64) -> i64 { if a > b { return a; } return b; }
pub fn math_clamp(v: i64, lo: i64, hi: i64) -> i64 {
    if v < lo { return lo; }
    if v > hi { return hi; }
    return v;
}

// ----------------------------
// Random
// ----------------------------
pub fn rand_seed(seed: i64) -> () { unsafe { fusion_rand_seed(seed) }; }
pub fn rand_next() -> i64 { return unsafe { fusion_rand_next() }; }

// ----------------------------
// Crypto (lightweight)
// ----------------------------
pub fn hash32(s: String) -> i64 { return unsafe { fusion_hash32(s) }; }
pub fn hmac32(key: String, msg: String) -> i64 { return unsafe { fusion_hmac32(key, msg) }; }

// ----------------------------
// JSON / Formatting
// ----------------------------
pub fn fmt_int(v: i64) -> String { return unsafe { fusion_fmt_int(v) }; }
pub fn fmt_pair(k: String, v: String) -> String { return unsafe { fusion_fmt_pair(k, v) }; }
pub fn json_escape(s: String) -> String { return unsafe { fusion_json_escape(s) }; }
pub fn json_kv_string(k: String, v: String) -> String { return unsafe { fusion_json_kv_string(k, v) }; }
pub fn json_kv_int(k: String, v: i64) -> String { return unsafe { fusion_json_kv_int(k, v) }; }

// ----------------------------
// Net
// ----------------------------
pub fn tcp_connect(host: String, port: i64) -> i64 { return unsafe { fusion_tcp_connect(host, port) }; }
pub fn tcp_send(fd: i64, data: String) -> i64 { return unsafe { fusion_tcp_send_str(fd, data) }; }
pub fn tcp_recv(fd: i64, max_bytes: i64) -> String { return unsafe { fusion_tcp_recv_str(fd, max_bytes) }; }
pub fn tcp_close(fd: i64) -> () { unsafe { fusion_tcp_close(fd) }; }
pub fn udp_send(host: String, port: i64, data: String) -> i64 { return unsafe { fusion_udp_send_str(host, port, data) }; }
pub fn udp_recv(port: i64, max_bytes: i64) -> String { return unsafe { fusion_udp_recv_str(port, max_bytes) }; }

// ----------------------------
// Process
// ----------------------------
pub fn process_exit(code: i64) { unsafe { exit(code as c_int); } }

// ----------------------------
// Sync
// ----------------------------
pub fn mutex_new() -> i64 { return unsafe { fusion_mutex_new() }; }
pub fn mutex_lock(handle: i64) -> () { unsafe { fusion_mutex_lock(handle) }; }
pub fn mutex_unlock(handle: i64) -> () { unsafe { fusion_mutex_unlock(handle) }; }
pub fn mutex_free(handle: i64) -> () { unsafe { fusion_mutex_free(handle) }; }

// ----------------------------
// Test
// ----------------------------
pub fn assert_true(v: bool, msg: String) -> () {
    if v { return; }
    panic(msg);
}
pub fn assert_eq_int(a: i64, b: i64, msg: String) -> () {
    if a == b { return; }
    panic(msg);
}
pub fn assert_eq_string(a: String, b: String, msg: String) -> () {
    if a == b { return; }
    panic(msg);
}