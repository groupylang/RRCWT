package front_end.token;

public class CharacterLiteral extends Token {
    public final char value;
    public CharacterLiteral(final char value) {
        super(Tag.CHARACTER);
        this.value = value;
    }
    @Override
    public String toString() {
        return "" + value;
    }
}
