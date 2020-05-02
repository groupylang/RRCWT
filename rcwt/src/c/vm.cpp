#include <utility>
#include <algorithm>
#include <string>
#include <iostream>
#include <fstream>

#include "vm.h"

uint8_t virtual_execute_wrapper(
  env* e,
  uint32_t text_size,
  uint32_t data_size,
  uint32_t numRegisters,
  uint32_t entry_point
) {
  std::thread th(debugger, *e, text_size, data_size, numRegisters);
  auto status = virtual_execute(
    /* vm          */ nullptr,
    /* e           */ e,
    /* entry point */ entry_point
  );
  SYNC([] { alive_flag = false; })
  th.join();
  return status;
}

env* env_new(uint8_t* text, uint8_t* data, uint32_t numRegisters) {
  env* e = new env;
  e->text          = text;
  e->data          = data;
  e->registers     = (uint32_t*) calloc(numRegisters, 4);
  e->stack         = vec_new();
  e->heap          = vec_new();
  e->stack_pointer = 0;
  e->base_pointer  = 0;
  return e;
}

std::vector<uint32_t> vec_new() {
  auto tmp = std::vector<uint32_t>();
  tmp.reserve(32);
  std::fill(tmp.begin(), tmp.end(), 0);
  return tmp;
}

uint8_t is_hot(std::unordered_map<size_t, uint32_t>& hot_spots, size_t pc) {
  if (hot_spots[pc] < 3) { hot_spots[pc]++; return 0; }
  else { return 1; }
}

#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
void jit_asm(std::unordered_map<size_t, procedure>& procs, size_t id, const char* jit_str) {
  std::ofstream fout(format("tmp/jit%zu.cpp", id).c_str());
  fout << jit_str;
  fout.flush();
  system(format("clang++ tmp/jit%zu.cpp -o tmp/jit%zu.so -Wall -Wextra -g -shared -fPIC", id, id).c_str());
  auto handle = LoadLibraryA(format("tmp/jit%zu.so", id).c_str());
  auto f = reinterpret_cast<procedure>(GetProcAddress(handle, "f"));
  procs[id] = f;
}
#elif defined(__linux)
#include <dlfcn.h>
void jit_asm(std::unordered_map<size_t, procedure>& procs, size_t id, const char* jit_str) {
  std::ofstream fout(format("tmp/jit%zu.cpp", id).c_str());
  fout << jit_str;
  fout.flush();
  system(format("clang++ tmp/jit%zu.cpp -o tmp/jit%zu.so -Wall -Wextra -g -shared -fPIC", id, id).c_str());
  auto handle = dlopen(format("tmp/jit%zu.so", id).c_str(), RTLD_LAZY);
  auto f = reinterpret_cast<procedure>(dlsym(handle, "f"));
  procs[id] = f;
}
#endif

void native_execute(std::unordered_map<size_t, procedure>& procs, size_t id, env* e) {
  procs[id](e);
}

