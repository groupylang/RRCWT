// interface: c++ to c
#ifndef RCWT_ENV_H
#define RCWT_ENV_H

#include <cstdio>
#include <cstdint>
#include <cstdlib>
#include <vector>
#include <unordered_map>
#include <iostream>

typedef struct _env env;

typedef void(*procedure)(env*);

struct _env {
  uint8_t* text;
  const uint8_t* data;
  uint32_t* registers;
  std::vector<uint32_t> stack;
  std::vector<uint32_t> heap;
  uint32_t stack_pointer;
  uint32_t base_pointer;
  std::unordered_map<size_t, uint32_t> hot_spots;
  std::unordered_map<size_t, procedure> natives;
};

extern "C" {
  uint8_t virtual_execute(env*, uint32_t);
  // push into stack
  void push(env*, uint32_t);
  // pop out of stack
  uint32_t pop(env*);
  // breakpoint
  void breakpoint();
}

#define TEXT(n) e->text[n]
#define DATA(n) e->data[n]
#define REGISTERS(n) e->registers[n]
#define SP e->stack_pointer
#define BP e->base_pointer
#define TOP(n) e->stack[SP - n]
#define LOCAL(n) e->stack[BP + n]
#define ARG(n) e->stack[BP - n]
#define PROLOG push(e, BP); BP = SP
#define EPILOG SP = BP; BP = pop(e)

#endif