#include <stdio.h>
#include <stdlib.h>

const int arr_size = 10;

int main() {
  int *array = malloc(arr_size * sizeof(int));
  for (int i = 0; i < arr_size; i++) {
    array[i] = i * 2;
  }

  for (int i = arr_size - 1; i >= 0; i--) {
    printf("%d", array[i]);
  }

  free(array);
}

