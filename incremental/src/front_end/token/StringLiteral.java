package front_end.token;

public class StringLiteral extends Token {
    public final String value;
    public StringLiteral(final String value) {
        super(Tag.STRING);
        this.value = value;
    }
    @Override
    public String toString() {
        return value;
    }
}
