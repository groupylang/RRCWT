package ir;

import java.util.List;

public interface Instruction extends Code {
    void print();
    List<String> registers();
}
