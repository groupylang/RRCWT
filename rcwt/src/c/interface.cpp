#include "vm.h" /* vm.hをヘッダファイルとしますよ */
#include <thread>

void print_int(uint32_t i) {
  std::cout << i;
  std::cout.flush();
}
void print_str(char* s) {
  std::cout << s;
  std::cout.flush();
}

int main() {
  std::cout << "[*] Disignate Register's number" << std::endl
            << "> ";
  uint32_t numRegisters;
  std::cin >> numRegisters;
  std::cout << "[*] Please write RCWT code; op2 op1 op0 code" << std::endl
            << "[*] Type in 0 to END" << std::endl;
  std::cin.setf( std::ios::hex, std::ios::basefield );
  auto text = std::vector<instruction>();
  while(true) {
    std::cout << "> ";
    uint32_t instr;
    std::cin >> instr;
    if (instr == 0) {
      // print all instructions user input
      for (auto instr: text) {
        printf("code: %02x op0: %02x op1: %02x op2: %02x\n",
          instr.code, instr.op0, instr.op1, instr.op2);
      }
      break;
    }
    text.push_back(*reinterpret_cast<instruction*>(&instr));
  }
  std::cout << "[*] Please write your data (without space)" << std::endl;
  std::cout << "[*] Type in exit to END" << std::endl;
  std::string data = "";
  while(true) {
    std::cout << "> ";
    std::string str;
    std::cin >> str;
    if (str == "exit") {
      // print all data user input
      for (auto c: data) {
        printf("%02x ", c);
      }
      std::cout << std::endl;
      break;
    }
    data += str;
    data += '\0';
  }

  std::cout << std::endl
            << std::endl
            << "[*] RCWT started"
            << std::endl;

  auto e = env_new(
    /* text          */ reinterpret_cast<uint8_t*>(&text[0]),
    /* data          */ reinterpret_cast<uint8_t*>(const_cast<char*>(data.c_str())),
    /* registers     */ numRegisters
  );

  std::thread debug_thread(debugger, e, text.size(), data.size(), numRegisters);

  auto status = virtual_execute(
    /* vm           */ nullptr,
    /* e            */ e,
    /* entry_point  */ 0
  );

  debug_thread.join();

  std::cout << std::endl
            << std::endl;
  printf("[*] RCWT was runned successfully with status: %d\n", status);
}
