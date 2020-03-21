package middle_end;

public class AssemblyBuilder {
    private static final StringBuilder assembly = new StringBuilder();
    public static void clear() {
        assembly.delete(0, assembly.length());
    }
    static void append(final String message) {
        assembly.append(message);
    }
    public static String build() {
        return assembly.toString();
    }
}
