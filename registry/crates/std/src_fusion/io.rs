// Fusion stdlib replacement: io (stub)
extern fn puts(s: string) -> int;
pub fn print_line(s: string) -> void { puts(s); }
