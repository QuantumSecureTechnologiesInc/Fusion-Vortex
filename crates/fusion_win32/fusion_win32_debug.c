// fusion_win32_debug.c — Implementation of Win32 Debug API helpers for Fusion
#include "fusion_win32_debug.h"
#include <stdio.h>
#include <psapi.h>

#pragma comment(lib, "psapi.lib")

// --- Process ---
BOOL fusion_debug_create_process(const wchar_t* cmd_line, DWORD* out_pid,
                                  HANDLE* out_proc, HANDLE* out_thread) {
    STARTUPINFOW si = { sizeof(si) };
    PROCESS_INFORMATION pi = { 0 };
    si.dwFlags = STARTF_USESHOWWINDOW;
    si.wShowWindow = SW_HIDE;

    // Use a mutable copy since CreateProcessW may modify lpCommandLine
    wchar_t cmd_copy[MAX_PATH * 2] = { 0 };
    wcsncpy_s(cmd_copy, _countof(cmd_copy), cmd_line, _TRUNCATE);

    BOOL ok = CreateProcessW(
        NULL,           // lpApplicationName
        cmd_copy,       // lpCommandLine (mutable)
        NULL, NULL,     // lpProcessAttributes, lpThreadAttributes
        FALSE,          // bInheritHandles
        DEBUG_ONLY_THIS_PROCESS | CREATE_NEW_CONSOLE,
        NULL, NULL,     // lpEnvironment, lpCurrentDirectory
        &si, &pi
    );

    if (ok) {
        *out_pid = pi.dwProcessId;
        *out_proc = pi.hProcess;
        *out_thread = pi.hThread;
        CloseHandle(pi.hThread);
    }
    return ok;
}

BOOL fusion_debug_terminate_process(HANDLE proc, DWORD exit_code) {
    return TerminateProcess(proc, exit_code);
}

// --- Debug Event Loop ---
BOOL fusion_debug_wait_for_event(DEBUG_EVENT* out_event, DWORD timeout_ms) {
    return WaitForDebugEvent(out_event, timeout_ms);
}

BOOL fusion_debug_continue(DWORD pid, DWORD tid, DWORD continue_status) {
    return ContinueDebugEvent(pid, tid, continue_status);
}

// --- Thread Context ---
BOOL fusion_debug_get_thread_context(HANDLE thread, CONTEXT* ctx) {
    ctx->ContextFlags = CONTEXT_FULL | CONTEXT_DEBUG_REGISTERS;
    return GetThreadContext(thread, ctx);
}

BOOL fusion_debug_set_thread_context(HANDLE thread, const CONTEXT* ctx) {
    return SetThreadContext(thread, ctx);
}

// --- Memory ---
BOOL fusion_debug_read_memory(HANDLE proc, LPCVOID addr, void* buf,
                               SIZE_T size, SIZE_T* out_read) {
    return ReadProcessMemory(proc, addr, buf, size, out_read);
}

BOOL fusion_debug_write_memory(HANDLE proc, LPVOID addr, const void* buf,
                                SIZE_T size, SIZE_T* out_written) {
    return WriteProcessMemory(proc, addr, buf, size, out_written);
}

// --- Breakpoints ---
BOOL fusion_debug_write_byte(HANDLE proc, LPVOID addr, BYTE byte_val) {
    SIZE_T written;
    return WriteProcessMemory(proc, addr, &byte_val, 1, &written);
}

BOOL fusion_debug_read_byte(HANDLE proc, LPCVOID addr, BYTE* out_byte) {
    SIZE_T read;
    return ReadProcessMemory(proc, addr, out_byte, 1, &read);
}

// --- Thread Handle ---
HANDLE fusion_debug_open_thread(DWORD tid) {
    return OpenThread(THREAD_ALL_ACCESS, FALSE, tid);
}

BOOL fusion_debug_close_handle(HANDLE handle) {
    return CloseHandle(handle);
}

