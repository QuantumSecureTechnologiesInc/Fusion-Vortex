// Fusion Standard Library entry (mirrors lib.fu)

extern fn malloc(size: int) -> int;
extern fn free(ptr: int) -> void;
extern fn realloc(ptr: int, size: int) -> int;
extern fn memcpy(dest: int, src: int, n: int) -> void;
extern fn strlen(s: string) -> int;
extern fn printf(fmt: string, val: int) -> int;
extern fn puts(s: string) -> int;
extern fn exit(code: int) -> void;
extern fn string_starts_with(s: string, prefix: string) -> bool;
extern fn fusion_read_line() -> string;
extern fn fusion_fs_read_to_string(path: string) -> string;
extern fn fusion_fs_write_str(path: string, contents: string) -> bool;
extern fn fusion_fs_append_str(path: string, contents: string) -> bool;
extern fn fusion_fs_exists(path: string) -> bool;
extern fn fusion_fs_create_dir(path: string) -> bool;
extern fn fusion_fs_remove_file(path: string) -> bool;
extern fn fusion_path_join(a: string, b: string) -> string;
extern fn fusion_path_basename(p: string) -> string;
extern fn fusion_path_dirname(p: string) -> string;
extern fn fusion_env_get(key: string) -> string;
extern fn fusion_argc() -> int;
extern fn fusion_argv(idx: int) -> string;
extern fn fusion_time_now_ms() -> int;
extern fn fusion_sleep_ms(ms: int) -> void;
extern fn fusion_rand_seed(seed: int) -> void;
extern fn fusion_rand_next() -> int;
extern fn fusion_hash32(s: string) -> int;
extern fn fusion_hmac32(key: string, msg: string) -> int;
extern fn fusion_fmt_int(v: int) -> string;
extern fn fusion_fmt_pair(k: string, v: string) -> string;
extern fn fusion_json_escape(s: string) -> string;
extern fn fusion_json_kv_string(k: string, v: string) -> string;
extern fn fusion_json_kv_int(k: string, v: int) -> string;
extern fn fusion_tcp_connect(host: string, port: int) -> int;
extern fn fusion_tcp_send_str(fd: int, data: string) -> int;
extern fn fusion_tcp_recv_str(fd: int, max_bytes: int) -> string;
extern fn fusion_tcp_close(fd: int) -> void;
extern fn fusion_udp_send_str(host: string, port: int, data: string) -> int;
extern fn fusion_udp_recv_str(port: int, max_bytes: int) -> string;
extern fn fusion_mutex_new() -> int;
extern fn fusion_mutex_lock(handle: int) -> void;
extern fn fusion_mutex_unlock(handle: int) -> void;
extern fn fusion_mutex_free(handle: int) -> void;

pub struct FInt { value: int }
pub struct FBool { value: bool }
pub struct FString { value: string }

pub fn f_int_new(v: int) -> FInt { let x: FInt = FInt { value: v }; return x; }
pub fn f_int_val(v: FInt) -> int { return v.value; }
pub fn f_bool_new(v: bool) -> FBool { let x: FBool = FBool { value: v }; return x; }
pub fn f_bool_val(v: FBool) -> bool { return v.value; }
pub fn f_string_new(v: string) -> FString { let x: FString = FString { value: v }; return x; }
pub fn f_string_val(v: FString) -> string { return v.value; }

pub struct FVecInt { data: [int; 1024], len: int }
pub fn fvec_new() -> FVecInt { let v: FVecInt = FVecInt { data: [0; 1024], len: 0 }; return v; }
pub fn fvec_len(v: FVecInt) -> int { return v.len; }
pub fn fvec_is_empty(v: FVecInt) -> bool { return v.len == 0; }
pub fn fvec_push(v: FVecInt, value: int) -> FVecInt { if v.len < 1024 { v.data[v.len] = value; v.len = v.len + 1; } return v; }
pub fn fvec_get(v: FVecInt, index: int) -> int { if v.len == 0 { return 0; } if index < 0 { return 0; } if index > v.len - 1 { return 0; } return v.data[index]; }

pub struct FusionContext { classical_sig: string, pq_sig: string }
pub fn fusion_ctx_new(classical: string, pq: string) -> FusionContext { let ctx: FusionContext = FusionContext { classical_sig: classical, pq_sig: pq }; return ctx; }
pub fn fusion_ctx_verify(ctx: FusionContext) -> bool { if strlen(ctx.classical_sig) > 0 { if strlen(ctx.pq_sig) > 0 { return true; } return false; } return false; }

pub fn print_line(s: string) -> void { puts(s); }
pub fn print_int(n: int) -> void { printf("%d\n", n); }

