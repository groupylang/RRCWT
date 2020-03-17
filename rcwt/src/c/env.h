#ifndef RCWT_ENV_H
#define RCWT_ENV_H

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint8_t* text;
  uint8_t* data;
  uint32_t* registers;
  uint32_t* stack;
  uint32_t* heap;
  uint32_t stack_pointer;
  uint32_t base_pointer;
} env;

#endif