// --- Symbol/Module Information ---
BOOL fusion_debug_get_module_base(HANDLE proc, const wchar_t* module_name,
                                   DWORD64* out_base) {
    HMODULE modules[1024];
    DWORD needed;
    if (!EnumProcessModules(proc, modules, sizeof(modules), &needed)) {
        return FALSE;
    }
    DWORD count = needed / sizeof(HMODULE);
    for (DWORD i = 0; i < count && i < 1024; i++) {
        wchar_t name[MAX_PATH];
        if (GetModuleBaseNameW(proc, modules[i], name, _countof(name))) {
            if (_wcsicmp(name, module_name) == 0) {
                MODULEINFO mi;
                if (GetModuleInformation(proc, modules[i], &mi, sizeof(mi))) {
                    *out_base = (DWORD64)mi.lpBaseOfDll;
                    return TRUE;
                }
            }
        }
    }
    return FALSE;
}

// Simple export dump: writes semicolon-separated "addr=name" pairs into buffer
BOOL fusion_debug_get_exports(HANDLE proc, DWORD64 module_base,
                               char* buffer, DWORD buffer_size, DWORD* out_count) {
    // For simplicity, use IMAGE_DOS_HEADER/IMAGE_NT_HEADERS via ReadProcessMemory
    IMAGE_DOS_HEADER dos;
    if (!ReadProcessMemory(proc, (LPCVOID)module_base, &dos, sizeof(dos), NULL))
        return FALSE;
    if (dos.e_magic != IMAGE_DOS_SIGNATURE) return FALSE;

    IMAGE_NT_HEADERS64 nt;
    if (!ReadProcessMemory(proc, (LPCVOID)(module_base + dos.e_lfanew), &nt, sizeof(nt), NULL))
        return FALSE;
    if (nt.Signature != IMAGE_NT_SIGNATURE) return FALSE;

    DWORD export_rva = nt.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress;
    DWORD export_size = nt.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].Size;
    if (export_rva == 0 || export_size == 0) return FALSE;

    IMAGE_EXPORT_DIRECTORY exp;
    if (!ReadProcessMemory(proc, (LPCVOID)(module_base + export_rva), &exp, sizeof(exp), NULL))
        return FALSE;

    DWORD* names_rva = (DWORD*)malloc(exp.NumberOfNames * sizeof(DWORD));
    DWORD* funcs_rva = (DWORD*)malloc(exp.NumberOfFunctions * sizeof(DWORD));
    WORD*  ords_rva  = (WORD*)malloc(exp.NumberOfNames * sizeof(WORD));

    if (!ReadProcessMemory(proc, (LPCVOID)(module_base + exp.AddressOfNames),
                           names_rva, exp.NumberOfNames * sizeof(DWORD), NULL))
        goto cleanup;
    if (!ReadProcessMemory(proc, (LPCVOID)(module_base + exp.AddressOfFunctions),
                           funcs_rva, exp.NumberOfFunctions * sizeof(DWORD), NULL))
        goto cleanup;
    if (!ReadProcessMemory(proc, (LPCVOID)(module_base + exp.AddressOfNameOrdinals),
                           ords_rva, exp.NumberOfNames * sizeof(WORD), NULL))
        goto cleanup;

    DWORD total = 0;
    for (DWORD i = 0; i < exp.NumberOfNames && buffer_size > 128; i++) {
        char name_buf[256];
        if (ReadProcessMemory(proc, (LPCVOID)(module_base + names_rva[i]),
                              name_buf, sizeof(name_buf) - 1, NULL)) {
            name_buf[255] = 0;
            DWORD64 addr = module_base + funcs_rva[ords_rva[i]];
            int n = snprintf(buffer, buffer_size, "%llu=%s;", addr, name_buf);
            if (n > 0 && (DWORD)n < buffer_size) {
                buffer += n;
                buffer_size -= n;
                total++;
            }
        }
    }
    *out_count = total;

cleanup:
    free(names_rva);
    free(funcs_rva);
    free(ords_rva);
    return total > 0;
}

// --- Utility ---
DWORD fusion_debug_get_last_error(void) {
    return GetLastError();
}