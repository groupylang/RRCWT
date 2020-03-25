package ir;

import java.util.ArrayList;
import java.util.List;

public class ThreeAddress implements Instruction {
    private final Register dst;
    private final Operand src1;
    private final String op;
    private final Operand src2;
    public ThreeAddress(final Register dst, final Operand src1, final String op, final Operand src2) {
        this.dst = dst;
        this.src1 = src1;
        this.op = op;
        this.src2 = src2;
    }
    @Override
    public String toString() {
        return "\t" + dst.toString() + " = " + src1.toString() + " " + op + " " + src2.toString() + "\n";
    }
    @Override
    public String build() {
        final StringBuilder assembly = new StringBuilder();
        assembly.append(dst.build())
                .append("  pop  rdi\n  pop  rax\n");
        switch (op) {
            case "+":
                assembly.append("  add  rax, rdi\n  push rax\n");
            case "-":
                assembly.append("  sub  rax, rdi\n  push rax\n");
            case "*":
                assembly.append("  imul rax, rdi\n  push rax\n");
            case "/":
                assembly.append("  cqo\n  idiv rdi\n  push rax\n");
        }
        assembly.append(src1.build());
        assembly.append(src2.build());
        return assembly.toString();
    }
    public Code reduce() {
        if (src1 instanceof Immediate && src2 instanceof Immediate) {
            final int left_value = ((Immediate) src1).value;
            final int right_value = ((Immediate) src2).value;
            switch (op) {
                case "+":
                    return new TwoAddress(dst, new Immediate(left_value + right_value));
                case "-":
                    return new TwoAddress(dst, new Immediate(left_value - right_value));
                case "*":
                    return new TwoAddress(dst, new Immediate(left_value * right_value));
                case "/":
                    return new TwoAddress(dst, new Immediate(left_value / right_value));
            }
        } else if (src1 instanceof Immediate) {
            final int left_value = ((Immediate) src1).value;
            switch (op) {
                case "+": case "-":
                    if (left_value == 0) {
                        return new TwoAddress(dst, src1);
                    }
                case "*": case "/":
                    if (left_value == 1) {
                        return new TwoAddress(dst, src1);
                    } else if (left_value == 0) {
                        return new TwoAddress(dst, new Immediate(0));
                    }
            }
        } else if (src2 instanceof Immediate) {
            final int right_value = ((Immediate) src2).value;
            switch (op) {
                case "+": case "-":
                    if (right_value == 0) {
                        return new TwoAddress(dst, src2);
                    }
                case "*": case "/":
                    if (right_value == 1) {
                        return new TwoAddress(dst, src2);
                    } else if (right_value == 0) {
                        return new TwoAddress(dst, new Immediate(0));
                    }
            }
        }
        return this;
    }
    @Override
    public int toWC() {
        switch (op) {
            case "+":
                return 0x20000000 |
                        (dst.toWC() << 16 & 0xff0000) |
                        (src1.toWC() << 8 & 0xff00) |
                        src2.toWC();
            case "-":
                return 0x23000000 |
                        (dst.toWC() << 16 & 0xff0000) |
                        (src1.toWC() << 8 & 0xff00) |
                        src2.toWC();
            case "*":
                return 0x26000000 |
                        (dst.toWC() << 16 & 0xff0000) |
                        (src1.toWC() << 8 & 0xff00) |
                        src2.toWC();
            case "/":
                return 0x29000000 |
                        (dst.toWC() << 16 & 0xff0000) |
                        (src1.toWC() << 8 & 0xff00) |
                        src2.toWC();
            default:
                return 0;
        }
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
        src1.print();
        System.out.print(" " + op + " ");
        src2.print();
        System.out.println();
    }
    @Override
    public List<String> registers() {
        List<String> registers = new ArrayList<>();
        registers.add(dst.toString());
        if (src1 instanceof Register) registers.add(src1.toString());
        if (src2 instanceof Register) registers.add(src2.toString());
        return registers;
    }
}
