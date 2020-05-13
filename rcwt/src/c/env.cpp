#include "env.h"

void push(env* e, uint32_t item) {
  e->stack.push_back(item);
  SP++;
}
uint32_t pop(env* e) {
  auto tmp = e->stack.back();
  e->stack.pop_back();
  SP--;
  return tmp;
}
void breakpoint() {
  std::string input;
  std::cin >> input;
}