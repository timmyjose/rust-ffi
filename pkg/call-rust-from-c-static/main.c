#include <stdio.h>

int add(int, int);
int sub(int, int);
int mul(int, int);
int div(int, int);

int main(int argc, char *argv[]) {
  int a, b;
  scanf("%d%d", &a, &b);

  printf("%d + %d = %d\n", a, b, add(a, b));
  printf("%d - %d = %d\n", a, b, sub(a, b));
  printf("%d * %d = %d\n", a, b, mul(a, b));
  printf("%d / %d = %d\n", a, b, div(a, b));

  return 0;
}
