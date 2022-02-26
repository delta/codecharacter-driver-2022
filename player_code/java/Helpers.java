import java.util.HashSet;
import java.util.Set;

public class Helpers {
    public static Set<Position> getAllValidSpawnPositions() {
        Set<Position> allValidSpawnPositions = new HashSet<Position>();

        for (int y = 0; y < Constants.MAP_NO_OF_ROWS; y++) {
            allValidSpawnPositions.add(new Position(0, y));
            allValidSpawnPositions.add(new Position(Constants.MAP_NO_OF_COLS - 1, y));
        }

        for (int x = 0; x < Constants.MAP_NO_OF_COLS; x++) {
            allValidSpawnPositions.add(new Position(x, 0));
            allValidSpawnPositions.add(new Position(x, Constants.MAP_NO_OF_ROWS - 1));
        }
        return allValidSpawnPositions;
    }

    public static boolean isValidSpawnPosition(Position position) {
        int x = position.getX();
        int y = position.getY();
        if (x < 0 || y < 0 || x >= Constants.MAP_NO_OF_COLS || y >= Constants.MAP_NO_OF_ROWS) {
            return false;
        }

        return x == 0 || y == 0 || x == Constants.MAP_NO_OF_COLS - 1 || y == Constants.MAP_NO_OF_ROWS - 1;
    }
}
