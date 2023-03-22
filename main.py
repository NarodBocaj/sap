import sys
from pathlib import Path
# import ctypes

# libsap = ctypes.cdll.LoadLibrary('./target/release/libsap.so')

sys.path.append(str(Path(__file__).resolve().parent / "target" / "release"))
import libsap #my rust code

# class PySAP:
#     def __init__(self):
#         self.game = libsap.Game()

#     def game_state(self):
#         return self.game.game_state()

#     def game_options(self):
#         return self.game.game_options()

# game = PySAP()

print(libsap.hello())
# pysap = libsap.Game.__new__(libsap.Game)
# pysap.__init__()

pysap = libsap.Game()
# pysap.__init__()
# pysap.__new__

print(pysap.game_state())
print(pysap.game_options())

game_ops = pysap.game_options()

pysap.do_action(game_ops[0])
print(pysap.game_state())

# print("Printing Attributes")
# for attr in dir(pysap):
#     print(f"{attr}: {getattr(pysap, attr)}")