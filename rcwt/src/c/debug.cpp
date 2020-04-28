#include "vm.h" /* vm.hをヘッダファイルとしますよ */

uint8_t bp_flag = false;

void show_memory(env* e, uint32_t text_size, uint32_t data_size, uint32_t numRegisters) {
  uint8_t* a = e->text; /*命令列先頭アドレス*/
  std::cout << "text" << *a << std::endl; /*アドレス中身の表示*/

  for (size_t i = 0; i <= text_size; i++) { 
    a++;
    std::cout << *a << " ";
    if (i % 4 == 3) std::cout << std::endl;
  }

  uint8_t* b = e->data; /*データ列先頭アドレス*/
  std::cout << "data" << *b << std::endl; /*アドレス中身の表示*/

  for (size_t i = 0; i <= text_size; i++) { 
    b++;
    std::cout << *b << " ";
    if (i % 4 == 3) std::cout << std::endl;
  }

  uint32_t* c = e->registers; /*レジスタ先頭アドレス*/
  std::cout << "register" << *c << std::endl; /*アドレス中身の表示*/

  for (size_t i = 0; i <= text_size; i++) { 
    c++;
    std::cout << *c << " ";
    if (i % 4 == 3) std::cout << std::endl;
  }
}

void debugger(env* e, uint32_t text_size, uint32_t data_size, uint32_t numRegisters) {
//   while (true) {
//     if (bp_flag) {
//       show_memory(e, text_size, data_size, numRegisters);
// 	  bp_flag = false;
//     }
//   }
}

void bp(env* e) {
  std::cout << "\x1b[33m" // change font yellow
            << "debug | stack:" << std::endl
            << "\x1b[36m"; // change font cyan
  for (auto dword: e->stack) {
    printf("%08x\n", dword);
  }
  std::cout << std::endl
            << "\x1b[33m" // change font yellow
            << "debug | heap:" << std::endl
            << "\x1b[36m"; // change font cyan
  for (auto dword: e->heap) {
    printf("%08x\n", dword);
  }
  std::cout << std::endl
            << "\x1b[39m"; // change font default
  printf("debug | stack pointer: %08x\n", e->stack_pointer);
  printf("debug | base pointer: %08x\n", e->base_pointer);
  std::cout << "> ";
  std::string input;
  std::cin >> input;
}