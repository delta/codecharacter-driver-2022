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

    public int getId() {
        return _id;
    }

    public int getHp() {
        return _hp;
    }

    public int getType() {
        return _type;
    }

    public Position getPosition() {
        return _position;
    }
}
