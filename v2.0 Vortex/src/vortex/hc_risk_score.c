
#include <stdio.h>
#include <stdlib.h>

int hc_compute_risk_score(const char *cbom_path, int *score_out) {
  if (!cbom_path || !score_out)
    return -1;
  // Very simple parser: count number of entries in the JSON array.
  FILE *f = fopen(cbom_path, "r");
  if (!f)
    return -1;
  fseek(f, 0, SEEK_END);
  long size = ftell(f);
  fseek(f, 0, SEEK_SET);
  char *buf = (char *)malloc(size + 1);
  if (!buf) {
    fclose(f);
    return -1;
  }
  fread(buf, 1, size, f);
  buf[size] = '\0';
  fclose(f);
  // Count occurrences of '"' to estimate number of symbols.
  int count = 0;
  for (char *p = buf; *p; ++p) {
    if (*p == '"')
      ++count;
  }
  // Each entry has two quotes, so entries = count/2.
  int entries = count / 2;
  // Simple heuristic: more entries => higher readiness.
  int score = entries * 5; // each entry 5 points.
  if (score > 100)
    score = 100;
  *score_out = score;
  free(buf);
  return 0;
}
