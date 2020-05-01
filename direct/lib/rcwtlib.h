// RCWT standard library to be compiled into dll/so and loaded by vm
#ifndef RCWT_RCWTLIB_H
#define RCWT_RCWTLIB_H

#include <cstdint>

// io
extern "C" {
  void print_int(uint32_t);
  void print_str(char*);
}

#endif