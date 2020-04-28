// interface: rust to c++
#ifndef RCWT_VM_H
#define RCWT_VM_H

#include "env.h"

#include <utility>
#include <algorithm>
#include <string>
#include <iostream>
#include <fstream>
#include <unordered_map>

typedef struct {
  uint8_t code;
  int8_t op0;
  int8_t op1;
  int8_t op2;
} instruction;

extern "C" {
  void print_int(uint32_t);
  void print_str(char*);
  typedef void(*procedure)(env*);
}
inline std::vector<uint32_t> vec_new();
inline uint8_t is_hot(std::unordered_map<size_t, uint32_t>&, size_t);
inline void jit_asm(std::unordered_map<size_t, procedure>&, size_t, const char*);
inline void native_execute(std::unordered_map<size_t, procedure>&, size_t, env*);
void bp(env*);
void debugger(uint32_t, uint32_t, uint32_t);

#endif