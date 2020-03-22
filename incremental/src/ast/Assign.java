package ast;

import middle_end.IRBuilder;

import java.util.ArrayList;
import java.util.List;

import static front_end.RecursiveDescentParser.tab;
import static middle_end.IRGenerator.emit;
import static middle_end.IRGenerator.three_address;

public class Assign extends Statement {
    private final VariableCall variable;
    private final Expression expression;
    public Assign(final VariableCall variable, final Expression expression) {
        this.variable = variable;
        this.expression = expression;
    }
    @Override
    public void generate(final int before, final int after) {
        emit(variable.toString()  + " = " + expression.generate().toString());
        three_address(variable.toString(), expression.generate().toString());
    }
    @Override
    public String toAssembly() {
        return variable.toAssembly() +
                expression.toAssembly() +
                "  pop  rdi\n" +
                "  pop  rax\n" +
                "  mov  [rax], rdi\n";
    }
    @Override
    public String toS(int tab) {
        tab += 8;
        final StringBuilder builder = new StringBuilder();
        builder.append("(assign ")
                .append(variable.toS(tab));
        final String s = expression.toS(tab);
        if (s.length() > 4) {
            builder.append('\n')
                    .append(tab(tab));
        } else {
            builder.append(' ');
        }
        builder.append(s)
                .append(')');
        return  builder.toString();
    }
    @Override
    public List<middle_end.Instruction> gen() {
        final List<middle_end.Instruction> list = new ArrayList<>(expression.red());
        list.add(new middle_end.Instruction(
                middle_end.InstructionType.POP, new middle_end.Register(variable.toString()), null
        ));
        return list;
    }
    @Override
    public void toIR() {
        ir.Operand o = expression.toIR();
        IRBuilder.add(new ir.ThreeAddress(
            new ir.Register(variable.toString()),
            new ir.Immediate(0),
            "+",
            o
        ));
    }
}
