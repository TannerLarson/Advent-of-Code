from enum import Enum
from pathlib import Path


def get_data():
    input_path = Path(__file__).parent.parent / "inputs/2-rock-paper-scissors.txt"

    with open(input_path, "r") as file:
        return [line.strip() for line in file]


data = get_data()


class Move(Enum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3


class Outcome(Enum):
    WIN = 6
    DRAW = 3
    LOSE = 0


MOVE_ENUM = {
    "A": Move.ROCK,
    "X": Move.ROCK,
    "B": Move.PAPER,
    "Y": Move.PAPER,
    "C": Move.SCISSORS,
    "Z": Move.SCISSORS,
}

OUTCOME_ENUM = {"X": Outcome.LOSE, "Y": Outcome.DRAW, "Z": Outcome.WIN}


def get_outcome(opponent_move: Move, your_move: Move) -> Outcome:
    if opponent_move == your_move:
        return Outcome.DRAW
    match opponent_move:
        case Move.ROCK:
            you_win = your_move == Move.PAPER
        case Move.PAPER:
            you_win = your_move == Move.SCISSORS
        case Move.SCISSORS:
            you_win = your_move == Move.ROCK
    return Outcome.WIN if you_win else Outcome.LOSE


def calculate_score(opponent_move: Move, your_move: Move):
    outcome = get_outcome(opponent_move, your_move)
    return outcome.value + your_move.value


def part_one():
    total_points = 0
    for line in data:
        if total_points > 100:
            break
        print(total_points)
        opponent_move = MOVE_ENUM[line[0]]
        your_move = MOVE_ENUM[line[2]]
        total_points += calculate_score(opponent_move, your_move)
    return total_points


def extrapolate_move(opponent_move: Move, outcome: Outcome) -> Move:
    if outcome == Outcome.DRAW:
        return opponent_move
    you_win = outcome == Outcome.WIN
    match opponent_move:
        case Move.ROCK:
            return Move.PAPER if you_win else Move.SCISSORS
        case Move.PAPER:
            return Move.SCISSORS if you_win else Move.ROCK
        case Move.SCISSORS:
            return Move.ROCK if you_win else Move.PAPER


def part_two():
    total_points = 0
    for line in data:
        opponent_move = MOVE_ENUM[line[0]]
        outcome = OUTCOME_ENUM[line[2]]
        your_move = extrapolate_move(opponent_move, outcome)
        total_points += calculate_score(opponent_move, your_move)
    return total_points


print(part_one())
print(part_two())
