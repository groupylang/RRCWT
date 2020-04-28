// interface: c++ to c
#ifndef RCWT_ENV_H
#define RCWT_ENV_H

#include <cstdio>
#include <cstdint>
#include <cstdlib>
#include <vector>

typedef struct {
  uint8_t* text;
  uint8_t* data;
  uint32_t* registers;
  std::vector<uint32_t> stack;
  std::vector<uint32_t> heap;
  uint32_t stack_pointer;
  uint32_t base_pointer;
} env;

extern "C" {
  uint8_t virtual_execute(uint32_t*, env*, uint32_t);
  // push into stack
  void push(env*, uint32_t);
  // pop out of stack
  uint32_t pop(env*);
}

#endif