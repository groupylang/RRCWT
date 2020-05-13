#include "rcwtlib.h"

void print_int(env* e) {
  std::cout << e->stack[e->base_pointer-1];
  std::cout.flush();
}
void print_str(env* e) {
  std::cout << e->data[e->stack[e->base_pointer-1]];
  std::cout.flush();
}
