package front_end;

public class ParsingException extends Exception {
    private final int line_no;
    private final int column_no;
    private final String message;
    ParsingException(final String message) {
        this.line_no = Tokenizer.line_no();
        this.column_no = Tokenizer.column_no();
        this.message = message;
    }
    @Override
    public String toString() {
        StringBuilder builder = new StringBuilder("ParsingException | ");
        builder.append(message)
                .append('\n')
                .append(line_no)
                .append(": ")
                .append(Tokenizer.line(line_no))
                .append('\n')
                .append(" ".repeat(Math.max(0, String.valueOf(line_no).length() + column_no)))
                .append('^');
        return builder.toString();
    }
}
