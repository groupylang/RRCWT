#include <functional>

#include "rcwtlib.h"

void hash_int(env* e) {
  push(e, std::hash<uint32_t>()(ARG(1)));
}
void hash_float(env* e) {
  push(e, std::hash<float>()(*reinterpret_cast<float*>(&ARG(1))));
}
void hash_str(env* e) {
  push(e, std::hash<const uint8_t*>()(e->data + ARG(1)));
}