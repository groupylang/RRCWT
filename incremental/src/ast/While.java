package ast;

import ir.Immediate;
import middle_end.IRBuilder;

import java.util.List;

import static middle_end.IRGenerator.*;

public class While extends Statement {
    private final Expression condition;
    private final Closure closure;
    private static int count = 0;
    public While(final Expression condition, final Closure closure) {
        this.condition = condition;
        this.closure = closure;
    }
    @Override
    public void generate(final int before, int after) {
        this.after = after;
        final int label = new_label();
        condition.jumping(0, after);
        emit_label(label);
        closure.generate(label, before);
        emit("goto L" + before);
        _goto(before);
    }
    @Override
    public String toAssembly() {
        final String assembly = ".Lwhilebegin" +
                count +
                ":\n" +
                condition.toAssembly() +
                "  cmp  rax, 0\n" +
                "  je   .Lend" +
                count +
                '\n' +
                closure.toAssembly() +
                "  jmp  .Lwhilebegin" +
                count +
                '\n' +
                ".Lwhileend" +
                count +
                ":\n";
        count++;
        return assembly;
    }
    public String toS(int tab) {
        final StringBuilder builder = new StringBuilder();
        tab += 7;
        final String s = condition.toS(tab);
        tab += 1 + s.length();
        builder.append("(while ")
                .append(s)
                .append(' ')
                .append(closure.toS(tab))
                .append(')');
        return builder.toString();
    }
    @Override
    public List<middle_end.Instruction> gen() {
        return null;
    }

    @Override
    public void toIR() {
        int begin = IRBuilder.tmp();
        int end = IRBuilder.tmp();
        IRBuilder.add(new ir.LocalLabel(begin));
        IRBuilder.add(new ir.Branch("iffalse", condition.toIR(), end));
        closure.toIR();
        IRBuilder.add(new ir.Branch("iffalse", new Immediate(0), begin));
        IRBuilder.add(new ir.LocalLabel(end));
    }
}
