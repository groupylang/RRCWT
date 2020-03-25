package ir;

import java.util.ArrayList;
import java.util.List;

public class Branch implements Instruction {
    private final String instruction;
    private final Operand cond;
    private final int dst; // TODO String
    public Branch(final String instruction, final Operand cond, final int dst) {
        this.instruction = instruction;
        this.cond = cond;
        this.dst = dst;
    }
    public Branch(final int dst) {
        this.instruction = null; // TODO remove
        this.dst = dst;
        this.cond = Register.TRUE;
    }
    public Branch(final int dst, final Register cond) {
        this.instruction = null; // TODO remove
        this.dst = dst;
        this.cond = cond;
    }
    public String dst() {
        return "$" + dst;
    }
    @Override
    public String toString() {
        return "\t" + instruction + " " + cond.toString() + " goto $" + dst + "\n";
    }
    @Override
    public String build() {
        switch (instruction) {
            case "iffalse":
                return cond.build() +
                        "  cmp  rax, 0\n" +
                        "  je   .L" +
                        dst +
                        "\n";
            case "":
                return "  jmp  .L" +
                        dst +
                        "\n";
            default:
                System.out.println("Building Exception");
                System.exit(1);
                return null;
        }
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
        if (!cond.equals(Register.TRUE)) {
            System.out.print("if ");
            cond.print();
            System.out.print(" ");
        }
        System.out.println("goto $" + dst);
    }
    @Override
    public List<String> registers() {
        List<String> registers = new ArrayList<>();
        if (cond instanceof Register) registers.add(cond.toString());
        return registers;
    }
}
