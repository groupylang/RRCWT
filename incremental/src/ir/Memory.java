package ir;

public class Memory implements Operand {
    @Override
    public String build() {
        return "";
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
        System.out.print("[]");
    }
}
