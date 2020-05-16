// RCWT standard library to be compiled into dll/so and loaded by vm
#ifndef RCWT_RCWTLIB_H
#define RCWT_RCWTLIB_H

#include "../../rcwt/src/c/env.h"

extern "C" {
  // hello
  void hello(env*);
  // io
  void print_int(env*);
  void print_intln(env*);
  void print_str(env*);
  void print_strln(env*);
  void print_float(env*);
  void print_floatln(env*);
  // hash
  void hash_int(env*);
  void hash_float(env*);
  void hash_str(env*);
}

#endif