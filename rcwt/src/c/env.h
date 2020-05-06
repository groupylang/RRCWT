// interface: c++ to c
#ifndef RCWT_ENV_H
#define RCWT_ENV_H

#include <cstdio>
#include <cstdint>
#include <cstdlib>
#include <vector>
#include <unordered_map>

typedef struct {
  uint8_t* text;
  const uint8_t* data;
  uint32_t* registers;
  std::vector<uint32_t> stack;
  std::vector<uint32_t> heap;
  uint32_t stack_pointer;
  uint32_t base_pointer;
} cenv;

typedef void(*procedure)(cenv*);

typedef struct {
  cenv e;
  std::unordered_map<size_t, uint32_t> hot_spots;
  std::unordered_map<size_t, procedure> procs;
} env;

extern "C" {
  uint8_t virtual_execute(env*, uint32_t);
  // push into stack
  void push(cenv*, uint32_t);
  // pop out of stack
  uint32_t pop(cenv*);
}

#endif