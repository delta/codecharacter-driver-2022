public class Run {
    public Game run(State state) {
        Game game = new Game();
        for (var spawnPosition: Helpers.getAllValidSpawnPositions()) {
            game.spawnAttacker(1, spawnPosition);
        }
        return game;
    }
}
