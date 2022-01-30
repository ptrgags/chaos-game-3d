from cga3 import *

invert = inversion()

for i in range(-10, 10 + 1):
    pos = e1 + i * e3
    inverted = sandwich(invert, pos)
    print(f"original: {pos}, inverted: {inverted}")