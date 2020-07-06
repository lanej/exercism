#include "armstrong_numbers.h"

int is_armstrong_number(int candidate) {
  int sum = 0;
  int remainder = candidate;

  unsigned digits[] = { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };

  for (int i = 0; remainder > 0; i++) {
    digits[i] = remainder % 10;
    remainder /= 10;
  }


  return sum;
}
