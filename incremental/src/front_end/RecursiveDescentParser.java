package front_end;

import front_end.token.*;

import java.util.*;

public class RecursiveDescentParser implements Parser {
    private Tokenizer tokenizer;
    private Token now;
    private SymbolList current;
    private List<String> strings;

    public RecursiveDescentParser() {}
    private boolean match(final int tag) {
        return now.tag == tag;
    }
    private void consume(final int tag) throws ParsingException {
        if (match(tag)) {
            now = tokenizer.tokenize();
        } else {
            if (tag < 0xff && now.tag < 0xff) {
                throw new ParsingException("expecting `" + (char) tag +
                        "`, but found `" + (char) now.tag + "`");
            } else if (tag < 0xff) {
                throw new ParsingException("expecting `" + (char) tag +
                        "`, but found `" + now.tag + "`");
            } else if (now.tag < 0xff) {
                throw new ParsingException("expecting `" + tag +
                        "`, but found `" + (char) now.tag + "`");
            } else {
                throw new ParsingException("expecting `" + tag +
                        "`, but found `" + now.tag + "`");
            }
        }
    }
    private int integer() throws ParsingException {
        final int value;
        if (match(Tag.INTEGER)) {
            value = ((IntegerLiteral)now).value;
            now = tokenizer.tokenize();
        } else {
            throw new ParsingException("expecting `INTEGER`, but found `" + now.tag + "`");
        }
        return value;
    }
    private float float_() throws ParsingException {
        final float value;
        if (match(Tag.FLOAT)) {
            value = ((FloatLiteral)now).value;
            now = tokenizer.tokenize();
        } else {
            throw new ParsingException("expecting `FLOAT`, but found `" + now.tag + "`");
        }
        return value;
    }
    private char character() throws ParsingException {
        final char value;
        if (match(Tag.CHARACTER)) {
            value = ((CharacterLiteral)now).value;
            now = tokenizer.tokenize();
        } else {
            throw new ParsingException("expecting `CHARACTER`, but found `" + now.tag + "`");
        }
        return value;
    }
    private String identifier() throws ParsingException {
        final String id;
        if (match(Tag.ID)) {
            id = ((Word)now).lexeme;
            now = tokenizer.tokenize();
        } else {
            throw new ParsingException("expecting `ID`, but found `" + now.tag + "`");
        }
        return id;
    }
    private String string() throws ParsingException {
        final String string;
        if (match(Tag.STRING)) {
            string = ((front_end.token.StringLiteral)now).value;
            if (!strings.contains(string)) {
                strings.add(string);
            }
            now = tokenizer.tokenize();
        } else {
            throw new ParsingException("expecting `STR`, but found `" + now.tag + "`");
        }
        return string;
    }
    public ParserResult parse(final String input) throws ParsingException {
        this.tokenizer = new Tokenizer(input);
        this.now = tokenizer.tokenize();
        this.current = null;
        this.strings = new ArrayList<>();

        final List<ast.FunctionDeclare> function_declarations = new ArrayList<>();
        while (match(Tag.TYPE)) {
            final ast.FunctionDeclare function_declaration = function_declaration();
            function_declarations.add(function_declaration);
        }
        return new ParserResult(function_declarations, strings);
    }
    private ast.FunctionDeclare function_declaration() throws ParsingException {
        final SymbolList symbols = new SymbolList(current);
        current = symbols;
        type();
        final String name = identifier();
        consume('(');
        if (!match(')')) {
            variable_declaration();
            while (match(',')) {
                consume(',');
                variable_declaration();
            }
        }
        consume(')');
        final ast.Closure closure = closure();
        current = current.enclosing;
        return new ast.FunctionDeclare(name, symbols, closure);
    }
    private ast.Closure closure() throws ParsingException {
        final SymbolList symbols = new SymbolList(current);
        current = symbols;
        consume('{');
        while (match(Tag.TYPE)) {
            variable_declaration();
            consume(';');
        }
        final List<ast.Statement> statements = new ArrayList<>();
        while (!match('}')) {
            ast.Statement statement = statement();
            statements.add(statement);
        }
        consume('}');
        current = current.enclosing;
        return new ast.Closure(symbols, statements);
    }
    private void variable_declaration() throws ParsingException {
        type();
        String name = identifier();
        current.declare(name);
    }
    private ast.Statement statement() throws ParsingException {
        if (match(Tag.IF)) {
            consume(Tag.IF);
            consume('(');
            final ast.Expression condition = expression();
            consume(')');
            final ast.Closure then_closure = closure();
            return new ast.If(condition, then_closure);
        } else if (match(Tag.WHILE)) {
            consume(Tag.WHILE);
            consume('(');
            final ast.Expression condition = expression();
            consume(')');
            final ast.Closure closure = closure();
            return new ast.While(condition, closure);
        } else if (match(Tag.RET)) {
            consume(Tag.RET);
            ast.Expression expression = expression();
            consume(';');
            return new ast.Return(expression);
        } else if (match(Tag.PRINT)) {
            consume(Tag.PRINT);
            ast.Expression expression;
            if (match(Tag.STRING)) {
                final String string= string();
                expression = new ast.StringLiteral(string, strings.indexOf(string));
            } else {
                expression = expression();
            }
            consume(';');
            return new ast.Print(expression);
        } else {
            final String name = identifier();
            consume('=');
            final ast.Expression expression = expression();
            consume(';');

            final LocalVariable variable = current.get(name);
            variable.update();
            return new ast.Assign(new ast.VariableCall(variable), expression);
        }
    }
    private ast.Expression expression() throws ParsingException {
        ast.Expression left = term();
        while (true) {
            if (match('+')) {
                consume('+');
                final ast.Expression right = term();
                left =  new ast.BinaryOperator("+", left, right);
            } else if (match('-')) {
                consume('-');
                final ast.Expression right = term();
                left = new ast.BinaryOperator("-", left, right);
            } else {
                return left;
            }
        }
    }
    private ast.Expression term() throws ParsingException {
        ast.Expression left = unary();
        while (true) {
            if (match('*')) {
                consume('*');
                final ast.Expression right = unary();
                left = new ast.BinaryOperator("*", left, right);
            } else if (match('/')) {
                consume('/');
                final ast.Expression right = unary();
                left = new ast.BinaryOperator("/", left, right);
            } else {
                return left;
            }
        }
    }
    private ast.Expression unary() throws ParsingException {
        if (match('-')) {
            consume('-');
            final ast.Expression operand = factor();
            return new ast.UnaryOperator("-", operand);
        } else {
            final ast.Expression expression = factor();
            return expression;
        }
    }
    private ast.Expression factor() throws ParsingException {
        if (match(Tag.INTEGER)) {
            final int value = integer();
            return new ast.IntegerLiteral(value);
        } else if (match(Tag.FLOAT)) {
            final float value = float_();
            return new ast.FloatLiteral(value);
        } else if (match(Tag.CHARACTER)) {
            final char value = character();
            return new ast.CharacterLiteral(value);
        } else if (match(Tag.TRUE)) {
            consume(Tag.TRUE);
            return new ast.BooleanLiteral(true);
        } else if (match(Tag.FALSE)) {
            consume(Tag.TRUE);
            return new ast.BooleanLiteral(false);
        } else if (match(Tag.ID)) {
            final String name = identifier();
            if (match('(')) {
                consume('(');
                final List<ast.Expression> arguments = new ArrayList<>();
                if (!match(')')) {
                    ast.Expression expression = expression();
                    arguments.add(expression);
                    while (match(',')) {
                        consume(',');
                        expression = expression();
                        arguments.add(expression);
                    }
                }
                consume(')');
                return new ast.FunctionCall(name, arguments);
            } else {
                return new ast.VariableCall(current.get(name));
            }
        } else {
            consume('(');
            final ast.Expression expression = expression();
            consume(')');
            return expression;
        }
    }
    private void type() throws ParsingException {
        consume(Tag.TYPE);
    }
    public static String tab(int tab) {
        // TODO improve tab system
        return " ".repeat(Math.max(0, tab));
    }
}
