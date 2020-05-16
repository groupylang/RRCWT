#include "librcwt.h"

void print_int(env* e) {
  std::cout << ARG(1);
  std::cout.flush();
}
void print_intln(env* e) {
  std::cout << ARG(1) << std::endl;
}
void print_str(env* e) {
  std::cout << e->data[ARG(1)];
  std::cout.flush();
}
void print_strln(env* e) {
  std::cout << e->data[ARG(1)] << std::endl;
}
void print_float(env* e) {
  std::cout << reinterpret_cast<float*>(&ARG(1));
  std::cout.flush();
}
void print_floatln(env* e) {
  std::cout << reinterpret_cast<float*>(&ARG(1)) << std::endl;
}