uint8_t virtual_execute(uint32_t* vm, env* e, uint32_t entry_point) {
  // initialize
  uint8_t jit_flag = 0;
  std::string jit_str = std::string();
  auto hot_spots = std::unordered_map<size_t, uint32_t>();
  hot_spots.reserve(32);
  auto procs = std::unordered_map<size_t, procedure>();
  procs.reserve(32);
  // execute
// direct threading
#if defined __GNUC__ || defined __clang__ || defined __INTEL_COMPILER
  #define INIT_DISPATCH JUMP;
  #define CASE(op) L_ ## op:
  #define NEXT i=*++pc; goto *table[i.code]
  #define JUMP i=*pc; goto *table[i.code]
  #define END_DISPATCH
#else
  #define INIT_DISPATCH for (;;) { i=*pc; switch (i.code) {
  #define CASE(op) case op:
  #define NEXT ++pc; break
  #define JUMP break
  #define END_DISPATCH }}
#endif

  instruction* pc = reinterpret_cast<instruction*>(e->text + entry_point);
  instruction i;

#if defined __GNUC__ || defined __clang__ || defined __INTEL_COMPILER
  static void* table[] = {
      /* 00 */ &&L_NOP,   /* 01 */ &&L_NOP,   /* 02 */ &&L_BP,    /* 03 */ &&L_NOP,
      /* 04 */ &&L_STORE, /* 05 */ &&L_LOAD,  /* 06 */ &&L_PUSH,  /* 07 */ &&L_POP,
      /* 08 */ &&L_NOP,   /* 09 */ &&L_NOP,   /* 0a */ &&L_NOP,   /* 0b */ &&L_NOP,
      /* 0c */ &&L_NOP,   /* 0d */ &&L_NOP,   /* 0e */ &&L_NOP,   /* 0f */ &&L_NOP,

      /* 10 */ &&L_ADDR,  /* 11 */ &&L_SUBR,  /* 12 */ &&L_MULR,  /* 13 */ &&L_DIVR,
      /* 14 */ &&L_REMR,  /* 15 */ &&L_GT,    /* 16 */ &&L_GE,    /* 17 */ &&L_EQ,
      /* 18 */ &&L_AND,   /* 19 */ &&L_OR,    /* 1a */ &&L_NOT,   /* 1b */ &&L_XOR,
      /* 1c */ &&L_SHL,   /* 1d */ &&L_SHR,   /* 1e */ &&L_NOP,   /* 1f */ &&L_NOP,

      /* 20 */ &&L_ADDI,  /* 21 */ &&L_SUBI,  /* 22 */ &&L_MULI,  /* 23 */ &&L_DIVI,
      /* 24 */ &&L_REMI,  /* 25 */ &&L_NOP,   /* 26 */ &&L_NOP,   /* 27 */ &&L_NOP,
      /* 28 */ &&L_NOP,   /* 29 */ &&L_NOP,   /* 2a */ &&L_NOP,   /* 2b */ &&L_NOP,
      /* 2c */ &&L_SHLI,  /* 2d */ &&L_SHRI,  /* 2e */ &&L_NOP,   /* 2f */ &&L_NOP,

      /* 30 */ &&L_ADDA,  /* 31 */ &&L_SUBA,  /* 32 */ &&L_NOP,   /* 33 */ &&L_NOP,
      /* 34 */ &&L_NOP,   /* 35 */ &&L_NOP,   /* 36 */ &&L_NOP,   /* 37 */ &&L_NOP,
      /* 38 */ &&L_ANDA,  /* 39 */ &&L_ORA,   /* 3a */ &&L_NOP,   /* 3b */ &&L_NOP,
      /* 3c */ &&L_NOP,   /* 3d */ &&L_NOP,   /* 3e */ &&L_NOP,   /* 3f */ &&L_CASA,

      /* 40 */ &&L_GOTO,  /* 41 */ &&L_EXIT,  /* 42 */ &&L_CALL,  /* 43 */ &&L_RET,
      /* 44 */ &&L_IFGT,  /* 45 */ &&L_IFGE,  /* 46 */ &&L_IFEQ,  /* 47 */ &&L_NOP,
      /* 48 */ &&L_NOP,   /* 49 */ &&L_NOP,   /* 4a */ &&L_NOP,   /* 4b */ &&L_NOP,  
      /* 4c */ &&L_NOP,   /* 4d */ &&L_NOP,   /* 4e */ &&L_NOP,   /* 4f */ &&L_NOP,

      /* 50 */ &&L_NEW,   /* 51 */ &&L_SET,   /* 52 */ &&L_GET,   /* 53 */ &&L_NOP,
      /* 54 */ &&L_NOP,   /* 55 */ &&L_NOP,   /* 56 */ &&L_NOP,   /* 57 */ &&L_NOP,
      /* 58 */ &&L_NOP,   /* 59 */ &&L_NOP,   /* 5a */ &&L_NOP,   /* 5b */ &&L_NOP,
      /* 5c */ &&L_NOP,   /* 5d */ &&L_NOP,   /* 5e */ &&L_NOP,   /* 5f */ &&L_NOP,

      /* 60 */ &&L_FADD,  /* 61 */ &&L_FSUB,  /* 62 */ &&L_FMUL,  /* 63 */ &&L_FDIV,
      /* 64 */ &&L_NOP,   /* 65 */ &&L_FGT,   /* 66 */ &&L_FGE,   /* 67 */ &&L_FEQ,
      /* 68 */ &&L_NOP,   /* 69 */ &&L_NOP,   /* 6a */ &&L_NOP,   /* 6b */ &&L_NOP,
      /* 6c */ &&L_NOP,   /* 6d */ &&L_NOP,   /* 6e */ &&L_NOP,   /* 6f */ &&L_NOP,

      /* 70 */ &&L_NOP,   /* 71 */ &&L_NOP,   /* 72 */ &&L_NOP,   /* 73 */ &&L_NOP,
      /* 74 */ &&L_NOP,   /* 75 */ &&L_NOP,   /* 76 */ &&L_NOP,   /* 77 */ &&L_NOP,
      /* 78 */ &&L_NOP,   /* 79 */ &&L_NOP,   /* 7a */ &&L_NOP,   /* 7b */ &&L_NOP,
      /* 7c */ &&L_NOP,   /* 7d */ &&L_NOP,   /* 7e */ &&L_NOP,   /* 7f */ &&L_NOP,

      /* 80 */ &&L_NOP,   /* 81 */ &&L_NOP,   /* 82 */ &&L_NOP,   /* 83 */ &&L_NOP,
      /* 84 */ &&L_NOP,   /* 85 */ &&L_NOP,   /* 86 */ &&L_NOP,   /* 87 */ &&L_NOP,
      /* 88 */ &&L_NOP,   /* 89 */ &&L_NOP,   /* 8a */ &&L_NOP,   /* 8b */ &&L_NOP,
      /* 8c */ &&L_NOP,   /* 8d */ &&L_NOP,   /* 8e */ &&L_NOP,   /* 8f */ &&L_NOP,

      /* 90 */ &&L_NOP,   /* 91 */ &&L_NOP,   /* 92 */ &&L_NOP,   /* 93 */ &&L_NOP,
      /* 94 */ &&L_NOP,   /* 95 */ &&L_NOP,   /* 96 */ &&L_NOP,   /* 97 */ &&L_NOP,
      /* 98 */ &&L_NOP,   /* 99 */ &&L_NOP,   /* 9a */ &&L_NOP,   /* 9b */ &&L_NOP,
      /* 9c */ &&L_NOP,   /* 9d */ &&L_NOP,   /* 9e */ &&L_NOP,   /* 9f */ &&L_NOP,

      /* a0 */ &&L_NOP,   /* a1 */ &&L_NOP,   /* a2 */ &&L_NOP,   /* a3 */ &&L_NOP,
      /* a4 */ &&L_NOP,   /* a5 */ &&L_NOP,   /* a6 */ &&L_NOP,   /* a7 */ &&L_NOP,
      /* a8 */ &&L_NOP,   /* a9 */ &&L_NOP,   /* aa */ &&L_NOP,   /* ab */ &&L_NOP,
      /* ac */ &&L_NOP,   /* ad */ &&L_NOP,   /* ae */ &&L_NOP,   /* af */ &&L_NOP,

      /* b0 */ &&L_NOP,   /* b1 */ &&L_NOP,   /* b2 */ &&L_NOP,   /* b3 */ &&L_NOP,
      /* b4 */ &&L_NOP,   /* b5 */ &&L_NOP,   /* b6 */ &&L_NOP,   /* b7 */ &&L_NOP,
      /* b8 */ &&L_NOP,   /* b9 */ &&L_NOP,   /* ba */ &&L_NOP,   /* bb */ &&L_NOP,
      /* bc */ &&L_NOP,   /* bd */ &&L_NOP,   /* be */ &&L_NOP,   /* bf */ &&L_NOP,

      /* c0 */ &&L_NOP,   /* c1 */ &&L_NOP,   /* c2 */ &&L_NOP,   /* c3 */ &&L_NOP,
      /* c4 */ &&L_NOP,   /* c5 */ &&L_NOP,   /* c6 */ &&L_NOP,   /* c7 */ &&L_NOP,
      /* c8 */ &&L_NOP,   /* c9 */ &&L_NOP,   /* ca */ &&L_NOP,   /* cb */ &&L_NOP,
      /* cc */ &&L_NOP,   /* cd */ &&L_NOP,   /* ce */ &&L_NOP,   /* cf */ &&L_NOP,

      /* d0 */ &&L_NOP,   /* d1 */ &&L_NOP,   /* d2 */ &&L_NOP,   /* d3 */ &&L_NOP,
      /* d4 */ &&L_NOP,   /* d5 */ &&L_NOP,   /* d6 */ &&L_NOP,   /* d7 */ &&L_NOP,
      /* d8 */ &&L_NOP,   /* d9 */ &&L_NOP,   /* da */ &&L_NOP,   /* db */ &&L_NOP,
      /* dc */ &&L_NOP,   /* dd */ &&L_NOP,   /* de */ &&L_NOP,   /* df */ &&L_NOP,

      /* e0 */ &&L_NOP,   /* e1 */ &&L_NOP,   /* e2 */ &&L_NOP,   /* e3 */ &&L_NOP,
      /* e4 */ &&L_NOP,   /* e5 */ &&L_NOP,   /* e6 */ &&L_NOP,   /* e7 */ &&L_NOP,
      /* e8 */ &&L_NOP,   /* e9 */ &&L_NOP,   /* ea */ &&L_NOP,   /* eb */ &&L_NOP,
      /* ec */ &&L_NOP,   /* ed */ &&L_NOP,   /* ee */ &&L_NOP,   /* ef */ &&L_NOP,

      /* f0 */ &&L_NOP,   /* f1 */ &&L_NOP,   /* f2 */ &&L_NOP,   /* f3 */ &&L_NOP,
      /* f4 */ &&L_NOP,   /* f5 */ &&L_NOP,   /* f6 */ &&L_NOP,   /* f7 */ &&L_NOP,
      /* f8 */ &&L_NOP,   /* f9 */ &&L_NOP,   /* fa */ &&L_MOV,   /* fb */ &&L_NOP,
      /* fc */ &&L_NOP,   /* fd */ &&L_FOUT,  /* fe */ &&L_IOUT,  /* ff */ &&L_SOUT,
  };
#else
  #define NOP   0x00
  #define BP    0x02
  #define STORE 0x04
  #define LOAD  0x05
  #define PUSH  0x06
  #define POP   0x07
  #define ADDR  0x10
  #define SUBR  0x11
  #define MULR  0x12
  #define DIVR  0x13
  #define REMR  0x14
  #define GT    0x15
  #define GE    0x16
  #define EQ    0x17
  #define AND   0x18
  #define OR    0x19
  #define NOT   0x1a
  #define XOR   0x1b
  #define SHL   0x1c
  #define SHR   0x1d
  #define ADDI  0x20
  #define SUBI  0x21
  #define MULI  0x22
  #define DIVI  0x23
  #define REMI  0x24
  #define SHLI  0x2c
  #define SHRI  0x2d
  #define GOTO  0x40
  #define EXIT  0x41
  #define CALL  0x42
  #define RET   0x43
  #define IFGT  0x44
  #define IFGE  0x45
  #define IFEQ  0x46
  #define NEW   0x50
  #define SET   0x51
  #define GET   0x52
  #define FADD  0x60
  #define FSUB  0x61
  #define FMUL  0x62
  #define FDIV  0x63
  #define FGT   0x65
  #define FGE   0x66
  #define FEQ   0x67
  #define MOV   0xfa
  #define FOUT  0xfd
  #define IOUT  0xfe
  #define SOUT  0xff
#endif
  try {
  INIT_DISPATCH {
    CASE(NOP) {
    } NEXT;
    CASE(BP) {
      std::cout << "debug | break point" << std::endl << "> ";
      SYNC([] { debug_flag = true; })
      std::string input;
      std::cin >> input;
    } NEXT;
    CASE(STORE) {
      e->stack[e->base_pointer + i.op0] = e->registers[i.op1];
      if (jit_flag) {
        jit_str += format("\te->stack[e->base_pointer + %d] = e->registers[%d];\n", i.op0, i.op1);
      }
    } NEXT;
    CASE(LOAD) {
      e->registers[i.op0] = e->stack[e->base_pointer + i.op1];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->stack[e->base_pointer + %d];\n", i.op0, i.op1);
      }
    } NEXT;
    CASE(PUSH) {
      push(e, e->registers[i.op0]);
    } NEXT;
    CASE(POP) {
      e->registers[i.op0] = pop(e);
    } NEXT;

    CASE(ADDR) {
      e->registers[i.op0] = e->registers[i.op1] + e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] + e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(SUBR) {
      e->registers[i.op0] = e->registers[i.op1] - e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] - e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(MULR) {
      e->registers[i.op0] = e->registers[i.op1] * e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] * e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(DIVR) {
      e->registers[i.op0] = e->registers[i.op1] / e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] / e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(REMR) {
      e->registers[i.op0] = e->registers[i.op1] % e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] % e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(GT) {
      e->registers[i.op0] = e->registers[i.op1] > e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] > e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(GE) {
      e->registers[i.op0] = e->registers[i.op1] >= e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] >= e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(EQ) {
      e->registers[i.op0] = e->registers[i.op1] == e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] == e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(AND) {
      e->registers[i.op0] = e->registers[i.op1] && e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] && e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(OR) {
      e->registers[i.op0] = e->registers[i.op1] || e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] || e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(NOT) {
      e->registers[i.op0] = !e->registers[i.op1];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = !e->registers[%d];\n", i.op0, i.op1);
      }
    } NEXT;
    CASE(XOR) {
      e->registers[i.op0] = e->registers[i.op1] ^ e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] ^ e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(SHL) {
      e->registers[i.op0] = e->registers[i.op1] << e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] << e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(SHR) {
      e->registers[i.op0] = e->registers[i.op1] >> e->registers[i.op2];
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] >> e->registers[%d];\n", i.op0, i.op1, i.op2);
      }
    } NEXT;

    CASE(ADDI) {
      e->registers[i.op0] = e->registers[i.op1] + i.op2;
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] + %d;\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(SUBI) {
      e->registers[i.op0] = e->registers[i.op1] - i.op2;
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] - %d;\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(MULI) {
      e->registers[i.op0] = e->registers[i.op1] * i.op2;
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] * %d;\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(DIVI) {
      e->registers[i.op0] = e->registers[i.op1] / i.op2;
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] / %d;\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(REMI) {
      e->registers[i.op0] = e->registers[i.op1] % i.op2;
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] % %d;\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(SHLI) {
      e->registers[i.op0] = e->registers[i.op1] << i.op2;
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] >> %d;\n", i.op0, i.op1, i.op2);
      }
    } NEXT;
    CASE(SHRI) {
      e->registers[i.op0] = e->registers[i.op1] >> i.op2;
      if (jit_flag) {
        jit_str += format("\te->registers[%d] = e->registers[%d] << %d;\n", i.op0, i.op1, i.op2);
      }
    } NEXT;

