from cga3 import *

"""
Recipe: poloidal transformations of order 4

From the book Indra's Pearls by Mumford et al., a useful transformation in
the complex plane is (z - 1)/(z + 1), which cycles 1 -> 0 -> -1 -> inf -> 1
while keeping i and -i fixed. Furthermore, it swirls the upper half plane
around the point i, while swirling the lower half plane around -i. This 
transformation is of order 4, in other words, xform**4 = identity.

This transformation is used to conjugate other transforms. The key property
is it moves the origin to -1 and infinity to 1, which makes it possible to
turn simple translations, rotations, and dilations into more visually
interesting parabolic, elliptic, hyperbolic or loxodromic mobius transformations

Generalizing this to 3D takes some thought, since multivectors in general
are not commutative, and there's an extra dimension to consider. The properties
I was looking for as follows:

1. cycles +x -> 0 -> -x -> infinity -> +x
2. Fixes the unit circle in the yz plane.
3. The swirling pushes points through said unit circle in the -x direction.
    This motion makes me think of vortex rings in that the motion is poloidal
5. Is a conformal transformation. after transforming the axes, x, y, and z should
    still follow the right-hand rule

The recipe I found was this:

1. Translate in the direction opposite the poloidal rotation. In this particular
    case, that would be the +x direction.
2. Perform a reflection in plane where the fixed ring will appear. In this
    case, this is the yz plane with normal +x.
3. Perform a sphere inversion.
4. Perform a dilation with a factor of 2.
5. Translate in the direction opposite the poloidal rotation again.

To see how this works, consider the motion of a point A that starts at the origin
and a point B that starts at infinity.
1. A moves to +x, B stays at infinity
2. A flips to -x (more importantly, stays on the unit sphere), B stays at infinity
3. A stays at -x since sphere inversion fixes the unit sphere. B inverts to the origin
4. A scales to -2x, B stays at the origin
5. A moves to -x, B moves to +x

To check conformality, all of the transformations are conformal except for
steps 2 and 3 which are anti-conformal (since they involve reflections). 
These two reflection-like transformations cancel out to produce a conformal
transformation.

The code below will check the other properties.
"""

Tx = translation(e1)
S2 = dilation(2)
Vx = inversion() * reflection(e1)

# print some intermediate calculations to check my work on paper
print("Tx:", Tx)
print("S2:", S2)
print("Vx:", Vx)
print("Tx * S2:", Tx * S2, abs(Tx * S2))
print("Vx * Tx:", Vx * Tx, abs(Vx * Tx))

poloidal_x = Tx * S2 * Vx * Tx
print("poloidal_x:", poloidal_x)
print("poloidal_x ** 4 = identity?:", sandwich(poloidal_x ** 4, e1), 1)

# check that the unit sphere is transformed correctly
print("Check poloidal_x properties -------------------")
print("x -> 0:", sandwich(poloidal_x, e1))
print("0 -> -x:", sandwich(poloidal_x, 0*e1))
# this line should print a warning
print("-x -> inf:", sandwich(poloidal_x, -e1))
# using the inverse should send x to infinity, thus another set of warnings
print("inf -> x:", sandwich(~poloidal_x, e1))
print("fixes y:", sandwich(poloidal_x, e2))
print("fixes z:", sandwich(poloidal_x, e3))

# Conjecture: this easily generalizes to other directions
def poloidal4(direction):
    Td = translation(direction)
    S2 = dilation(2)
    Vd = inversion() * reflection(direction)
    return Td * S2 * Vd * Td

poloidal_y = poloidal4(e2)
# check that the unit sphere is transformed correctly
print("Check poloidal_y properties -------------------")
print("y -> 0:", sandwich(poloidal_y, e2))
print("0 -> -y:", sandwich(poloidal_y, 0*e2))
# this line should print a warning
print("-y -> inf:", sandwich(poloidal_y, -e2))
# using the inverse should send x to infinity, thus another set of warnings
print("inf -> y:", sandwich(~poloidal_y, e2))
print("fixes x:", sandwich(poloidal_y, e1))
print("fixes z:", sandwich(poloidal_y, e3))
print("poloidal_y ** 4 = identity?:", sandwich(poloidal_y ** 4, e2), 1)

# Conjecture: the versor looks like a rotor in the XP plane. Can I generate
# it directly with the exponential map?
def poloidal(mixed_plane, angle):
    return e**(-0.5 * angle * mixed_plane)

# if true, then I can make a transformation of order 8 by choosing a 45 degree
# angle
poloidal_8z = poloidal(e34, pi / 4.0)
print("poloidal(zp, 45 deg) ** 8 = identity?:", poloidal_8z ** 8)