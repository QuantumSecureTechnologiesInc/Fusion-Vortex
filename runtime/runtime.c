#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>
#include <stdint.h>
#include <pthread.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>

void panic(const char* msg) {
    if (msg) {
        fprintf(stderr, "Runtime Panic: %s\n", msg);
    } else {
        fprintf(stderr, "Runtime Panic\n");
    }
    abort();
}

// Returns a heap-allocated line (caller can free). Strips trailing newline.
char* fusion_read_line(void) {
    char* line = NULL;
    size_t cap = 0;
    ssize_t n = getline(&line, &cap, stdin);
    if (n <= 0) {
        if (line) {
            line[0] = '\0';
            return line;
        }
        char* empty = (char*)malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    if (n > 0 && line[n - 1] == '\n') {
        line[n - 1] = '\0';
    }
    return line;
}

int string_starts_with(const char* s, const char* prefix) {
    if (!s || !prefix) return 0;
    size_t plen = strlen(prefix);
    return strncmp(s, prefix, plen) == 0;
}

static char* read_file_all(const char* path) {
    FILE* f = fopen(path, "rb");
    if (!f) return NULL;
    if (fseek(f, 0, SEEK_END) != 0) { fclose(f); return NULL; }
    long size = ftell(f);
    if (size < 0) { fclose(f); return NULL; }
    if (fseek(f, 0, SEEK_SET) != 0) { fclose(f); return NULL; }
    char* buf = (char*)malloc((size_t)size + 1);
    if (!buf) { fclose(f); return NULL; }
    size_t n = fread(buf, 1, (size_t)size, f);
    fclose(f);
    buf[n] = '\0';
    return buf;
}

char* fusion_fs_read_to_string(const char* path) {
    return read_file_all(path);
}

int fusion_fs_write_str(const char* path, const char* contents) {
    FILE* f = fopen(path, "wb");
    if (!f) return 0;
    if (contents) {
        fwrite(contents, 1, strlen(contents), f);
    }
    fclose(f);
    return 1;
}

int fusion_fs_append_str(const char* path, const char* contents) {
    FILE* f = fopen(path, "ab");
    if (!f) return 0;
    if (contents) {
        fwrite(contents, 1, strlen(contents), f);
    }
    fclose(f);
    return 1;
}

int fusion_fs_exists(const char* path) {
    struct stat st;
    return stat(path, &st) == 0;
}

int fusion_fs_create_dir(const char* path) {
    if (!path) return 0;
    return mkdir(path, 0755) == 0 || errno == EEXIST;
}

int fusion_fs_remove_file(const char* path) {
    if (!path) return 0;
    return remove(path) == 0;
}

char* fusion_env_get(const char* key) {
    if (!key) return NULL;
    const char* val = getenv(key);
    if (!val) {
        char* empty = (char*)malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    size_t len = strlen(val);
    char* out = (char*)malloc(len + 1);
    if (!out) return NULL;
    memcpy(out, val, len + 1);
    return out;
}

char* fusion_ipc_query(const char* key) {
    (void)key;
    char* empty = (char*)malloc(1);
    if (empty) empty[0] = '\0';
    return empty;
}

char* fusion_prompt(const char* prompt) {
    if (prompt) {
        fprintf(stdout, "%s", prompt);
        fflush(stdout);
    }
    return fusion_read_line();
}

int fusion_argc(void) {
    FILE* f = fopen("/proc/self/cmdline", "rb");
    if (!f) return 0;
    int count = 0;
    int ch;
    int last_was_null = 1;
    while ((ch = fgetc(f)) != EOF) {
        if (ch == '\0') {
            if (!last_was_null) {
                count++;
                last_was_null = 1;
            }
        } else {
            last_was_null = 0;
        }
    }
    fclose(f);
    return count;
}

char* fusion_argv(int idx) {
    FILE* f = fopen("/proc/self/cmdline", "rb");
    if (!f) return NULL;
    int current = 0;
    size_t cap = 256;
    char* buf = (char*)malloc(cap);
    size_t len = 0;
    int ch;
    while ((ch = fgetc(f)) != EOF) {
        if (ch == '\0') {
            if (current == idx) {
                buf[len] = '\0';
                fclose(f);
                return buf;
            }
            current++;
            len = 0;
            continue;
        }
        if (current == idx) {
            if (len + 1 >= cap) {
                cap *= 2;
                buf = (char*)realloc(buf, cap);
                if (!buf) { fclose(f); return NULL; }
            }
            buf[len++] = (char)ch;
        }
    }
    fclose(f);
    if (buf) free(buf);
    return NULL;
}

int fusion_time_now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int)((ts.tv_sec * 1000) + (ts.tv_nsec / 1000000));
}

void fusion_sleep_ms(int ms) {
    if (ms <= 0) return;
    struct timespec ts;
    ts.tv_sec = ms / 1000;
    ts.tv_nsec = (ms % 1000) * 1000000;
    nanosleep(&ts, NULL);
}

static uint64_t g_rand_state = 88172645463325252ULL;
void fusion_rand_seed(int seed) {
    if (seed == 0) seed = 1;
    g_rand_state = ((uint64_t)seed << 32) ^ 0x9e3779b97f4a7c15ULL;
}
int fusion_rand_next(void) {
    uint64_t x = g_rand_state;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    g_rand_state = x;
    uint64_t r = x * 2685821657736338717ULL;
    return (int)(r & 0x7fffffff);
}

