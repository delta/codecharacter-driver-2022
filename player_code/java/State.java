import java.util.List;

public class State {
    private final List<Attacker> _attackers;
    private final List<Defender> _defenders;
    private final int _noOfCoinsLeft;
    private final int _turnNo;

    public State(List<Attacker> attackers, List<Defender> defenders,
            int noOfCoinsLeft, int turnNo) {
        _attackers = attackers;
        _defenders = defenders;
        _noOfCoinsLeft = noOfCoinsLeft;
        _turnNo = turnNo;
    }

    public List<Attacker> getAttackers() {
        return _attackers;
    }

    public List<Defender> getDefenders() {
        return _defenders;
    }

    public int getTurnNo() {
        return _turnNo;
    }

    public int getCoinsLeft() {
        return _noOfCoinsLeft;
    }
}
