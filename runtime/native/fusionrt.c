#include "fusionrt.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdarg.h>

#ifdef _WIN32
#include <direct.h>
#include <windows.h>

#define mkdir(path) _mkdir(path)
#define PATH_SEP "\\"
#else
#include <sys/stat.h>
#include <sys/time.h>
#include <unistd.h>

#define PATH_SEP "/"
#endif

// ========================================
// String Pool for Interned Returns
// ========================================
#define STRING_POOL_SIZE (16 * 1024 * 1024) // 16MB
static char string_pool[STRING_POOL_SIZE];
static size_t pool_offset = 0;

static const char *pool_intern(const char *s) {
  if (!s)
    return pool_intern("");

  size_t len = strlen(s) + 1;
  if (pool_offset + len > STRING_POOL_SIZE) {
    fprintf(stderr, "FATAL: Fusion string pool exhausted\n");
    exit(1);
  }

  char *dest = string_pool + pool_offset;
  memcpy(dest, s, len);
  pool_offset += len;
  return dest;
}

static const char *pool_sprintf(const char *fmt, ...) {
  char buf[4096];
  va_list args;
  va_start(args, fmt);
  vsnprintf(buf, sizeof(buf), fmt, args);
  va_end(args);
  return pool_intern(buf);
}

// ========================================
// Global State
// ========================================
static int g_argc = 0;
static char **g_argv = NULL;
static uint64_t g_rand_state = 12345;

void fusion_rt_init(void) {
  // Called before any runtime functions
  pool_offset = 0;
  g_rand_state = (uint64_t)time(NULL);
}

void fusion_rt_shutdown(void) {
  // Cleanup (string pool is static, nothing to free)
}

// ========================================
// I/O Functions
// ========================================
const char *fusion_read_line(void) {
  char buf[8192];
  if (fgets(buf, sizeof(buf), stdin)) {
    // Remove trailing newline
    size_t len = strlen(buf);
    if (len > 0 && buf[len - 1] == '\n')
      buf[len - 1] = '\0';
    return pool_intern(buf);
  }
  return pool_intern("");
}

// ========================================
// File System
// ========================================
const char *fusion_fs_read_to_string(const char *path) {
  FILE *f = fopen(path, "rb");
  if (!f)
    return pool_intern("");

  fseek(f, 0, SEEK_END);
  long size = ftell(f);
  fseek(f, 0, SEEK_SET);

  if (size < 0 || size > STRING_POOL_SIZE - pool_offset - 1) {
    fclose(f);
    return pool_intern("");
  }

  char *dest = string_pool + pool_offset;
  size_t read = fread(dest, 1, size, f);
  fclose(f);

  dest[read] = '\0';
  pool_offset += read + 1;
  return dest;
}

bool fusion_fs_write_str(const char *path, const char *contents) {
  FILE *f = fopen(path, "wb");
  if (!f)
    return false;

  size_t len = strlen(contents);
  size_t written = fwrite(contents, 1, len, f);
  fclose(f);
  return written == len;
}

bool fusion_fs_append_str(const char *path, const char *contents) {
  FILE *f = fopen(path, "ab");
  if (!f)
    return false;

  size_t len = strlen(contents);
  size_t written = fwrite(contents, 1, len, f);
  fclose(f);
  return written == len;
}

bool fusion_fs_exists(const char *path) {
  FILE *f = fopen(path, "rb");
  if (f) {
    fclose(f);
    return true;
  }
  return false;
}

bool fusion_fs_create_dir(const char *path) { return mkdir(path, 0755) == 0; }

bool fusion_fs_remove_file(const char *path) { return remove(path) == 0; }

// ========================================
// Path Manipulation
// ========================================
const char *fusion_path_join(const char *a, const char *b) {
  return pool_sprintf("%s%s%s", a, PATH_SEP, b);
}

const char *fusion_path_basename(const char *p) {
  const char *last_sep = strrchr(p, PATH_SEP[0]);
  if (last_sep)
    return pool_intern(last_sep + 1);
  return pool_intern(p);
}

const char *fusion_path_dirname(const char *p) {
  const char *last_sep = strrchr(p, PATH_SEP[0]);
  if (!last_sep)
    return pool_intern(".");

  size_t len = last_sep - p;
  if (len == 0)
    return pool_intern(PATH_SEP);

  char buf[4096];
  if (len >= sizeof(buf))
    len = sizeof(buf) - 1;
  memcpy(buf, p, len);
  buf[len] = '\0';
  return pool_intern(buf);
}

// ========================================
// Environment
// ========================================
const char *fusion_env_get(const char *key) {
  char *val = getenv(key);
  return val ? pool_intern(val) : pool_intern("");
}

int64_t fusion_argc(void) { return g_argc; }

const char *fusion_argv(int64_t idx) {
  if (idx < 0 || idx >= g_argc)
    return pool_intern("");
  return pool_intern(g_argv[idx]);
}

