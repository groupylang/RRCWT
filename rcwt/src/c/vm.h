// interface: rust to c++
#ifndef RCWT_VM_H
#define RCWT_VM_H

#include <mutex>
#include <thread>
#include <unordered_map>

#include "env.h"

#define SYNC(lambda) std::mutex mtx; { std::lock_guard<std::mutex> lock(mtx); lambda(); }

typedef struct {
  uint8_t code;
  int8_t op0;
  int8_t op1;
  int8_t op2;
} instruction;

extern "C" {
  void print_int(uint32_t);
  void print_str(const char*);
  void print_float(float);
  uint8_t virtual_execute_wrapper(env*, uint32_t, uint32_t, uint32_t, uint32_t);
  typedef void(*procedure)(env*);
  env* env_new(uint8_t*, uint8_t*, uint32_t);
}
// create new vector and fill it with 0
inline std::vector<uint32_t> vec_new();
// count how many times vm calls the virtual function and check if it is hot
uint8_t is_hot(std::unordered_map<size_t, uint32_t>&, size_t);
// just-in-time assemble (dll/so) and load
void jit_asm(std::unordered_map<size_t, procedure>&, size_t, const char*);
void native_load(std::unordered_map<size_t, procedure>&, size_t, std::string);
void native_execute(std::unordered_map<size_t, procedure>&, size_t, env*);
template <typename ... Args>
inline std::string format(const char fmt[], Args ... args) {
  size_t len = std::snprintf(nullptr, 0, fmt, args ...);
  std::vector<char> buf(len + 1);
  std::snprintf(&buf[0], len + 1, fmt, args ...);
  return std::string(&buf[0], &buf[0] + len);
}
void debugger(env&&, uint32_t, uint32_t, uint32_t);

extern uint8_t debug_flag;
extern uint8_t alive_flag;

#endif