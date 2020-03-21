package middle_end;

public class Jump extends Instruction {
    private final Label label;
    private static final Register[] registers = {
            Register.RDI,
            Register.RSI,
            Register.RDX,
            Register.RCX,
            Register.R8,
            Register.R9
    };
    Jump(final InstructionType type, final Operand left, final Operand right, final Label label) {
        super(type, left, right);
        this.label = label;
    }
    @Override
    void toAssembly() {
        switch (type) {
            case CALL:
                if (left != null) {
                    AssemblyBuilder.append("  mov  " + registers[0].toString() + ", " + left.toString() + "\n");
                    if (right != null) {
                        AssemblyBuilder.append("  mov  " + registers[1].toString() + ", " + right.toString() + "\n");
                    }
                }
                AssemblyBuilder.append("  call " + label.toString() + "\n");
                break;
        }
    }
}
