import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;

public class Main {

    private static final Scanner in = new Scanner(System.in);

    private static State nextState(int currentTurnNo) {
        int noOfActiveAttackers = in.nextInt();
        List<Attacker> attackers = new ArrayList<>();
        for (int i = 0; i < noOfActiveAttackers; i++) {
            attackers.add(new Attacker(in.nextInt(), in.nextInt(), in.nextInt(), new Position(in.nextInt(), in.nextInt())));
        }

        int noOfActiveDefenders = in.nextInt();
        List<Defender> defenders = new ArrayList<>();
        for (int i = 0; i < noOfActiveDefenders; i++) {
            defenders.add(new Defender(in.nextInt(), in.nextInt(), in.nextInt(), new Position(in.nextInt(), in.nextInt())));
        }

        int coinsLeft = in.nextInt();

        return new State(attackers, defenders, coinsLeft, currentTurnNo + 1);
    }

    private static GameMap getInitialMap() {
        Constants.MAP_NO_OF_ROWS = in.nextInt();
        Constants.MAP_NO_OF_COLS = in.nextInt();

        int grid[][] = new int[Constants.MAP_NO_OF_ROWS][Constants.MAP_NO_OF_COLS];
        for (int y = 0; y < Constants.MAP_NO_OF_ROWS; y++) {
            for (int x = 0; x < Constants.MAP_NO_OF_COLS; x++) {
                grid[x][y] = in.nextInt();
            }
        }

        return new GameMap(grid);
    }

    private static void output(State state, Game game) {
        
        String log = game.getLog();

        System.err.println("TURN " + state.getTurnNo());
        System.err.println(log);
        System.err.println("ENDLOG");

        Map<Integer, Position> spawnPositions = game.getSpawnPositions();

        System.out.println(spawnPositions.size());
        for (Map.Entry<Integer, Position> entry : spawnPositions.entrySet()) {
            System.out.println(entry.getKey() + " " + entry.getValue().getX() + " " + entry.getValue().getY());
        }

        Map<Integer, Integer> playerSetTargets = game.getPlayerSetTargets();
        System.out.println(playerSetTargets.size());
        for (Map.Entry<Integer, Integer> entry : playerSetTargets.entrySet()) {
            System.out.println(entry.getKey() + " " + entry.getValue());
        }
    }

    public static void main(String[] args) {
        Constants.NO_OF_TURNS = in.nextInt();
        Constants.MAX_NO_OF_COINS = in.nextInt();

        Constants.NO_OF_ATTACKER_TYPES = in.nextInt();
        Constants.ATTACKER_TYPE_ATTRIBUTES = new HashMap<Integer,Attributes>();
        for (int i = 1; i <= Constants.NO_OF_ATTACKER_TYPES; i++) {
            Constants.ATTACKER_TYPE_ATTRIBUTES.put(i, new Attributes(in.nextInt(), in.nextInt(), in.nextInt(), in.nextInt(), in.nextInt()));
        }

        Constants.NO_OF_DEFENDER_TYPES = in.nextInt();
        Constants.DEFENDER_TYPE_ATTRIBUTES = new HashMap<Integer,Attributes>();
        for (int i = 1; i <= Constants.NO_OF_DEFENDER_TYPES; i++) {
            Constants.DEFENDER_TYPE_ATTRIBUTES.put(i, new Attributes(in.nextInt(), in.nextInt(), in.nextInt(), in.nextInt(), in.nextInt()));
        }

        GameMap map = getInitialMap();
        List<Defender> defenders = map.spawnDefenders();

        State state = new State(new ArrayList<>(), defenders, Constants.MAX_NO_OF_COINS, 0);

        Run run = new Run();
        Game game = run.run(state);
        output(state, game);
        
        for (int i = 0; i < Constants.NO_OF_TURNS; i++) {
            state = nextState(state.getTurnNo());
            game = run.run(state);
            output(state, game);
        }

        in.close();
    }
}
