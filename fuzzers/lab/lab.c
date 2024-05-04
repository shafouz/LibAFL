#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

__AFL_FUZZ_INIT();

int main(int argc, char **argv) {
  FILE *file = stdin;
  if (argc > 1) { file = fopen(argv[1], "rb"); }

  #ifdef __AFL_HAVE_MANUAL_CONTROL
    __AFL_INIT();
  #endif

  unsigned char *buf = __AFL_FUZZ_TESTCASE_BUF;
  int len = __AFL_FUZZ_TESTCASE_LEN;
  if (len < 8) return 0;

  if (buf[0] == 'c') {
    if (buf[1] == 'o') {
      if (buf[2] == 'o') {
        if (buf[3] == 'l') {}
      }
    }
  }

  return 0;
}
