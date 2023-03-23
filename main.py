import sys
from pathlib import Path
import random

sys.path.append(str(Path(__file__).resolve().parent / "target" / "release"))
import libsap #my rust code


pysap = libsap.Game()

playing_game = True

while playing_game:
    game_state = pysap.game_state()
    game_ops = pysap.game_options()
    #choose option to do 
    reward = pysap.do_action(game_ops[random.randint(0, len(game_ops) - 1)])
    playing_game = pysap.game_alive()
    print("Game going??", pysap.game_alive())