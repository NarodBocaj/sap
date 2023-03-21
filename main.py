import sys
from pathlib import Path
# import ctypes

# libsap = ctypes.cdll.LoadLibrary('./target/release/libsap.so')

sys.path.append(str(Path(__file__).resolve().parent / "target" / "release"))
import libsap #my rust code

# class PySAP:
#     def __init__(self):
#         self.game = libsap.Game.new()

#     def game_state(self):
#         return self.game.game_state()

#     def game_options(self):
#         return self.game.game_options()

# game = PySAP()

print(libsap.hello())