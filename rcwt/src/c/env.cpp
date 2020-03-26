#include "../c/env.h"

// push into stack
void push(env* e, uint32_t item) {
  e->stack.push_back(item);
  e->stack_pointer++;
}
// pop out of stack
uint32_t pop(env* e) {
  auto tmp = e->stack.back();
  e->stack.pop_back();
  e->stack_pointer--;
  return tmp;
}