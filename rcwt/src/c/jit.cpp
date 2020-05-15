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

/// @arg id identifier of native procedure = address of virtual procedure's first instruction
void jit_compile(env& e, size_t id, const char* jit_str) {
  std::ofstream fout(format("tmp/jit%zu.cpp", id).c_str());
  fout << jit_str;
  fout.flush();
  // compile
#if defined(_WIN32) || defined(_WIN64)
  system(format("clang++ tmp/jit%zu.cpp -o tmp/libjit%zu.dll -Wall -Wextra -g -shared -fPIC", id, id).c_str());
#elif defined(__linux)
  system(format("clang++ tmp/jit%zu.cpp -o tmp/libjit%zu.so -Wall -Wextra -g -shared -fPIC", id, id).c_str());
#endif
  native_load(&e, id, format("tmp/jit%zu", id).c_str(), "f");
}

/// @arg index index of NCALL instruction which call function in @param path
/// @arg name name of native procedure
void native_load_wrapper(env* e, size_t index, const char* path, const char* name) {
  native_load(e, reinterpret_cast<size_t>(e->text + index * 4), path, name);
}

/// @arg id identifier of native procedure = address of virtual procedure's first instruction
/// @arg name name of native procedure
void native_load(env* e, size_t id, const char* path, const char* name) {
#if defined(_WIN32) || defined(_WIN64)
  auto handle = LoadLibraryA(format("%s.dll", path).c_str());
#elif defined(__linux)
  auto handle = dlopen(format("%s.so", path).c_str(), RTLD_LAZY);
#endif
  if (!handle) {
    std::cout << "error | FileNotFound: " << path << std::endl;
    exit(1);
  }
#if defined(_WIN32) || defined(_WIN64)
  auto fn = reinterpret_cast<procedure>(GetProcAddress(handle, name));
#elif defined(__linux)
  auto fn = reinterpret_cast<procedure>(dlsym(handle, name));
#endif
  if (!fn) {
    std::cout << "error | SymbolNotFound: " << name << std::endl;
    exit(1);
  }
  e->natives[id] = fn;
}

/// @arg id identifier of native procedure = address of virtual procedure's first instruction
void native_execute(std::unordered_map<size_t, procedure>& natives, size_t id, env* e) {
  natives[id](e);
}