pub struct Verse { message: string }
pub struct Narrative { id: string, chapter: string, author: string, verse_count: int }
pub fn narrative_start(chapter: string, author: string) -> Narrative { let n: Narrative = Narrative { id: "narrative", chapter: chapter, author: author, verse_count: 0 }; return n; }
pub fn narrative_record(n: Narrative, _msg: string) -> Narrative { n.verse_count = n.verse_count + 1; return n; }
pub fn narrative_conclude_success(_n: Narrative, _summary: string) -> string { return "ok"; }

pub struct ComputeNode { id: string, reputation_score: int, is_active: bool, region: string }
pub fn compute_node_new(id: string, region: string) -> ComputeNode { let n: ComputeNode = ComputeNode { id: id, reputation_score: 100, is_active: true, region: region }; return n; }
pub fn compute_node_is_trustworthy(n: ComputeNode, min_score: int) -> bool { if n.reputation_score > min_score { return true; } return n.reputation_score == min_score; }

pub struct MarketBid { max_price_per_hour: int, currency_symbol: string }
pub fn market_bid_new(amount: int) -> MarketBid { let bid: MarketBid = MarketBid { max_price_per_hour: amount, currency_symbol: "$" }; return bid; }
pub fn market_bid_negotiate(bid: MarketBid, node: ComputeNode) -> bool { let node_rate: int = if node.reputation_score > 90 { 50 } else { 20 }; if bid.max_price_per_hour > node_rate { return true; } return bid.max_price_per_hour == node_rate; }

pub fn seal_sanitise(s: string) -> string { return s; }

pub fn alloc(size: int) -> int { return malloc(size); }
pub fn release(ptr: int) -> void { free(ptr); }
pub fn resize(ptr: int, size: int) -> int { return realloc(ptr, size); }
pub fn copy(dest: int, src: int, n: int) -> void { memcpy(dest, src, n); }
pub fn str_len(s: string) -> int { return strlen(s); }

pub fn panic(msg: string) -> void { print_line("PANIC:"); print_line(msg); exit(1); }

pub struct AiSession { provider: string, model: string, offline: bool }
pub struct LlmRuntime { model_path: string }

pub fn llm_runtime_new(model_path: string) -> LlmRuntime { let rt: LlmRuntime = LlmRuntime { model_path: model_path }; return rt; }
pub fn ai_session_new(provider: string, model: string, offline: bool) -> AiSession { let s: AiSession = AiSession { provider: provider, model: model, offline: offline }; return s; }
pub fn ai_session_provider(s: AiSession) -> string { return s.provider; }
pub fn ai_session_model(s: AiSession) -> string { return s.model; }
pub fn ai_session_offline(s: AiSession) -> bool { return s.offline; }
pub fn ai_session_from_env(provider: string, model: string, offline: bool) -> AiSession { return ai_session_new(provider, model, offline); }

pub struct RuntimeContext { env: int, region: string, version: string }
pub fn runtime_context_current() -> RuntimeContext { let ctx: RuntimeContext = RuntimeContext { env: 0, region: "local", version: "1.0.0" }; return ctx; }

pub struct CycleFailure { code: int, message: string, agreement_id: string, required: int, offered: int }
pub struct ServiceAgreement { id: string, provider: ComputeNode, payload_hash: string, state: int }
pub fn cycle_failure_contract_breach(agreement_id: string, reason: string) -> CycleFailure { let e: CycleFailure = CycleFailure { code: 1, message: reason, agreement_id: agreement_id, required: 0, offered: 0 }; return e; }
pub fn cycle_failure_insufficient_bid(required: int, offered: int) -> CycleFailure { let e: CycleFailure = CycleFailure { code: 2, message: "INSUFFICIENT_BID", agreement_id: "", required: required, offered: offered }; return e; }
pub fn agreement_draft(provider: ComputeNode, payload_hash: string) -> ServiceAgreement { let a: ServiceAgreement = ServiceAgreement { id: "agreement", provider: provider, payload_hash: payload_hash, state: 0 }; return a; }
pub fn agreement_sign_terms(a: ServiceAgreement, bid: MarketBid) -> ServiceAgreement { if a.state != 0 { return a; } if market_bid_negotiate(bid, a.provider) { a.state = 1; } return a; }
pub fn agreement_execute_workload(a: ServiceAgreement) -> string { if a.state != 1 { return "CONTRACT_NOT_SIGNED"; } return "EXECUTING"; }
pub fn agreement_settle(a: ServiceAgreement) -> ServiceAgreement { a.state = 3; return a; }

