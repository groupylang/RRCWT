package middle_end;


import java.util.List;

public class Module {
    private final List<Function> functions;
    public Module(final List<Function> functions) {
        this.functions = functions;
    }
    public void toAssembly() {
        AssemblyBuilder.append(".intel_syntax noprefix\n");
        for (final Function function : functions) {
            function.toAssembly();
        }
    }
}
