/// `fusion_if!`: Secure control flow gated by the active security context.
#[macro_export]
macro_rules! fusion_if {
    ($condition:expr, $block:block) => {
        if $crate::security::verify_current_context() && $condition { $block }
    };
    ($condition:expr, $block:block else $else_block:block) => {
        if $crate::security::verify_current_context() && $condition { $block } else {
        $else_block }
    };
}
/// `fusion_match!`: Pattern matching that requires a valid security context.
#[macro_export]
macro_rules! fusion_match {
    ($expression:expr, { $($pattern:pat => $body:expr),* $(,)? }) => {
        if $crate::security::verify_current_context() { match $expression { $($pattern =>
        $body,)* } } else {
        panic!("FUSION ACCESS DENIED: Missing or invalid security context."); }
    };
}
/// `fusion_loop!`: Re-verifies security on every iteration.
#[macro_export]
macro_rules! fusion_loop {
    ($body:block) => {
        loop { if !$crate::security::verify_current_context() { break; } $body }
    };
}
/// Records a simple verse in the current story.
#[macro_export]
macro_rules! verse {
    ($story:expr, $msg:expr) => {
        $story .record_verse($msg)
    };
    ($story:expr, $fmt:expr, $($arg:tt)*) => {
        $story .record_verse(& format!($fmt, $($arg)*))
    };
}
/// Records a verse with structural evidence (data).
#[macro_export]
macro_rules! verse_with {
    ($story:expr, $msg:expr, $data:expr) => {
        $story .record_verse_with_data($msg, $data)
    };
}
/// Guard Clause macro.
/// Checks a condition; if false, concludes the story with failure and returns early.
#[macro_export]
macro_rules! ensure {
    ($story:expr, $condition:expr, $failure_reason:expr) => {
        if !$condition { let _ = $story .conclude_failure($failure_reason); return
        Err($crate::seal::SecurityViolation::ProcessingError($failure_reason
        .to_string()).into()); }
    };
}