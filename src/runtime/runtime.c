#include "stdlib.h"
#include <inttypes.h>
#include <stdio.h>

extern uint64_t lisp_entry(void *heap);

#define num_mask 0b11
#define num_tag 0b00
#define num_shift 2

#define bool_mask 0b1111111
#define bool_tag 0b0011111
#define bool_shift 7

#define heap_mask 0b111
#define pair_tag 0b010

#define nil_mask 0b11111111
#define nil_tag 0b11111111

#define vector_tag 0b101
#define vector_mask 0b111

uint64_t print_value(uint64_t value) {
  if ((value & num_mask) == num_tag) {
    int64_t ivalue = (int64_t)value;
    printf("%" PRIi64, ivalue >> num_shift);
  } else if ((value & bool_mask) == bool_tag) {
    if (value >> bool_shift) {
      printf("true");
    } else {
      printf("false");
    }
  } else if ((value & heap_mask) == pair_tag) {
    uint64_t v1 = *(uint64_t *)(value - pair_tag);
    uint64_t v2 = *(uint64_t *)(value - pair_tag + 8);
    printf("(pair ");
    print_value(v1);
    printf(" ");
    print_value(v2);
    printf(")");
  } else if ((value & nil_mask) == nil_tag) {
    printf("()");
  } else if ((value & vector_mask) == vector_tag) {
    uint64_t *vector = (uint64_t *)(value - vector_tag);
    uint64_t length = *vector;
    vector++;
    printf("[");
    for (uint64_t i = 0; i < length; i++) {
      print_value(vector[i]);
      if (i != length - 1) {
        printf(" ");
      }
    }
    printf("]");
  } else {
    printf("BAD VALUE: %" PRIu64, value);
  }
  return value;
}

void lisp_error(char *exp) {
  printf("Stuck[%s]", exp);
  exit(1);
}

int main(int argc, char **argv) {
  void *heap = malloc(4096);
  print_value(lisp_entry(heap));
  printf("\n");
  return 0;
}
