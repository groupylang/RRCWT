package front_end.token;

public class IntegerLiteral extends Token {
    public final int value;
    public IntegerLiteral(final int value) {
        super(Tag.INTEGER);
        this.value = value;
    }
    @Override
    public String toString() {
        return "" + value;
    }
}
