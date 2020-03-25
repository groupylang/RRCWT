package ssa;

import java.nio.ByteBuffer;
import java.security.NoSuchAlgorithmException;
import java.security.MessageDigest;
import java.util.*;

public final class BasicBlock {
    private final String name;
    private final List<ir.Instruction> instructions;
    private final List<BasicBlock> destinations;
    public BasicBlock(final String name) {
        this.name = name;
        this.instructions = new ArrayList<>();
        this.destinations = new ArrayList<>();
    }
    public void instr(final ir.Instruction instr) {
        this.instructions.add(instr);
    }
    public void dst(final BasicBlock dst) {
        this.destinations.add(dst);
    }
    @Override
    public String toString() {
        return name;
    }
    public Dag<Node> toDag() {
        Set<String> registers = new HashSet<>();
        Map<String, Integer> id_table = new HashMap<>();
        instructions.stream()
                .map(ir.Instruction::registers)
                .forEach(registers::addAll);
        registers.stream()
                .filter(reg -> !id_table.containsKey(reg))
                .forEach(reg -> {
                    id_table.put(reg, hash(reg));
                });

        System.out.println(id_table);
        return null; // TODO
    }
    public void print() {
        System.out.println(name + ":");
        instructions.forEach(ir.Instruction::print);
        System.out.println(destinations);
        toDag();
    }
    public Optional<String> dst() {
        ir.Instruction last = instructions.get(instructions.size() - 1);
        if (last instanceof ir.Branch) {
            return Optional.of(((ir.Branch) last).dst());
        }
        return Optional.empty();
    }
    public ir.Instruction last() {
        return instructions.get(instructions.size() - 1);
    }
    public String name() {
        return name;
    }
    public List<ir.Instruction> instructions() {
        return instructions;
    }
    public boolean isEmpty() {
        return instructions.isEmpty();
    }
    private static int hash(String str) {
        try {
            return ByteBuffer.wrap(MessageDigest.getInstance("MD5").digest(str.getBytes())).getInt();
        } catch (NoSuchAlgorithmException e) {
            e.printStackTrace();
        }
        return -1;
    }
}
