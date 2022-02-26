public class Actor {
    private final int _id;
    private final int _hp;
    private final int _type;
    private final Position _position;
    
    public Actor(int id, int hp, int type, Position pos) {
        _id = id;
        _hp = hp;
        _type = type;
        _position = pos;
    }

    public int get_id() {
        return _id;
    }

    public int get_hp() {
        return _hp;
    }

    public int get_type() {
        return _type;
    }

    public Position get_position() {
        return _position;
    }
}
