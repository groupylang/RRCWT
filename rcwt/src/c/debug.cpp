#include "vm.h" /* vm.hをヘッダファイルとしますよ */

uint32_t break_flag_;
env* e;

void debugger(uint32_t text_size, uint32_t data_size, uint32_t numRegisters) {
   ;
  while(break_flag_){
    printf("数字を入力してください\n");
    printf("0...スレッドBにメッセージを送信します\n");
    printf("1...プロセスを終了します\n");
    scanf("%d", &input);
    switch(input){
      case 0:
        msg_ = 1;
        break;
      case 1:
        break_flag_ = 0;
        show_memory(env* e);
        break;
      default :
        printf("０か１を入力してください\n");
        break;
    }
  }
  printf("スレッドA終了\n");
}



void show_memory(env* e) {
  uint8_t* a = e -> text; /*命令列先頭アドレス*/
  std::cout << "text" <<  *a << std::endl; /*アドレス中身の表示*/

  for (size_t i = 0; i <= 1000; i++){ 
                                      /*アドレス表示の繰り返し*/
    for (size_t i = 0; i <= 4; i++) {
      a ++;                             /*アドレス表示4回繰り返し*/
      std::cout <<  *a << " " ;
    }
    std::cout << std::endl;
  }

  uint8_t* b = e -> data; /*データ列先頭アドレス*/
  std::cout << "data" <<  *b << std::endl; /*アドレス中身の表示*/

  for (size_t i = 0; i <= 1000; i++){ 
                                      /*アドレス表示の繰り返し*/
    for (size_t i = 0; i <= 4; i++) {
      b ++;                             /*アドレス表示4回繰り返し*/
      std::cout <<  *b << " " ;
    }
    std::cout << std::endl;
  } 

  uint32_t* c = e -> registers; /*レジスタ先頭アドレス*/
  std::cout << "register" <<  *c << std::endl; /*アドレス中身の表示*/

  for (size_t i = 0; i <= 1000; i++){ 
                                      /*アドレス表示の繰り返し*/
    for (size_t i = 0; i <= 4; i++) {
      c ++;                             /*アドレス表示4回繰り返し*/
      std::cout <<  *c << " " ;
    }
    std::cout << std::endl;
  }
  
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