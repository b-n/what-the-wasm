long long sum_reduce(int to) {
  long long total = 0;
  for (int i = 0; i <= to; i++) {
    total += i;
  }
  return total;
}
