package ast;

import middle_end.IRBuilder;

import java.util.ArrayList;
import java.util.List;

import static middle_end.IRGenerator._print;
import static middle_end.IRGenerator.emit;

public class Print extends Statement {
    private final Expression expression;
    public Print(final Expression expression) {
        this.expression = expression;
    }
    @Override
    public void generate(final int before, final int after) {
        Expression temporary = expression.reduce();
        emit("print " + temporary.toString());
        _print(temporary.toString());
    }
    @Override
    public String toAssembly() {
        return expression.toAssembly() + "  out  rax\n";
    }
    @Override
    public String toS(int tab) {
        tab += 7;
        return "(print " + expression.toS(tab) + ")";
    }
    @Override
    public List<middle_end.Instruction> gen() {
        final List<middle_end.Instruction> list = new ArrayList<>(expression.red());
        list.add(new middle_end.Instruction(
                middle_end.InstructionType.POP, middle_end.Register.RAX, null
        ));
        list.add(new middle_end.Instruction(
                middle_end.InstructionType.OUT, middle_end.Register.RAX, null
        ));
        return list;
    }
    @Override
    public void toIR() {
        IRBuilder.add(new ir.Print(expression.toIR()));
    }
}