#if defined __GNUC__ || defined __clang__ || defined __INTEL_COMPILER
    CASE(ADDA) {
      __sync_fetch_and_add(e->registers + i.op0, e->registers[i.op1]);
    } NEXT;
    CASE(SUBA) {
      __sync_fetch_and_sub(e->registers + i.op0, e->registers[i.op1]);
    } NEXT;
    CASE(ANDA) {
      __sync_fetch_and_and(e->registers + i.op0, e->registers[i.op1]);
    } NEXT;
    CASE(ORA) {
      __sync_fetch_and_or(e->registers + i.op0, e->registers[i.op1]);
    } NEXT;
    CASE(CASA) {
      __sync_bool_compare_and_swap(e->registers + i.op0, e->registers[i.op1], e->registers[i.op2]);
    } NEXT;
#endif

    CASE(GOTO) {
      pc += i.op0;
    } JUMP;
    CASE(EXIT) {
      return i.op0;
    } NEXT;
    CASE(CALL) {
      jit_flag = is_hot(hot_spots, reinterpret_cast<size_t>(pc));
      if (jit_flag == 2) { native_execute(procs, reinterpret_cast<size_t>(pc), e); NEXT; }
      push(e, e->base_pointer); // save bp to stack
      e->base_pointer = e->stack_pointer;
      e->stack_pointer += i.op0; // allocate locals
      push(e, reinterpret_cast<uint8_t*>(++pc) - e->text); // save pc to stack
      pc = reinterpret_cast<instruction*>(e->text + i.op2);
      if (jit_flag) {
        jit_str += format("#include\"../rcwt/src/c/env.h\"\nextern \"C\" void f(env* e) {\n");
      }
    } JUMP;
    CASE(RET) {
      pc = reinterpret_cast<instruction*>(e->text + pop(e)); // get pc from stack
      // jit
      if (jit_flag) {
        jit_str += "\treturn;\n}\n";
        jit_asm(procs, reinterpret_cast<size_t>(pc - 1), jit_str.c_str());
        jit_flag = 0;
      }
      e->stack_pointer = e->base_pointer; // free locals
      e->base_pointer = pop(e); // get bp from stack
    } JUMP;
    CASE(IFGT) {
      if (e->registers[i.op1] > e->registers[i.op2]) { pc += i.op0; JUMP; }
      else{ NEXT; }
    }
    CASE(IFGE) {
      if (e->registers[i.op1] >= e->registers[i.op2]) { pc += i.op0; JUMP; }
      else{ NEXT; }
    }
    CASE(IFEQ) {
      if (e->registers[i.op1] == e->registers[i.op2]) { pc += i.op0; JUMP; }
      else{ NEXT; }
    }

    CASE(NEW) {
      auto tmp = e->heap.size();
      for (uint8_t u = 0; u < i.op2; u++) e->heap.push_back(0);
      e->registers[i.op0] = tmp;
    } NEXT;
    CASE(SET) {
      e->heap[e->registers[i.op0] + i.op1] = e->registers[i.op2];
    } NEXT;
    CASE(GET) {
      e->registers[i.op2] = e->heap[e->registers[i.op0] + i.op1];
    } NEXT;

    CASE(FADD) {
      *reinterpret_cast<float*>(e->registers + i.op0)
        = *reinterpret_cast<float*>(e->registers + i.op1)
        + *reinterpret_cast<float*>(e->registers + i.op2);
    } NEXT;
    CASE(FSUB) {
      *reinterpret_cast<float*>(e->registers + i.op0)
        = *reinterpret_cast<float*>(e->registers + i.op1)
        - *reinterpret_cast<float*>(e->registers + i.op2);
    } NEXT;
    CASE(FMUL) {
      *reinterpret_cast<float*>(e->registers + i.op0)
        = *reinterpret_cast<float*>(e->registers + i.op1)
        * *reinterpret_cast<float*>(e->registers + i.op2);
    } NEXT;
    CASE(FDIV) {
      *reinterpret_cast<float*>(e->registers + i.op0)
        = *reinterpret_cast<float*>(e->registers + i.op1)
        / *reinterpret_cast<float*>(e->registers + i.op2);
    } NEXT;
    CASE(FGT) {
      *reinterpret_cast<float*>(e->registers + i.op0)
        = *reinterpret_cast<float*>(e->registers + i.op1)
        > *reinterpret_cast<float*>(e->registers + i.op2);
    } NEXT;
    CASE(FGE) {
      *reinterpret_cast<float*>(e->registers + i.op0)
        = *reinterpret_cast<float*>(e->registers + i.op1)
        >= *reinterpret_cast<float*>(e->registers + i.op2);
    } NEXT;
    CASE(FEQ) {
      *reinterpret_cast<float*>(e->registers + i.op0)
        = *reinterpret_cast<float*>(e->registers + i.op1)
        == *reinterpret_cast<float*>(e->registers + i.op2);
    } NEXT;

    // macros TODO be replaced with rcwtlib
    CASE(MOV) {
      e->registers[i.op0] = (i.op1 << 8) + i.op2;
    } NEXT;
    CASE(FOUT) {
      print_float(*reinterpret_cast<float*>(e->registers + i.op0));
    } NEXT;
    CASE(IOUT) {
      print_int(e->registers[i.op0]);
    } NEXT;
    CASE(SOUT) {
      print_str(reinterpret_cast<const char*>(e->data) + e->registers[i.op0]);
      if (jit_flag) {
        jit_str += format("\tprintf(\"%s\", e->data + e->registers[%d]);\n", "%s", i.op0);
      }
    } NEXT;
  } END_DISPATCH;

  } catch (std::invalid_argument) {
    std::cout << "error | InvalidArgument" << std::endl;
    return 1;
  } catch (std::length_error) {
    std::cout << "error | VectorOutOfBounds" << std::endl;
    return 1;
  } catch (std::out_of_range) {
    std::cout << "error | ObjectTooLong" << std::endl;
    return 1;
  } catch (std::bad_alloc) {
    std::cout << "error | BadAlloc" << std::endl;
    return 1;
  } catch (std::overflow_error) {
    std::cout << "error | OverFlow" << std::endl;
    return 1;
  } catch (std::underflow_error) {
    std::cout << "error | UnderFlow" << std::endl;
    return 1;
  }

  return 0;
}
