package ir;

import back_end.Builder;

import java.util.ArrayList;
import java.util.List;
import java.util.Collections;

public class Return implements Instruction {
    private final Operand src;
    public Return(final Operand src) {
        this.src = src;
    }
    @Override
    public String toString() {
        return "\treturn " + src.toString() + "\n";
    }
    @Override
    public String build() {
        return "  pop  rax\n" +
                Builder.epilogue();
    }
    @Override
    public Code reduce() {
        return this;
    }
    @Override
    public int toWC() {
        return 0x41000000;
    }
    @Override
    public String toAssembly() {
        return "\tmov     rax, " + src.toAssembly() +
                "\n\tmov     rsp, rbp\n\tpop     rbp\n\tret\n";
    }
    @Override
    public void print() {
        System.out.print("\tprint ");
        src.print();
        System.out.println();
    }
    @Override
    public List<String> registers() {
        List<String> registers = new ArrayList<>();
        if (src instanceof Register) registers.add(src.toString());
        return registers;
    }
}
