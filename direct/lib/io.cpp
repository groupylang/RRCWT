#include "rcwtlib.h"

#include <iostream>

void print_int(uint32_t i) {
  std::cout << i;
  std::cout.flush();
}
void print_str(char* s) {
  std::cout << s;
  std::cout.flush();
}
