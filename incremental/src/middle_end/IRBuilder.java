package middle_end;

import java.util.List;
import java.util.ArrayList;

public class IRBuilder {
  private static List<ir.Code> ir = new ArrayList<>();
  private static int tmp_count = 0;
  public static void clear() { ir.clear(); }
  public static void add(ir.Code code) {
    ir.add(code);
  }
  public static List<ir.Code> build() {
    return List.copyOf(ir);
  }
  public static int tmp() { return tmp_count++; }
}