int fusion_hash32(const char* s) {
    if (!s) return 0;
    uint32_t hash = 5381;
    int c;
    while ((c = *s++)) {
        hash = ((hash << 5) + hash) + (uint32_t)c;
    }
    return (int)hash;
}

int fusion_hmac32(const char* key, const char* msg) {
    int h1 = fusion_hash32(key);
    int h2 = fusion_hash32(msg);
    return h1 ^ h2;
}

char* fusion_fmt_int(int v) {
    char buf[64];
    snprintf(buf, sizeof(buf), "%d", v);
    return strdup(buf);
}

char* fusion_fmt_pair(const char* k, const char* v) {
    if (!k) k = "";
    if (!v) v = "";
    size_t len = strlen(k) + strlen(v) + 2;
    char* out = (char*)malloc(len + 1);
    if (!out) return NULL;
    snprintf(out, len + 1, "%s=%s", k, v);
    return out;
}

char* fusion_json_escape(const char* s) {
    if (!s) return strdup("");
    size_t len = strlen(s);
    char* out = (char*)malloc(len * 2 + 1);
    if (!out) return NULL;
    size_t j = 0;
    for (size_t i = 0; i < len; i++) {
        char c = s[i];
        if (c == '\"' || c == '\\') {
            out[j++] = '\\';
        }
        out[j++] = c;
    }
    out[j] = '\0';
    return out;
}

char* fusion_json_kv_string(const char* k, const char* v) {
    char* ek = fusion_json_escape(k);
    char* ev = fusion_json_escape(v);
    size_t len = strlen(ek) + strlen(ev) + 6;
    char* out = (char*)malloc(len + 1);
    if (!out) return NULL;
    snprintf(out, len + 1, "\"%s\":\"%s\"", ek, ev);
    free(ek);
    free(ev);
    return out;
}

char* fusion_json_kv_int(const char* k, int v) {
    char* ek = fusion_json_escape(k);
    char buf[64];
    snprintf(buf, sizeof(buf), "%d", v);
    size_t len = strlen(ek) + strlen(buf) + 4;
    char* out = (char*)malloc(len + 1);
    if (!out) return NULL;
    snprintf(out, len + 1, "\"%s\":%s", ek, buf);
    free(ek);
    return out;
}

int fusion_tcp_connect(const char* host, int port) {
    char port_str[16];
    snprintf(port_str, sizeof(port_str), "%d", port);
    struct addrinfo hints;
    struct addrinfo* res = NULL;
    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    if (getaddrinfo(host, port_str, &hints, &res) != 0) return -1;
    int sock = -1;
    for (struct addrinfo* p = res; p; p = p->ai_next) {
        sock = socket(p->ai_family, p->ai_socktype, p->ai_protocol);
        if (sock < 0) continue;
        if (connect(sock, p->ai_addr, p->ai_addrlen) == 0) break;
        close(sock);
        sock = -1;
    }
    freeaddrinfo(res);
    return sock;
}

int fusion_tcp_send_str(int fd, const char* data) {
    if (fd < 0 || !data) return -1;
    return (int)send(fd, data, strlen(data), 0);
}

char* fusion_tcp_recv_str(int fd, int max_bytes) {
    if (fd < 0 || max_bytes <= 0) return strdup("");
    char* buf = (char*)malloc((size_t)max_bytes + 1);
    if (!buf) return NULL;
    int n = (int)recv(fd, buf, (size_t)max_bytes, 0);
    if (n < 0) n = 0;
    buf[n] = '\0';
    return buf;
}

void fusion_tcp_close(int fd) {
    if (fd >= 0) close(fd);
}

int fusion_udp_send_str(const char* host, int port, const char* data) {
    if (!host || !data) return -1;
    int sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0) return -1;
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons((uint16_t)port);
    if (inet_pton(AF_INET, host, &addr.sin_addr) <= 0) {
        close(sock);
        return -1;
    }
    int n = (int)sendto(sock, data, strlen(data), 0, (struct sockaddr*)&addr, sizeof(addr));
    close(sock);
    return n;
}

char* fusion_udp_recv_str(int port, int max_bytes) {
    if (max_bytes <= 0) return strdup("");
    int sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0) return strdup("");
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons((uint16_t)port);
    if (bind(sock, (struct sockaddr*)&addr, sizeof(addr)) != 0) {
        close(sock);
        return strdup("");
    }
    char* buf = (char*)malloc((size_t)max_bytes + 1);
    if (!buf) { close(sock); return strdup(""); }
    int n = (int)recvfrom(sock, buf, (size_t)max_bytes, 0, NULL, NULL);
    if (n < 0) n = 0;
    buf[n] = '\0';
    close(sock);
    return buf;
}

int fusion_mutex_new(void) {
    pthread_mutex_t* m = (pthread_mutex_t*)malloc(sizeof(pthread_mutex_t));
    if (!m) return 0;
    pthread_mutex_init(m, NULL);
    return (int)(intptr_t)m;
}

void fusion_mutex_lock(int handle) {
    if (handle == 0) return;
    pthread_mutex_t* m = (pthread_mutex_t*)(intptr_t)handle;
    pthread_mutex_lock(m);
}

void fusion_mutex_unlock(int handle) {
    if (handle == 0) return;
    pthread_mutex_t* m = (pthread_mutex_t*)(intptr_t)handle;
    pthread_mutex_unlock(m);
}

void fusion_mutex_free(int handle) {
    if (handle == 0) return;
    pthread_mutex_t* m = (pthread_mutex_t*)(intptr_t)handle;
    pthread_mutex_destroy(m);
    free(m);
}
