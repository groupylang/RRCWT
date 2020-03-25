package ir;

import java.util.ArrayList;
import java.util.List;

public class Assign implements Instruction {
    private final Operand dst, src;
    public Assign(final Operand dst, final Operand src) {
        this.dst = dst;
        this.src = src;
    }
    @Override
    public String build() {
        return "  mov  " + dst.toString() + ", " + src.toString() + "\n";
    }
    @Override
    public Code reduce() {
        return this;
    }

    @Override
    public int toWC() {
        return 0x30000000 | (dst.toWC() << 16 & 0x00110000);
    }
    // TODO
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
        if (dst instanceof Register) registers.add(dst.toString());
        if (src instanceof Register) registers.add(src.toString());
        return registers;
    }
}
