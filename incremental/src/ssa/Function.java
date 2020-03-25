package ssa;

import java.util.List;

public final class Function {
    private final String name;
    private List<BasicBlock> blocks;
    public Function(final String name, final List<BasicBlock> blocks) {
        this.name = name;
        this.blocks = blocks;
    }
    public void print() {
        System.out.println("void " + name + "() {");
        blocks.forEach(BasicBlock::print);
        System.out.println("}");
    }
    public Dag<BasicBlock> toDag() {
        Dag<BasicBlock> dag = new Dag<>();
        dag.addVertex(blocks.get(0));
        for (int i = 0; i < blocks.size() - 1; i++) {
            dag.addEdge(blocks.get(i), blocks.get(i + 1));
        }
        for (BasicBlock block: blocks) {
            block.dst().ifPresent(next -> dag.addEdge(block,
                    blocks.stream()
                            .filter(b -> b.toString().equals(next))
                            .findFirst()
                            .orElseThrow()
            ));
        }
        return dag;
    }
}
