from numpy import pi, e
from numpy.lib.scimath import log
from clifford import Cl, conformalize

G3, blades_g3 = Cl(3)
G3c, blades_g3c, stuff = conformalize(G3)
locals().update(blades_g3c)
locals().update(stuff)

def sandwich(versor, vector):
    return down(versor * up(vector) * ~versor)

def inversion():
    return ep

def involution():
    return E0

def dilation(k):
    return e**(-log(k)/2.0 * E0)

def translation(d):
    return e**(0.5 * einf * d)

def reflection(m):
    return m

def rotation(plane, theta):
    return e**(-0.5 * theta * plane)

def sandwich(versor, point_ga):
    return down(versor * up(point_ga) * ~versor)
