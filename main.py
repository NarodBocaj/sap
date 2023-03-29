import sys
from pathlib import Path
import random
import time


sys.path.append(str(Path(__file__).resolve().parent / "target" / "release"))
import libsap #my rust code



max_wins = 0

start_time = time.time()

got_warning = False


for _ in range(100):
    pysap = libsap.Game()
    playing_game = True
    wins = 0
    while playing_game:
        game_state = pysap.game_state()
        print(game_state)
        #print(f"Length of game state = {len(game_state)}")
        #print(game_state)
        if len(game_state) != 66:
            got_warning = True
            warning_mssg = f"Bad game state had len of  = {len(game_state)} and equaled {game_state}"
        game_ops = pysap.game_options()
        #choose option to do 
        reward = pysap.do_action(game_ops[random.randint(0, len(game_ops) - 1)])
        if reward == 10:
            wins += 1
            max_wins = max(max_wins, wins)
        playing_game = pysap.game_alive()
        print("Game going??", pysap.game_alive())


end_time = time.time()
print("Code execution time:", end_time - start_time, "seconds")
print(f"Max wins was {max_wins}")
if got_warning:
    print(warning_mssg)