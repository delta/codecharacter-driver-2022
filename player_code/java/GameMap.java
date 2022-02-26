import java.util.ArrayList;
import java.util.List;

public class GameMap {
    public static int noOfRows;
    public static int noOfCols;
    private int[][] _grid;

    public GameMap(int[][] grid) {
        _grid = grid;
        GameMap.noOfRows = grid.length;
        GameMap.noOfCols = grid[0].length;
    }

    public List<Defender> spawnDefenders() {
        int id = 0;
        List<Defender> defenders = new ArrayList<>();
        for (int y = 0; y < GameMap.noOfRows; y++) {
            for (int x = 0; x < GameMap.noOfCols; x++) {
                if (_grid[x][y] > 0) {
                    Attributes defenderAttributes = Constants.DEFENDER_TYPE_ATTRIBUTES.get(_grid[x][y]);
                    defenders.add(new Defender(id++, defenderAttributes.hp, _grid[x][y], new Position(x, y)));
                }
            }
        }
        return defenders;
    }
}
