#include "env.h"

void push(env* e, uint32_t item) {
  e->stack.push_back(item);
  e->stack_pointer++;
}
uint32_t pop(env* e) {
  auto tmp = e->stack.back();
  e->stack.pop_back();
  e->stack_pointer--;
  return tmp;
}