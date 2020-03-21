package ast;

import java.util.List;

import static middle_end.IRGenerator.emit_label;
import static middle_end.IRGenerator.new_label;

public class Sequence extends Statement {
    private final Statement current;
    Statement next;
    Sequence(final Statement current) {
        this.current = current;
        this.next = null;
    }

    @Override
    public void generate(int before, int after) {
        if (current == null) {
            next.generate(before, after);
        } else if (next == null) {
            current.generate(before, after);
        } else {
            final int label = new_label();
            current.generate(before, label);
            emit_label(label);
            next.generate(label, after);
        }
    }

    @Override
    public String toAssembly() {
        return "";
    }
    @Override
    public String toS(int tab) {
        return "";
    }
    @Override
    public List<middle_end.Instruction> gen() {
        return null;
    }
}
