package ir;

import java.util.List;
import java.util.stream.Collectors;
import java.util.ArrayList;

public class Call implements Instruction {
    private final Register dst;
    private final String function_name;
    private final List<Operand> args;
    public Call(final Register dst, final String function_name, final List<Operand> args) {
        this.dst = dst;
        this.function_name = function_name;
        this.args = args;
    }
    @Override
    public String toString() {
        return "\t" + dst.toString() + " = call " + function_name + " " + args + "\n";
    }
    @Override
    public String build() {
        return "";
    }
    @Override
    public Code reduce() {
        return this;
    }
    @Override
    public int toWC() {
        return 0x42000000;
    }
    @Override
    public String toAssembly() {
        return "";
    }
    public static final Register ZERO = new Register(0);
    @Override
    public void print() {
        System.out.print("\t");
        dst.print();
        System.out.print(" = call ");
        args.forEach(arg -> { arg.print(); System.out.print(" "); });
        System.out.println();
    }
    @Override
    public List<String> registers() {
        List<String> registers = new ArrayList<>();
        registers.add(dst.toString());
        registers.addAll(
            args.stream()
                    .filter(arg -> arg instanceof Register)
                    .map(Operand::toString)
                    .collect(Collectors.toList()));
        return registers;
    }
}
