import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Game {
    private final Map<Integer, Integer> _playerSetTargets;
    private final Map<Integer, Position> _spawnPositions;
    private final Set<Position> _alreadySpawnedPositions;
    private final StringBuilder _logr;

    public Game() {
        _playerSetTargets = new HashMap<>();
        _spawnPositions = new HashMap<>();
        _alreadySpawnedPositions = new HashSet<>();
        _logr = new StringBuilder();
    }

    public void spawnAttacker(int id, Position pos) {
        _spawnPositions.put(id, pos);
        _alreadySpawnedPositions.add(pos);
    }

    public Map<Integer, Position> getSpawnPositions() {
        return _spawnPositions;
    }

    public Map<Integer, Integer> getPlayerSetTargets() {
        return _playerSetTargets;
    }

    public boolean alreadySpawnedAtPosition(Position pos) {
        return _alreadySpawnedPositions.contains(pos);
    }

    public void setTarget(int attackerId, int defenderId) {
        _playerSetTargets.put(attackerId, defenderId);
    }

    public void setTarget(Attacker attacker, Defender defender) {
        setTarget(attacker.getId(), defender.getId());
    }

    public void log(String s) {
        _logr.append(s + "\n");
    }

    public String getLog() {
        return _logr.toString();
    }

    public void clearLog() {
        _logr.setLength(0);
    }
}
