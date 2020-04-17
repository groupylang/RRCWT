package ast;

public class IntegerLiteral extends Expression {
    final int value;
    public IntegerLiteral(final int value) {
        super("" + value);
        this.value = value;
    }
    @Override
    public void jumping(final int _true,  final int _false) {}
    @Override
    public String toAssembly() {
        return "  push " + value + "\n";
    }
    @Override
    public String toS(int tab) {
        return "" + value;
    }
    @Override
    public ir.Operand toIR() {
        return new ir.Immediate(value);
    }
}
