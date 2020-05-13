// RCWT standard library to be compiled into dll/so and loaded by vm
#ifndef RCWT_RCWTLIB_H
#define RCWT_RCWTLIB_H

#include "../../rcwt/src/c/env.h"

extern "C" {
  // hello
  void hello(env*);
  // io
  void print_int(env*);
  void print_str(env*);
}

#endif