#ifndef RCWT_VM_H
#define RCWT_VM_H

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "env.h"

typedef struct {
  uint8_t code;
  int8_t op0;
  int8_t op1;
  int8_t op2;
} instruction;

void v_exec(uint8_t*, uint8_t*, uint32_t);

// uint8_t is_hot(instruction*);
typedef void(*procedure)(env*);
procedure jit(char*);

#endif