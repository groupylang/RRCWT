// interface: rust to c++
#ifndef RCWT_VM_H
#define RCWT_VM_H

#include "env.h"

typedef struct {
  uint8_t code;
  int8_t op0;
  int8_t op1;
  int8_t op2;
} instruction;

extern "C" {
  void print_int(uint32_t);
  void print_str(char*);
  uint8_t is_hot(uint32_t*, instruction*);
  typedef void(*procedure)(env*);
}

#endif