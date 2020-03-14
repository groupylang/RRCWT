#include "vm.h"

instruction* text;
uint8_t* data;
uint32_t* registers;
uint32_t* stack;
uint32_t* heap;
uint32_t stack_pointer;
uint32_t base_pointer;

void push(uint32_t item) {
  stack[stack_pointer++] = item;
}
uint32_t pop(void) {
  return stack[--stack_pointer];
}

void v_exec(uint32_t entry_point) {
  // initialize
  registers = (uint32_t*) malloc(1024);
  stack = (uint32_t*) malloc(1024);
  heap = (uint32_t*) malloc(1024);
  stack_pointer = 0;
  base_pointer = 0;
  // execute
// direct threading
#if defined __GNUC__ || defined __clang__ || defined __INTEL_COMPILER
  #define INIT_DISPATCH JUMP;
  #define CASE(op) L_ ## op:
  #define NEXT i=*++pc; goto *table[i.code]
  #define JUMP i=*pc; goto *table[i.code]
  #define END_DISPATCH
#else
  #define INIT_DISPATCH for (;;) { i = *pc; switch (i.code) {
  #define CASE(op) case op:
  #define NEXT pc++; break
  #define JUMP break
  #define END_DISPATCH }}
#endif

  instruction* pc = text + entry_point;
  instruction i;

#if defined __GNUC__ || defined __clang__ || defined __INTEL_COMPILER

  static void* table[] = {
      /* 00 */ &&L_NOP,   /* 01 */ &&L_NOP,   /* 02 */ &&L_BP,    /* 03 */ &&L_NOP,
      /* 04 */ &&L_STORE, /* 05 */ &&L_LOAD,  /* 06 */ &&L_PUSH,  /* 07 */ &&L_POP,
      /* 08 */ &&L_NOP,   /* 09 */ &&L_NOP,   /* 0a */ &&L_NOP,   /* 0b */ &&L_NOP,
      /* 0c */ &&L_NOP,   /* 0d */ &&L_NOP,   /* 0e */ &&L_NOP,   /* 0f */ &&L_NOP,

      /* 10 */ &&L_ADDR,  /* 11 */ &&L_SUBR,  /* 12 */ &&L_MULR,  /* 13 */ &&L_DIVR,
      /* 14 */ &&L_GT,    /* 15 */ &&L_GE,    /* 16 */ &&L_EQ,    /* 17 */ &&L_NOP,
      /* 18 */ &&L_AND,   /* 19 */ &&L_OR,    /* 1a */ &&L_NOT,   /* 1b */ &&L_NOP,
      /* 1c */ &&L_SHL,   /* 1d */ &&L_SHR,   /* 1e */ &&L_NOP,   /* 1f */ &&L_NOP,

      /* 20 */ &&L_ADDI,  /* 21 */ &&L_SUBI,  /* 22 */ &&L_MULI,  /* 23 */ &&L_DIVI,
	    /* 24 */ &&L_NOP,   /* 25 */ &&L_NOP,   /* 26 */ &&L_NOP,   /* 27 */ &&L_NOP,
      /* 28 */ &&L_NOP,   /* 29 */ &&L_NOP,   /* 2a */ &&L_NOP,   /* 2b */ &&L_NOP,
      /* 2c */ &&L_NOP,   /* 2d */ &&L_NOP,   /* 2e */ &&L_NOP,   /* 2f */ &&L_NOP,

      /* 30 */ &&L_NOP,   /* 31 */ &&L_NOP,   /* 32 */ &&L_NOP,   /* 33 */ &&L_NOP,
	    /* 34 */ &&L_NOP,   /* 35 */ &&L_NOP,   /* 36 */ &&L_NOP,   /* 37 */ &&L_NOP,
      /* 38 */ &&L_NOP,   /* 39 */ &&L_NOP,   /* 3a */ &&L_NOP,   /* 3b */ &&L_NOP,
      /* 3c */ &&L_NOP,   /* 3d */ &&L_NOP,   /* 3e */ &&L_NOP,   /* 3f */ &&L_NOP,

      /* 40 */ &&L_GOTO,  /* 41 */ &&L_EXIT,  /* 42 */ &&L_CALL,  /* 43 */ &&L_RET,
      /* 44 */ &&L_IFGT,  /* 45 */ &&L_IFGE,  /* 46 */ &&L_IFEQ,  /* 47 */ &&L_NOP,
      /* 48 */ &&L_NOP,   /* 49 */ &&L_NOP,   /* 4a */ &&L_NOP,   /* 4b */ &&L_NOP,  
      /* 4c */ &&L_NOP,   /* 4d */ &&L_NOP,   /* 4e */ &&L_NOP,   /* 4f */ &&L_NOP,

      /* 50 */ &&L_NEW,   /* 51 */ &&L_SET,   /* 52 */ &&L_GET
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
  #define GT    0x14
  #define GE    0x15
  #define EQ    0x16
  #define AND   0x18
  #define OR    0x19
  #define NOT   0x1a
  #define SHL   0x1c
  #define SHR   0x1d
  #define ADDI  0x20
  #define SUBI  0x21
  #define MULI  0x22
  #define DIVI  0x23
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
#endif

	INIT_DISPATCH {
    CASE(NOP) {
    } NEXT;
		CASE(BP) {
      pc++;
		  printf("debug | code: %x op0: %x op1: %x op2: %x\n", i.code, i.op0, i.op1, i.op2);
		} JUMP;
    CASE(STORE) {
      stack[base_pointer + i.op0] = registers[i.op1];
    } NEXT;
    CASE(LOAD) {
      registers[i.op1] = stack[base_pointer + i.op0];
    } NEXT;
    CASE(PUSH) {
      push(registers[i.op0]);
    } NEXT;
    CASE(POP) {
      registers[i.op0] = pop();
    } NEXT;

		CASE(ADDR) {
		  registers[i.op0] = registers[i.op1] + registers[i.op2];
		} NEXT;
		CASE(SUBR) {
      registers[i.op0] = registers[i.op1] - registers[i.op2];
		} NEXT;
    CASE(MULR) {
      registers[i.op0] = registers[i.op1] * registers[i.op2];
    } NEXT;
    CASE(DIVR) {
      registers[i.op0] = registers[i.op1] / registers[i.op2];
    } NEXT;
    CASE(GT) {
      registers[i.op0] = registers[i.op1] > registers[i.op2];
    } NEXT;
    CASE(GE) {
      registers[i.op0] = registers[i.op1] >= registers[i.op2];
    } NEXT;
    CASE(EQ) {
      registers[i.op0] = registers[i.op1] == registers[i.op2];
    } NEXT;
    CASE(AND) {
      registers[i.op0] = registers[i.op1] && registers[i.op2];
    } NEXT;
    CASE(OR) {
      registers[i.op0] = registers[i.op1] || registers[i.op2];
    } NEXT;
    CASE(NOT) {
      registers[i.op0] = !registers[i.op1];
    } NEXT;
    CASE(SHL) {
      registers[i.op0] = registers[i.op1] >> registers[i.op2];
    } NEXT;
    CASE(SHR) {
      registers[i.op0] = registers[i.op1] << registers[i.op2];
    } NEXT;

    CASE(ADDI) {
		  registers[i.op0] = registers[i.op1] + i.op2;
		} NEXT;
		CASE(SUBI) {
      registers[i.op0] = registers[i.op1] - i.op2;
		} NEXT;
    CASE(MULI) {
      registers[i.op0] = registers[i.op1] * i.op2;
    } NEXT;
    CASE(DIVI) {
      registers[i.op0] = registers[i.op1] / i.op2;
    } NEXT;

    CASE(GOTO) {
      pc += i.op0;
    } JUMP;
    CASE(EXIT) {
			return;
		} NEXT;
    CASE(CALL) {
      push(base_pointer);
      base_pointer = stack_pointer;
      for (int8_t u = 0; u < i.op0; u++) push(0u);
      push((uint32_t) pc);
      pc = text + i.op2;
      // TODO jit
    } JUMP;
    CASE(RET) {
      pc = (instruction*) pop();
      stack_pointer = base_pointer;
      base_pointer = pop();
    } JUMP;
    CASE(IFGT) {
      if (registers[i.op1] > registers[i.op2]) { pc += i.op0; JUMP; }
      else { NEXT; }
    }
    CASE(IFGE) {
      if (registers[i.op1] >= registers[i.op2]) { pc += i.op0; JUMP; }
      else { NEXT; }
    }
    CASE(IFEQ) {
      if (registers[i.op1] == registers[i.op2]) { pc += i.op0; JUMP; }
      else { NEXT; }
    }

    CASE(NEW) {
      // TODO
    } NEXT;
    CASE(SET) {
      heap[registers[i.op0] + i.op1] = registers[i.op2];
    } NEXT;
    CASE(GET) {
      registers[i.op2] = heap[registers[i.op0] + i.op1];
    } NEXT;
  } END_DISPATCH;
}