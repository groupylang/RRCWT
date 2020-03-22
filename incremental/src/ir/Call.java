package ir;

import java.util.List;

public class Call implements Code {
    private final Register register;
    private final String function_name;
    private final List<Operand> arguments;
    public Call(final Register register, final String function_name, final List<Operand> arguments) {
        this.register = register;
        this.function_name = function_name;
        this.arguments = arguments;
    }
    @Override
    public String toString() {
        return "\t" + register.toString() + " = call " + function_name + " " + arguments + "\n";
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
}
