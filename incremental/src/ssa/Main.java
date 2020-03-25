package ssa;

import ir.LocalLabel;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public final class Main {
    // ssa manager
    private static Map<String, BasicBlock> blocks = new HashMap<>();
    public static List<BasicBlock> toSSA(ir.Code[] ir_in) {
        BasicBlock cur;
        List<BasicBlock> tmp = new ArrayList<>();
        // divide into blocks
        cur = new BasicBlock("entry");
        for (ir.Code code : ir_in) {
            if (code instanceof ir.LocalLabel) {
                if (!cur.isEmpty()) {
                    blocks.put(cur.name(), cur);
                    tmp.add(cur);
                }
                cur = new BasicBlock(((LocalLabel) code).name());
            } else {
                cur.instr((ir.Instruction) code);
            }
        }
        if (!cur.isEmpty()) {
            blocks.put(cur.name(), cur);
            tmp.add(cur);
        }
        // link blocks
        for (int i = 0; i < tmp.size(); i++) {
            cur = tmp.get(i);
            if (i < tmp.size() - 1) {
                cur.dst(blocks.get(tmp.get(i + 1).toString()));
            }
            ir.Instruction last = cur.last();
            if (last instanceof ir.Branch) {
                cur.dst(blocks.get(((ir.Branch) last).dst()));
            }
        }
        return tmp;
    }
    public static void main(String[] args) {
        ir.Code[] ir_in = {
                new ir.ThreeAddress(new ir.Register(1), ir.Register.ZERO, "+", new ir.Immediate(0)),
                new ir.ThreeAddress(new ir.Register(2), ir.Register.ZERO, "+", new ir.Immediate(0)),
                new ir.LocalLabel(3),
                new ir.ThreeAddress(new ir.Register(4), new ir.Register(1), ">", new ir.Immediate(5)),
                new ir.Branch(6, new ir.Register(4)),
                new ir.LocalLabel(5), // TODO
                new ir.ThreeAddress(new ir.Register(2), new ir.Register(2), "+", new ir.Register(1)),
                new ir.ThreeAddress(new ir.Register(1), new ir.Register(1), "+", new ir.Immediate(1)),
                new ir.Branch(3),
                new ir.LocalLabel(6),
                new ir.Return(new ir.Register(2))
        };
        List<BasicBlock> ssa_out = toSSA(ir_in);
        ssa_out.forEach(BasicBlock::print);
    }
}
//    public GlobalLabel(final int id) {
//        this.name = "" + id;
//    }