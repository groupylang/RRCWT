#include <chrono>
#include <fstream>
#include <iostream>
#include <iomanip>

#include "vm.h"

uint8_t debug_flag = false;
uint8_t alive_flag = true;

inline char* strNow() {
  auto time_point_now = std::chrono::system_clock::now();
  auto time_t_now =  std::chrono::system_clock::to_time_t(time_point_now);
  auto tm_now = std::localtime(&time_t_now);
  char* buf = new char[128];
  strftime(buf, sizeof(buf), "%H%M%S", tm_now);
  return buf;
}

inline void show_array8(std::ofstream& fout, const uint8_t* begin, uint32_t size) {
  size_t i;
  for (i = 0; i < size; i++) { 
    fout << format("%02x ", *begin);
    if (i % 4 == 3) fout << std::endl;
    begin++;
  }
  fout << std::endl;
}
inline void show_array32(std::ofstream& fout, const uint32_t* begin, uint32_t size) {
  for (size_t i = 0; i < size; i++) { 
    fout << format("%08x", *begin);
    fout << std::endl;
    begin++;
  }
}

void debugger(env* e, uint32_t text_size, uint32_t data_size, uint32_t numRegisters) {
  auto ce = reinterpret_cast<cenv*>(e);
  while (alive_flag) {
    if (debug_flag) {
      SYNC([] { debug_flag = false; })
      std::ofstream fout(format("tmp/dump_%s.txt", strNow()));

      fout << "text: "      << std::endl; show_array8(fout, ce->text, text_size);
      fout << "data: "      << std::endl; show_array8(fout, ce->data, data_size);
      fout << "registers: " << std::endl; show_array32(fout, ce->registers, numRegisters);
      fout << std::endl
           << "stack:" << std::endl;
      for (auto dword: ce->stack) {
        fout << format("%08x", dword) << std::endl;
      }
      fout << std::endl
           << "heap:" << std::endl;
      for (auto dword: ce->heap) {
        fout << format("%08x", dword) << std::endl;
      }
      fout << std::endl
           << "stack pointer: " << format("%08x", ce->stack_pointer) << std::endl
           << "base pointer: "  << format("%08x", ce->base_pointer)  << std::endl;
      fout.flush();
    }
  }
}