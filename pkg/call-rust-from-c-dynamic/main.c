#include <stdio.h>
#include <string.h>

void greet(const char *);

#define MAX_NAME_LEN 1000

int main(int argc, char *argv[]) {
  char name[MAX_NAME_LEN];

  if (fgets(name, sizeof(name), stdin)) {
    size_t len = strlen(name);
    if (len > 0 && name[len - 1] == '\n') {
      name[len - 1] = '\0';
    }
  }

  greet(name);
  return 0;
}
