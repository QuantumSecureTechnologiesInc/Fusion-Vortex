#ifndef HYPERCYCLE_TEMPORAL_H
#define HYPERCYCLE_TEMPORAL_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <time.h>


typedef struct {
  uint64_t violations_prevented;
  struct timespec last_timestamp;
} temporal_protection_t;

int temporal_protection_init(temporal_protection_t *guard);
bool temporal_protection_check_violation(temporal_protection_t *guard);

#endif // HYPERCYCLE_TEMPORAL_H
