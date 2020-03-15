#include "vm.h"

#if defined(_WIN32) || defined(_WIN64)
  #include <windows.h>
  procedure jit(char* jit_str) {
    sprintf(jit_str, "%s#include <windows.h>\nBOOL APIENTRY DllMain(HANDLE h, DWORD d, LPVOID l) {\n\treturn TRUE;\n}\n", jit_str);
    FILE *c;
    c = fopen("tmp/f.c", "w");
    if (!c) {
      printf("error | FileNotFound: f.c\n");
      exit(1);
    }
    fprintf(c, "%s", jit_str);
    fclose(c);
    system("clang tmp/f.c -o tmp/f.dll -Wall -g -shared -fPIC");
    HANDLE handle = LoadLibrary("tmp/f.dll");
    if (!handle) {
      printf("error | FileNotFound: f.dll\n");
      exit(1);
    }
    return (procedure) GetProcAddress(handle, "f");
  }
#elif defined(__linux__)
  #include <dlfcn.h>
  procedure jit(char* jit_str) {
    FILE *c;
    c = fopen("tmp/f.c", "w");
    if (!c) {
      printf("FileNotFound: f.c\n");
      exit(1);
    }
    fprintf(c, "%s", jit_str);
    fclose(c);
    system("clang tmp/f.c -o tmp/f.so -Wall -g -shared -fPIC");
    void* handle = dlopen("tmp/f.so", RTLD_LAZY);
    if (!handle) {
      printf("error | %s", dlerror());
      exit(1);
    }
    return (procedure) dlsym(handle, "f");
  }
#endif