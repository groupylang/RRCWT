#include <fstream>

#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
#elif defined(__linux)
#include <dlfcn.h>
#endif

#include "vm.h"

uint8_t is_hot(std::unordered_map<size_t, uint32_t>& hot_spots, size_t pc) {
  if (hot_spots[pc] < 3) { hot_spots[pc]++; return 0; }
  else { return 1; }
}

void jit_asm(env& e, size_t id, const char* jit_str) {
  std::ofstream fout(format("tmp/jit%zu.cpp", id).c_str());
  fout << jit_str;
  fout.flush();
#if defined(_WIN32) || defined(_WIN64)
  system(format("clang++ tmp/jit%zu.cpp -o tmp/jit%zu.dll -Wall -Wextra -g -shared -fPIC", id, id).c_str());
  native_load(&e, id, "tmp/jit%zu.dll");
#elif defined(__linux)
  system(format("clang++ tmp/jit%zu.cpp -o tmp/jit%zu.so -Wall -Wextra -g -shared -fPIC", id, id).c_str());
  native_load(&e, id, "tmp/jit%zu.so");
#endif
}

void native_load_wrapper(env* e, size_t index, const char* path) {
  auto ce = reinterpret_cast<cenv*>(e);
  native_load(e, reinterpret_cast<size_t>(ce->text + index * 4), path);
}

void native_load(env* e, size_t id, const char* path) {
#if defined(_WIN32) || defined(_WIN64)
  auto handle = LoadLibraryA(path);
  auto f = reinterpret_cast<procedure>(GetProcAddress(handle, "f"));
#elif defined(__linux)
  auto handle = dlopen(path, RTLD_LAZY);
  auto f = reinterpret_cast<procedure>(dlsym(handle, "f"));
#endif
  e->procs[id] = f;
}
void native_execute(std::unordered_map<size_t, procedure>& procs, size_t id, cenv* e) {
  procs[id](e);
}