pub struct FluxState { id: string, tick: int }
pub fn flux_state_new(id: string) -> FluxState { let f: FluxState = FluxState { id: id, tick: 0 }; return f; }
pub fn flux_state_step(f: FluxState) -> FluxState { f.tick = f.tick + 1; return f; }

pub struct SecurityViolation { reason: string }
pub fn security_violation(reason: string) -> SecurityViolation { let v: SecurityViolation = SecurityViolation { reason: reason }; return v; }
pub fn ensure_system_operational(active: bool) -> bool { if active { return true; } return false; }
pub fn ensure_clearance(user_id: string) -> bool { if string_starts_with(user_id, "usr_") { return true; } return string_starts_with(user_id, "admin_"); }
pub fn sanitise(input: string) -> string { return input; }

pub fn stdio_println(s: string) -> void { puts(s); }
pub fn stdio_print(s: string) -> void { printf("%s", s); }
pub fn stdio_read_line() -> string { return fusion_read_line(); }

pub fn fs_read_to_string(path: string) -> string { return fusion_fs_read_to_string(path); }
pub fn fs_write_str(path: string, contents: string) -> bool { return fusion_fs_write_str(path, contents); }
pub fn fs_append_str(path: string, contents: string) -> bool { return fusion_fs_append_str(path, contents); }
pub fn fs_exists(path: string) -> bool { return fusion_fs_exists(path); }
pub fn fs_create_dir(path: string) -> bool { return fusion_fs_create_dir(path); }
pub fn fs_remove_file(path: string) -> bool { return fusion_fs_remove_file(path); }

pub fn path_join(a: string, b: string) -> string { return fusion_path_join(a, b); }
pub fn path_basename(p: string) -> string { return fusion_path_basename(p); }
pub fn path_dirname(p: string) -> string { return fusion_path_dirname(p); }
pub fn env_get(key: string) -> string { return fusion_env_get(key); }
pub fn env_argc() -> int { return fusion_argc(); }
pub fn env_argv(idx: int) -> string { return fusion_argv(idx); }

pub fn time_now_ms() -> int { return fusion_time_now_ms(); }
pub fn sleep_ms(ms: int) -> void { fusion_sleep_ms(ms); }

pub fn math_abs(v: int) -> int { if v < 0 { return 0 - v; } return v; }
pub fn math_min(a: int, b: int) -> int { if a < b { return a; } return b; }
pub fn math_max(a: int, b: int) -> int { if a > b { return a; } return b; }
pub fn math_clamp(v: int, lo: int, hi: int) -> int { if v < lo { return lo; } if v > hi { return hi; } return v; }

pub fn rand_seed(seed: int) -> void { fusion_rand_seed(seed); }
pub fn rand_next() -> int { return fusion_rand_next(); }

pub fn hash32(s: string) -> int { return fusion_hash32(s); }
pub fn hmac32(key: string, msg: string) -> int { return fusion_hmac32(key, msg); }

pub fn fmt_int(v: int) -> string { return fusion_fmt_int(v); }
pub fn fmt_pair(k: string, v: string) -> string { return fusion_fmt_pair(k, v); }
pub fn json_escape(s: string) -> string { return fusion_json_escape(s); }
pub fn json_kv_string(k: string, v: string) -> string { return fusion_json_kv_string(k, v); }
pub fn json_kv_int(k: string, v: int) -> string { return fusion_json_kv_int(k, v); }

pub fn tcp_connect(host: string, port: int) -> int { return fusion_tcp_connect(host, port); }
pub fn tcp_send(fd: int, data: string) -> int { return fusion_tcp_send_str(fd, data); }
pub fn tcp_recv(fd: int, max_bytes: int) -> string { return fusion_tcp_recv_str(fd, max_bytes); }
pub fn tcp_close(fd: int) -> void { fusion_tcp_close(fd); }
pub fn udp_send(host: string, port: int, data: string) -> int { return fusion_udp_send_str(host, port, data); }
pub fn udp_recv(port: int, max_bytes: int) -> string { return fusion_udp_recv_str(port, max_bytes); }

pub fn process_exit(code: int) -> void { exit(code); }

pub fn mutex_new() -> int { return fusion_mutex_new(); }
pub fn mutex_lock(handle: int) -> void { fusion_mutex_lock(handle); }
pub fn mutex_unlock(handle: int) -> void { fusion_mutex_unlock(handle); }
pub fn mutex_free(handle: int) -> void { fusion_mutex_free(handle); }

pub fn assert_true(v: bool, msg: string) -> void { if v { return; } panic(msg); }
pub fn assert_eq_int(a: int, b: int, msg: string) -> void { if a == b { return; } panic(msg); }
pub fn assert_eq_string(a: string, b: string, msg: string) -> void { if a == b { return; } panic(msg); }