// ========================================
// Time
// ========================================
int64_t fusion_time_now_ms(void) {
#ifdef _WIN32
  return (int64_t)GetTickCount64();
#else
  struct timeval tv;
  gettimeofday(&tv, NULL);
  return (int64_t)tv.tv_sec * 1000 + tv.tv_usec / 1000;
#endif
}

void fusion_sleep_ms(int64_t ms) {
#ifdef _WIN32
  Sleep((DWORD)ms);
#else
  usleep(ms * 1000);
#endif
}

// ========================================
// Random (simple LCG)
// ========================================
void fusion_rand_seed(int64_t seed) { g_rand_state = (uint64_t)seed; }

int64_t fusion_rand_next(void) {
  g_rand_state = g_rand_state * 6364136223846793005ULL + 1442695040888963407ULL;
  return (int64_t)(g_rand_state >> 32);
}

// ========================================
// Crypto/Hash (simple DJB2)
// ========================================
int64_t fusion_hash32(const char *s) {
  uint64_t hash = 5381;
  int c;
  while ((c = *s++)) {
    hash = ((hash << 5) + hash) + c;
  }
  return (int64_t)(hash & 0xFFFFFFFF);
}

int64_t fusion_hmac32(const char *key, const char *msg) {
  // Simplified: just hash(key + msg)
  char buf[8192];
  snprintf(buf, sizeof(buf), "%s%s", key, msg);
  return fusion_hash32(buf);
}

// ========================================
// JSON/Formatting
// ========================================
const char *fusion_fmt_int(int64_t v) {
  return pool_sprintf("%lld", (long long)v);
}

const char *fusion_fmt_pair(const char *k, const char *v) {
  return pool_sprintf("%s=%s", k, v);
}

const char *fusion_json_escape(const char *s) {
  char buf[8192];
  char *dst = buf;
  const char *src = s;

  while (*src && (dst - buf) < sizeof(buf) - 10) {
    switch (*src) {
    case '"':
      *dst++ = '\\';
      *dst++ = '"';
      break;
    case '\\':
      *dst++ = '\\';
      *dst++ = '\\';
      break;
    case '\n':
      *dst++ = '\\';
      *dst++ = 'n';
      break;
    case '\r':
      *dst++ = '\\';
      *dst++ = 'r';
      break;
    case '\t':
      *dst++ = '\\';
      *dst++ = 't';
      break;
    default:
      *dst++ = *src;
      break;
    }
    src++;
  }
  *dst = '\0';
  return pool_intern(buf);
}

const char *fusion_json_kv_string(const char *k, const char *v) {
  return pool_sprintf("\"%s\":\"%s\"", fusion_json_escape(k),
                      fusion_json_escape(v));
}

const char *fusion_json_kv_int(const char *k, int64_t v) {
  return pool_sprintf("\"%s\":%lld", fusion_json_escape(k), (long long)v);
}

// ========================================
// Networking (Stubs - TODO: implement)
// ========================================
int64_t fusion_tcp_connect(const char *host, int64_t port) {
  fprintf(stderr, "STUB: fusion_tcp_connect(%s, %lld)\n", host,
          (long long)port);
  return -1;
}

int64_t fusion_tcp_send_str(int64_t fd, const char *data) {
  fprintf(stderr, "STUB: fusion_tcp_send_str(%lld, ...)\n", (long long)fd);
  return -1;
}

const char *fusion_tcp_recv_str(int64_t fd, int64_t max_bytes) {
  fprintf(stderr, "STUB: fusion_tcp_recv_str(%lld, %lld)\n", (long long)fd,
          (long long)max_bytes);
  return pool_intern("");
}

void fusion_tcp_close(int64_t fd) {
  fprintf(stderr, "STUB: fusion_tcp_close(%lld)\n", (long long)fd);
}

int64_t fusion_udp_send_str(const char *host, int64_t port, const char *data) {
  fprintf(stderr, "STUB: fusion_udp_send_str(%s, %lld, ...)\n", host,
          (long long)port);
  return -1;
}

const char *fusion_udp_recv_str(int64_t port, int64_t max_bytes) {
  fprintf(stderr, "STUB: fusion_udp_recv_str(%lld, %lld)\n", (long long)port,
          (long long)max_bytes);
  return pool_intern("");
}

// ========================================
// Synchronization (Stubs - TODO: implement)
// ========================================
int64_t fusion_mutex_new(void) {
  return 0; // Stub
}

void fusion_mutex_lock(int64_t handle) {
  // Stub
}

void fusion_mutex_unlock(int64_t handle) {
  // Stub
}

void fusion_mutex_free(int64_t handle) {
  // Stub
}

// ========================================
// Runtime Entry Point Shim
// ========================================
// This allows C runtime to capture argc/argv before Fusion main
void fusion_rt_set_args(int argc, char **argv) {
  g_argc = argc;
  g_argv = argv;
}
