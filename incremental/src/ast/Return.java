package ast;

import back_end.Builder;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

import static middle_end.IRGenerator._return;
import static middle_end.IRGenerator.emit;

public class Return extends Statement {
    private final Expression expression; // TODO implement Expression
    public Return(final Expression expression) {
        this.expression = expression;
    }
    @Override
    public void generate(final int before, final int after) {
        Expression temporary = expression.reduce();
        emit("return " + temporary.toString());
        _return(temporary.toString());
    }
    @Override
    public String toAssembly() {
        return expression.toAssembly() +
                Builder.epilogue();
    }
    @Override
    public String toS(int tab) {
        tab += 8;
        return "(return " + expression.toS(tab) + ")";
    }
    @Override
    public List<middle_end.Instruction> gen() {
        final List<middle_end.Instruction> list = new ArrayList<>(expression.red());
        list.add(new middle_end.Instruction(
                middle_end.InstructionType.POP, middle_end.Register.RAX, null
        ));
        list.add(new middle_end.Instruction(
                middle_end.InstructionType.MOV, middle_end.Register.RSP, middle_end.Register.RBP
        ));
        list.add(new middle_end.Instruction(
                middle_end.InstructionType.POP, middle_end.Register.RBP, null
        ));
        list.add(new middle_end.Instruction(
                middle_end.InstructionType.RET, null, null
        ));
        return list;
    }
    @Override
    public List<ir.Code> toIR() {
        return Collections.singletonList(new ir.Return(expression.toIR()));
    }
}
