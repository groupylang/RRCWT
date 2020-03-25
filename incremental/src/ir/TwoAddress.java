package ir;

import java.util.ArrayList;
import java.util.List;

public class TwoAddress implements Instruction {
    private final Register dst;
    private final Operand src;
    public TwoAddress(final Register dst, final Operand src) {
        this.dst = dst;
        this.src = src;
    }
    @Override
    public String toString() {
        return "\t" + dst.toString() + " = " + src.toString() + "\n";
    }
    @Override
    public String build() {
        return src.build() +
                dst.build() +
                "  mov  rax, rdi\n";
    }
    @Override
    public Code reduce() {
        return this;
    }
    @Override
    public int toWC() {
        return 0;
    }
    @Override
    public String toAssembly() {
        return "";
    }
    @Override
    public void print() {
        System.out.print("\t");
        dst.print();
        System.out.print(" = ");
        src.print();
        System.out.println();
    }
    @Override
    public List<String> registers() {
        List<String> registers = new ArrayList<>();
        registers.add(dst.toString());
        if (src instanceof Register) registers.add(src.toString());
        return registers;
    }
}
