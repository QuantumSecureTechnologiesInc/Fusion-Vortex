// fusion_win32_debug.h — C helper for Win32 Debug API access from Fusion
// Provides thin wrappers around the Windows Debug API so Fusion
// extern fn calls can drive the debugger.

#ifndef FUSION_WIN32_DEBUG_H
#define FUSION_WIN32_DEBUG_H

#include <windows.h>

#ifdef __cplusplus
extern "C" {
#endif

// --- Process ---
BOOL  fusion_debug_create_process(const wchar_t* cmd_line, DWORD* out_pid, HANDLE* out_proc, HANDLE* out_thread);
BOOL  fusion_debug_terminate_process(HANDLE proc, DWORD exit_code);

// --- Debug Event Loop ---
BOOL  fusion_debug_wait_for_event(DEBUG_EVENT* out_event, DWORD timeout_ms);
BOOL  fusion_debug_continue(DWORD pid, DWORD tid, DWORD continue_status);

// --- Thread Context ---
BOOL  fusion_debug_get_thread_context(HANDLE thread, CONTEXT* ctx);
BOOL  fusion_debug_set_thread_context(HANDLE thread, const CONTEXT* ctx);

// --- Memory ---
BOOL  fusion_debug_read_memory(HANDLE proc, LPCVOID addr, void* buf, SIZE_T size, SIZE_T* out_read);
BOOL  fusion_debug_write_memory(HANDLE proc, LPVOID addr, const void* buf, SIZE_T size, SIZE_T* out_written);

// --- Breakpoints via INT3 ---
// Write single byte to target address
BOOL  fusion_debug_write_byte(HANDLE proc, LPVOID addr, BYTE byte_val);
BOOL  fusion_debug_read_byte(HANDLE proc, LPCVOID addr, BYTE* out_byte);

// --- Symbol/Module Information ---
BOOL  fusion_debug_get_module_base(HANDLE proc, const wchar_t* module_name, DWORD64* out_base);
BOOL  fusion_debug_get_exports(HANDLE proc, DWORD64 module_base,
                                char* buffer, DWORD buffer_size, DWORD* out_count);

// --- Thread/Process Handle ---
HANDLE fusion_debug_open_thread(DWORD tid);
BOOL   fusion_debug_close_handle(HANDLE handle);

// --- Utility ---
DWORD fusion_debug_get_last_error(void);

#ifdef __cplusplus
}
#endif

#endif // FUSION_WIN32_DEBUG_H