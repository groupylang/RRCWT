package ast;

public class StringLiteral extends Expression {
    private final int offset;
    public StringLiteral(final String value, final int offset) {
        super(value);
        this.offset = offset;
    }
    @Override
    public String toAssembly() {
        return "";
    }
    @Override
    public String toS(int tab) {
        return "\"" + operator + "\"";
    }
    @Override
    public ir.Operand toIR() {
        return new ir.Immediate(offset);
    } // TODO Memory
}
