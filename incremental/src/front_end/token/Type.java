package front_end.token;

// TODO implement type system
public class Type extends Word {
    private int width;
    public Type(String lexeme, int tag, int width) {
        super(lexeme, tag);
        this.width = width;
    }
    public static final Type
        Integer    = new Type("Integer",   Tag.TYPE, 4),
        Float      = new Type("Float",     Tag.TYPE, 4),
        Character  = new Type("Character", Tag.TYPE, 1),
        Boolean    = new Type("Boolean",   Tag.TYPE, 1);
    public static boolean numeric(Type type) {
        return type == Type.Character || type == Type.Float || type == Type.Integer;
    }
    public static Type max(Type type1, Type type2) throws Exception {
        if (!numeric(type1) || !numeric(type2)) throw new Exception("");
        else if (type1 == Type.Float || type2 == Type.Float) return Type.Float;
        else if (type1 == Type.Integer || type2 == Type.Integer) return Type.Integer;
        else return Type.Character;
    }
}