#include "vm.h"
#include <fstream>
#include <locale>
#include <ctime>

uint8_t debug_flag = false;
uint8_t alive_flag = true;

void show_array8(uint8_t* begin, uint32_t size) {
  for (size_t i = 0; i < size; i++) { 
    printf("%02x ", *begin);
    if (i % 8 == 7) std::cout << std::endl;
    begin++;
  }
  std::cout << std::endl;
}
void show_array32(uint32_t* begin, uint32_t size) { 
  for (size_t i = 0; i < size; i++) { 
    printf("%08x ", *begin);
    if (i % 8 == 7) std::cout << std::endl;
    begin++;
  }  
}

void debugger(env* e, uint32_t text_size, uint32_t data_size, uint32_t numRegisters) {
  while (alive_flag) {
    if (debug_flag) {
      std::ofstream fout("debug.text");
      fout << "text: " << std::endl;
      show_array8(e->text, text_size);
      fout << "data: " << std::endl;
      show_array8(e->data, data_size);
      fout << "registers: " << std::endl;
      show_array32(e->registers, numRegisters);
      fout << std::endl;
        fout << "stack:" << std::endl;
      for (auto dword: e->stack) {
        char* b = "";
        snprintf(b, 8, "%08x", dword);
        fout << b << std::endl;
      }
      fout << std::endl
                << "heap:" << std::endl;
      for (auto dword: e->heap) {
        char* c = "";
        snprintf(c, 8, "%08x", dword);
        fout << c << std::endl;
      }
      fout << std::endl
                << "stack pointer: ";
      char* d = "";
      snprintf(d, 8, "%08x", e->stack_pointer);
      fout << d;
      fout << std::endl
                << "base pointer: ";
      char* f = "";
      snprintf(f, 8, "%08x", e->base_pointer);
      fout << f;
      fout << std::endl
                << "> ";
      fout.flush();
      std::string input;
      std::cin >> input;
	    debug_flag = false;
    }
  }
}