package ast;

public class CharacterLiteral extends Expression {
    final char value;
    public CharacterLiteral(final char value) {
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
        return "'" + value + "'";
    }
}
