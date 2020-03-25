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

uint8_t v_exec(uint32_t*, uint8_t*, uint8_t*, uint32_t);

void print_int(uint32_t);
void print_str(char*);
uint8_t is_hot(uint32_t*, instruction*);
typedef void(*procedure)(env*);
void jit(uint32_t*, uint32_t*, char*);

#endif