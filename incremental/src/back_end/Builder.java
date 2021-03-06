package back_end;

import ast.FunctionDeclare;

import java.util.List;

// TODO change build system
public class Builder {
    public String build(final List<FunctionDeclare> tree) {
        final StringBuilder assembly = new StringBuilder(".intel_syntax noprefix\n");
        tree.forEach(method -> assembly.append(method.toAssembly()));
        return assembly.toString();
    }
    public static String prologue(final int locals) {
        return "  push rbp\n" +
                "  mov  rbp, rsp\n" +
                "  sub  rsp, " +
                locals +
                '\n';
    }
    public static String epilogue() {
        return "  mov  rsp, rbp\n" +
                "  pop  rbp\n" +
                "  ret\n";
    }
}
