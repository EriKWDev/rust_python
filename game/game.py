from collections import namedtuple

DrawRectangle = namedtuple("DrawRectangle", ["color", "position"])
SetClearColor = namedtuple("SetClearColor", ["color"])


class GameState:
    potato = 0


def game_init() -> GameState:
    result = GameState()
    result.potato = 10

    return result


def game_update(state: GameState):
    print("[ PY ] Updating the game!")
    state.potato += 1


def game_render(state: GameState, queue: list):
    queue.append(SetClearColor((255, 255, 255, 255)))
    print("[ PY ] This line was printed from game_render in python!")

    #
    # NOTE: ~3300 is average amount of renderer commands in my normal
    #       rust_simpler game
    #
    for i in range(3300):
        t = float(i)
        queue.append(DrawRectangle((255, 255, 255, 255), (t, t, 0.0)))
