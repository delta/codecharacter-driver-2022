from dataclasses import dataclass
import sys


@dataclass(eq=True, frozen=True, order=True)
class Position:
    x: float
    y: float

    def distance_to(self, other: "Position") -> float:
        return ((self.x - other.x) ** 2 + (self.y - other.y) ** 2) ** 0.5


@dataclass(frozen=True)
class ActorType:
    hp: int
    range: int
    attack_power: int
    price: int


@dataclass(frozen=True)
class AttackerType(ActorType):
    speed: int


@dataclass(frozen=True)
class DefenderType(ActorType):
    pass


@dataclass(frozen=True)
class Attacker:
    id: int
    hp: int
    type: AttackerType
    position: Position


@dataclass(frozen=True)
class Defender:
    id: int
    hp: int
    type: DefenderType
    position: Position


@dataclass(frozen=True)
class State:
    attackers: list[Attacker]
    defenders: list[Defender]
    no_of_coins_left: int
    turn_no: int


class Game:
    def __init__(self):
        self._log = ""
        self.player_set_targets: dict[int, int] = {}
        self.spawn_positions: list[tuple[int, Position]] = []
        self.already_spawned_positions: set[Position] = set()

    def spawn_attacker(self, id: int, position: Position):
        self.spawn_positions.append((id, position))
        self.already_spawned_positions.add(position)

    def is_already_spawned_at_position(self, position: Position):
        return position in self.already_spawned_positions

    def set_target(self, attacker_id: int, defender_id: int):
        self.player_set_targets[attacker_id] = defender_id

    def log(self, line: str):
        self._log += line + "\n"

    def get_log(self):
        return self._log

    def clear_log(self):
        self._log = ""


class Constants:
    NO_OF_TURNS: int
    MAX_NO_OF_COINS: int
    NO_OF_ATTACKER_TYPES: int
    NO_OF_DEFENDER_TYPES: int
    ATTACKER_TYPE_ATTRIBUTES: dict[int, AttackerType]
    DEFENDER_TYPE_ATTRIBUTES: dict[int, DefenderType]
    MAP_NO_OF_ROWS: int
    MAP_NO_OF_COLS: int

    @classmethod
    def initialize(cls):
        cls.NO_OF_TURNS, cls.MAX_NO_OF_COINS = map(int, input().split())
        cls.NO_OF_ATTACKER_TYPES = int(input())
        cls.ATTACKER_TYPE_ATTRIBUTES = {}
        for i in range(1, cls.NO_OF_ATTACKER_TYPES + 1):
            hp, a_range, attack_power, speed, price = map(int, input().split())
            cls.ATTACKER_TYPE_ATTRIBUTES[i] = AttackerType(
                hp, a_range, attack_power, price, speed
            )

        cls.NO_OF_DEFENDER_TYPES = int(input())
        cls.DEFENDER_TYPE_ATTRIBUTES = {}
        for i in range(1, cls.NO_OF_DEFENDER_TYPES + 1):
            hp, d_range, attack_power, _, price = map(int, input().split())
            cls.DEFENDER_TYPE_ATTRIBUTES[i] = DefenderType(
                hp, d_range, attack_power, price
            )


class Map:
    map: list[list[int]]

    @classmethod
    def initialize(cls):
        Constants.MAP_NO_OF_ROWS, Constants.MAP_NO_OF_COLS = map(int, sys.stdin.readline().split())
        cls.map = []
        for _ in range(Constants.MAP_NO_OF_ROWS):
            cls.map.append(list(map(int, sys.stdin.readline().split())))

    @staticmethod
    def spawn_defenders():
        defenders = []
        id = 0
        for x in range(Constants.MAP_NO_OF_COLS):
            for y in range(Constants.MAP_NO_OF_ROWS):
                if Map.map[y][x] > 0:
                    attributes = Constants.DEFENDER_TYPE_ATTRIBUTES[Map.map[y][x]]
                    defenders.append(
                        Defender(id, attributes.hp, attributes, Position(x, y))
                    )
                    id += 1
        return defenders


def is_valid_spawn_position(pos: Position) -> bool:
    x, y = pos.x, pos.y
    if x < 0 or y < 0 or x >= Constants.MAP_NO_OF_ROWS or y >= Constants.MAP_NO_OF_COLS:
        return False
    return (
        x == 0
        or y == 0
        or x == Constants.MAP_NO_OF_ROWS - 1
        or y == Constants.MAP_NO_OF_COLS - 1
    )


def get_all_valid_spawn_positions() -> list[Position]:
    positions: set[Position] = set()
    for y in range(Constants.MAP_NO_OF_ROWS):
        positions.add(Position(0, y))
        positions.add(Position(Constants.MAP_NO_OF_COLS - 1, y))
    for x in range(Constants.MAP_NO_OF_COLS):
        positions.add(Position(x, 0))
        positions.add(Position(x, Constants.MAP_NO_OF_ROWS - 1))
    return list(positions)
