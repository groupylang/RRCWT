#ifndef RCWT_VM_H
#define RCWT_VM_H

#include <stdio.h>
#include <stdint.h>

typedef struct {
  uint8_t code;
  int8_t op0;
  int8_t op1;
  int8_t op2;
} instruction;

#endif