package ast;

public interface Node {
    // translate AST into Assembly
    String toAssembly();
    // display AST in S-Expr
    String toS(int tab);
}
