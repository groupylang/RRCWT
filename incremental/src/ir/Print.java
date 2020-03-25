package ir;

import java.util.ArrayList;
import java.util.List;
import java.util.Collections;

public class Print implements Instruction {
    private final Operand src;
    public Print(final Operand src) {
        this.src = src;
    }
    @Override
    public String toString() {
        return "\tprint " + src.toString() + "\n";
    }
    @Override
    public String build() {
        return "  out  " + src.toString();
    }
    @Override
    public Code reduce() {
        return this;
    }

    @Override
    public int toWC() {
        if (src instanceof String_) {
            return 0xfe000000 | (src.toWC() << 16 & 0xff0000);
        } else {
            return 0xff000000 | (src.toWC() << 16 & 0xff0000);
        }
    }
    // TODO
    public String toAssembly() {
        return "\tmov     rdi, [.Lc" + src.toAssembly() + "]\n\tcall    _printf\n";
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
