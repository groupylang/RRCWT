package front_end;

import ast.FunctionDeclare;

import java.util.List;

public class ParserResult {
    public List<FunctionDeclare> ast;
    public String[] strings;
    ParserResult(List<FunctionDeclare> ast, List<String> strings) {
        this.ast = ast;
        this.strings = strings.toArray(new String[0]);
    }
}
