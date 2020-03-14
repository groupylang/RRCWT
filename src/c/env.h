#ifndef RCWT_ENV_H
#define RCWT_ENV_H

#include "vm.h"

extern instruction* text;
extern uint8_t* data;
extern uint32_t* registers;
extern uint32_t* stack;
extern uint32_t* heap;
extern uint32_t stack_pointer;
extern uint32_t base_pointer;
extern uint32_t program_counter;

extern void push(uint32_t);
extern uint32_t pop();

#endif