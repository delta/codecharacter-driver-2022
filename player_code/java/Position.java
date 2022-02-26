public class Position {
    private final int _x;
    private final int _y;

    public Position(int x, int y) {
        _x = x;
        _y = y;
    }

    public int getX() {
        return _x;
    }

    public int getY() {
        return _y;
    }

    public double distanceTo(Position other) {
        return Math.sqrt(Math.pow(other.getX() - _x, 2) + Math.pow(other.getY() - _y, 2));
    }
}
