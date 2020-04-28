#include "vm.h"

uint8_t debug_flag = false;
uint8_t alive_flag = true;

void show_array8(uint8_t* begin, uint32_t size) {
  size_t i;
  for (i = 0; i < size; i++) { 
    printf("%02x ", *begin);
    if (i % 4 == 3) std::cout << std::endl;
    begin++;
  }
  std::cout << std::endl;
}
void show_array32(uint32_t* begin, uint32_t size) {
  for (size_t i = 0; i < size; i++) { 
    printf("%08x", *begin);
    std::cout << std::endl;
    begin++;
  }
}

void debugger(env* e, uint32_t text_size, uint32_t data_size, uint32_t numRegisters) {
  while (alive_flag) {
    if (debug_flag) {
      std::cout << "\x1b[33m" // change font yellow
                << "text: " << std::endl
                << "\x1b[36m"; // change font cyan
      show_array8(e->text, text_size);
      std::cout << "\x1b[33m" // change font yellow
                << "data: " << std::endl
                << "\x1b[36m"; // change font cyan
      show_array8(e->data, data_size);
      std::cout << "\x1b[33m" // change font yellow
                << "registers: " << std::endl
                << "\x1b[36m"; // change font cyan
      show_array32(e->registers, numRegisters);
      std::cout << std::endl
                << "\x1b[39m"; // change font default
        std::cout << "\x1b[33m" // change font yellow
            << "stack:" << std::endl
            << "\x1b[36m"; // change font cyan
      for (auto dword: e->stack) {
        printf("%08x\n", dword);
      }
      std::cout << std::endl
                << "\x1b[33m" // change font yellow
                << "heap:" << std::endl
                << "\x1b[36m"; // change font cyan
      for (auto dword: e->heap) {
        printf("%08x\n", dword);
      }
      std::cout << std::endl
                << "\x1b[33m" // change font yellow
                << "stack pointer: "
                << "\x1b[36m"; // change font cyan
      printf("%08x", e->stack_pointer);
      std::cout << std::endl
                << "\x1b[33m" // change font yellow
                << "base pointer: "
                << "\x1b[36m"; // change font cyan
      printf("%08x", e->base_pointer);
      std::cout << std::endl
                << "\x1b[39m" // change font default
                << "> ";
      std::string input;
      std::cin >> input;
	    debug_flag = false;
    }
  }
}