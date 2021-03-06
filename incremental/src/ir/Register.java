package ir;

import java.util.HashMap;
import java.util.Map;

public class Register implements Operand {
    private final String name;
    private static final Map<String, Integer> id_map = new HashMap<>(); // map of register id
    private static int counter = 0;
    public Register(final int id) {
        this.name = "$" + id;
        if (!id_map.containsKey(name)) {
            id_map.put(name, counter++);
        }
    }
    public Register(final String name) {
        this.name = name;
        if (!id_map.containsKey(name)) {
            id_map.put(name, counter++);
        }
    }
    @Override
    public String toString() {
        return name;
    }
    @Override
    public String build() { // TODO review
        return "  mov  rax, " +
                name +
                "\n";
    }
    @Override
    public int toWC() {
        return id_map.get(name);
    }
    public static int registers_size() {
        return counter;
    }
    @Override
    public String toAssembly() {
        return "";
    }
    @Override
    public void print() {
        System.out.print(name);
    }
    public static final Register
            ZERO = new Register(0),
            TRUE  = new Register("true");
}
