package ast;

public class BooleanLiteral extends Expression {
    final boolean value;
    public BooleanLiteral(final boolean value) {
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
        if (value) {
            return "#t";
        } else {
            return "#f";
        }
    }
}
