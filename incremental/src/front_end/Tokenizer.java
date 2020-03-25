package front_end;

import front_end.token.*;
import front_end.token.Number;

import java.util.HashMap;
import java.util.Map;

public class Tokenizer {
    private static String[] lines;
    private char[] input;
    private int position;
    private static int line_no;
    private static int column_no;
    private Map<String, Word> words;
    private TokenizerState state;

    Tokenizer(final String input) {
        this.input = input.toCharArray();
        this.position = 0;
        lines = input.lines().toArray(String[]::new);
        line_no = 1;
        column_no = 1;
        this.state = TokenizerState.STANDARD;
        this.words = new HashMap<>();
        reserve(new Word("if",      Tag.IF));
        reserve(new Word("else",    Tag.ELSE));
        reserve(new Word("while",   Tag.WHILE));
        reserve(new Word("do",      Tag.DO));
        reserve(new Word("break",   Tag.BREAK));
        reserve(new Word("return",  Tag.RET));
        reserve(new Word("print",   Tag.PRINT));
        reserve(new Word("Integer", Tag.INT));
    }
    private void reserve(final Word word) {
        words.put(word.lexeme, word);
    }
    Token tokenize() throws ParsingException {
        if (position >= input.length) {
            return Token.EOF;
        }
        switch (state) {
            case STANDARD:
                while (Character.isWhitespace(input[position])) {
                    if (input[position] == '\n') {
                        line_no++; column_no = 0;
                    }
                    position++; column_no++;
                }
                switch (input[position]) {
                    case '&':
                        position++; column_no++;
                        if (input[position] == '&') {
                            position++; column_no++;
                            return Word.and; // &&
                        } else {
                            return new Token('&');
                        }
                    case '|':
                        position++; column_no++;
                        if (input[position] == '|') {
                            position++; column_no++;
                            return Word.or; // ||
                        } else {
                            return new Token('|');
                        }
                    case '=':
                        position++; column_no++;
                        if (input[position] == '=') {
                            position++; column_no++;
                            return Word.eq; // ==
                        } else {
                            return new Token('=');
                        }
                    case '!':
                        position++; column_no++;
                        if (input[position] == '=') {
                            position++; column_no++;
                            return Word.ne; // !=
                        } else {
                            return new Token('!');
                        }
                    case '<':
                        position++; column_no++;
                        if (input[position] == '=') {
                            position++; column_no++;
                            return Word.le; // <=
                        } else {
                            return new Token('<');
                        }
                    case '>':
                        position++; column_no++;
                        if (input[position] == '=') {
                            position++; column_no++;
                            return Word.ge; // >=
                        } else {
                            return new Token('>');
                        }
                    case '/':
                        position++; column_no++;
                        if (input[position] == '/') {
                            position++; column_no++;
                            state = TokenizerState.SHORT_COMMENT;
                            return tokenize();
                        } else if (input[position] == '*') {
                            position++; column_no++;
                            state = TokenizerState.LONG_COMMENT;
                            return tokenize();
                        } else {
                            return new Token('/');
                        }
                    case '\"':
                        position++; column_no++;
                        state = TokenizerState.STRING;
                        return tokenize();
                }
                if (Character.isDigit(input[position])) {
                    int int_value = 0;
                    do {
                        int_value = 10 * int_value + Character.digit(input[position], 10);
                        position++; column_no++;
                    } while (Character.isDigit(input[position]));
                    if (input[position] != '.') {
                        return new Number(int_value);
                    }

                    float float_value = int_value;
                    for (int i = 0; ; i++) {
                        position++; column_no++;
                        if (!Character.isDigit(input[position])) break;
                        float_value += Character.digit(input[position], 10) / Math.pow(10, i + 1);
                    }
                    return new Real(float_value);
                } else if (Character.isLetter(input[position])) {
                    StringBuilder builder = new StringBuilder();
                    do {
                        builder.append(input[position]);
                        position++; column_no++;
                    } while (Character.isLetterOrDigit(input[position]));
                    String lexeme = builder.toString();
                    if (words.containsKey(lexeme)) {
                        return words.get(lexeme);
                    }
                    Word word = new Word(lexeme, Tag.ID);
                    words.put(lexeme, word);
                    return word;
                } else {
                    Token token = new Token(input[position]);
                    position++; column_no++;
                    return token;
                }
            case STRING:
                StringBuilder builder = new StringBuilder();
                while (input[position] != '\"') {
                    builder.append(input[position]);
                    if (input[position] == '\n') {
                        throw new ParsingException("string literal isn't multi-line token");
                    }
                    position++; column_no++;
                }
                position++; column_no++;
                state = TokenizerState.STANDARD;
                return new String_(builder.toString());
            case SHORT_COMMENT:
                while (input[position] != '\n') {
                    position++; column_no++;
                }
                line_no++; column_no = 0;
                position++;
                state = TokenizerState.STANDARD;
                return tokenize();
            case LONG_COMMENT:
                while (input[position] != '*' || input[position + 1] != '/') {
                    if (input[position] == '\n') {
                        line_no++; column_no = 0;
                    }
                    position++; column_no++;
                }
                position += 2; column_no += 2;
                state = TokenizerState.STANDARD;
                return tokenize();
        }
        throw new ParsingException("InvalidToken");
    }
    public static int line_no() {
        return line_no;
    }
    public static int column_no() {
        return column_no;
    }
    public static String line(int line_no) {
        return lines[line_no - 1];
    }
}
