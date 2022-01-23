from cga3 import *

ODD_COMPONENTS = [
    e12345,
    e1, e2, e3, e4, e5,
    e345, e245, e235, e234, e145, e135, e134, e125, e124, e123
]

EVEN_COMPONENTS = [
    1,
    e2345, e1345, e1245, e1235, e1234,
    e12, e13, e14, e15, e23, e24, e25, e34, e35, e45
]

right_even = sum((i + 1) * x for (i, x) in enumerate(EVEN_COMPONENTS))
right_odd = sum((i + 1) * x for (i, x) in enumerate(ODD_COMPONENTS))

def test_product(left_components, right):
    # to see all 256 terms, apply the multiplication component by component
    for x in left_components:
        print(x, "   ", x * right)
    
    # also print the product
    print("product", sum(left_components) * right)

print("\ntesting odd * even ===================")
test_product(ODD_COMPONENTS, right_even)
print("\ntesting odd * odd ===================")
test_product(ODD_COMPONENTS, right_odd)
print("\ntesting even * even ===================")
test_product(EVEN_COMPONENTS, right_even)
print("\ntesting even * odd ===================")
test_product(EVEN_COMPONENTS, right_odd)