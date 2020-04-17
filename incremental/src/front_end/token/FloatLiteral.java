package front_end.token;

public class FloatLiteral extends Token {
    public final float value;
    public FloatLiteral(final float value) {
        super(Tag.FLOAT);
        this.value = value;
    }
    @Override
    public String toString() {
        return "" + value;
    }
}
