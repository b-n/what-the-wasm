#include <stdio.h>
#include <stdlib.h>

const int ARR_LEN = 10;

int main() {
  int *array = malloc(ARR_LEN * sizeof(int));
  for (int i = 0; i < ARR_LEN; i++) {
    array[i] = i * 2;
  }

  for (int i = ARR_LEN - 1; i >= 0; i--) {
    printf("%d", array[i]);
  }

  free(array);
  return 0;
}

