package middle_end;

public class Instruction {
    final InstructionType type;
    final Operand left;
    final Operand right;
    public Instruction(final InstructionType type, final Operand left, final Operand right) {
        this.type = type;
        this.left = left;
        this.right = right;
    }
    void toAssembly() {
        switch (type) {
            case ADD:
                AssemblyBuilder.append("  add  " + left.toString() + ", " + right.toString() + "\n");
                break;
            case SUB:
                AssemblyBuilder.append("  sub  " + left.toString() + ", " + right.toString() + "\n");
                break;
            case MUL:
                AssemblyBuilder.append("  imul  " + left.toString() + ", " + right.toString() + "\n");
                break;
            case DIV:
                AssemblyBuilder.append("  cqo\n  idiv  " + left.toString() + ", " + right.toString() + "\n");
                break;
            case PUSH:
                AssemblyBuilder.append("  push " + left.toString() + "\n");
                break;
            case POP:
                // register or memory
                AssemblyBuilder.append("  pop  " + left.toString() + "\n");
                break;
            case MOV:
                AssemblyBuilder.append("  mov  " + left.toString() + ", " +right.toString() + "\n");
                break;
            case RET:
                AssemblyBuilder.append("  ret\n");
                break;
            case OUT:
                AssemblyBuilder.append("  out  " + left.toString() + "\n");
                break;
            case NOP:
                AssemblyBuilder.append("  nop\n");
                break;
        }
    }
    boolean isNecessary() {
        switch (type) {
            case NOP:
                return false;
            case ADD:
                if (left instanceof Immediate && ((Immediate)left).is_zero() ||
                        right instanceof Immediate && ((Immediate)right).is_zero()) {
                    return false;
                }
            case SUB:
                if (right instanceof Immediate && ((Immediate)right).is_zero()) {
                    return false;
                }
            case MUL:
                if ((left instanceof Immediate && ((Immediate)left).is_one()) ||
                        right instanceof Immediate && ((Immediate)right).is_one()) {
                    return false;
                }
            case DIV:
                if (right instanceof Immediate && ((Immediate)right).is_one()) {
                    return false;
                }
        }
        return true;
    }
}
