from cga3 import *

print("R(1, 0, 0, pi/2):", rotation(e23, pi/2))
print("R(0, 1, 0, pi/2):", rotation(-e13, pi/2))
print("R(0, 0, 1, pi/2):", rotation(e12, pi/2))

print("Rxy(pi/2) >>> x:", sandwich(rotation(e12, pi/2), e1))

# let's do the rotation that cycles x -> y -> z -> x
# this is a rotation around the vector 1/sqrt(3)(x + y + z)

axis = e1 + e2 + e3
axis /= abs(axis)

# Dual in the vector space: (aX + bY + cZ)XYZ = aYZ - bXZ + cXY
plane = axis * e123

# check if I have the direction correct, x -> y -> z -> x
cycle_axes = rotation(plane, 2 * pi / 3)
print("x goes to y or z?", sandwich(cycle_axes, e1))

# ah I have it backwards then... not to worry, just take the inverse :D
print("R(1, 1, 1, 2pi/3):", cycle_axes)
print("cycle(x):", sandwich(cycle_axes, e1))
print("cycle(y):", sandwich(cycle_axes, e2))
print("cycle(z):", sandwich(cycle_axes, e3))
print("cycle(0):", sandwich(cycle_axes, 0*e1))