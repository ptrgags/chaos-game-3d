## Chaos Game 3D 

## 2019, 2020 Initial Version

This project aims to take concepts from Chaos Game fractals (from 
_Fractals Everywhere_ by Michael F. Barnsley, used in programs such as
Apophysis) and extend it to 3D using point clouds.

Originally I started with basic affine transformations (translation, rotation, 
scale). But after learning about geometric algebra, I realized I could take
it further and express reflections and even sphere inversions. Furthermore,
I realized I could take concepts from _Indra's Pearls_ by Mumford, Series and 
Wright and extend them to 3D (to some extent, the formulas are not as elegant as
Möbius transformations in the complex plane. And this leads to some interesting
shapes.

The output of the program so far has been
[3D Tiles](https://github.com/CesiumGS/3d-tiles/tree/main/specification),
an open standard for streaming massive 3D datasets. Usually it's used for
geospatial applications, but I find it useful because it represents octrees
and point clouds easily. I used the `.pnts` format for encoding point clouds
and overall the tileset is structured like an octree.

## 2021-12-29 Future Direction

Looking back at this after almost a year, I realize there's a lot of things
I could have done better. When I first made this application, I was still 
new at Cesium and hadn't used CesiumJS much. Since then I've gotten much more
familiar with 3D Tiles, so I realize there's some easy tweaks I could use to
make this better.

Additionally, I have some new ideas given the recent developments
of [3D Tiles Next](https://github.com/CesiumGS/3d-tiles/tree/main/next)

>**Disclaimer** I'm a developer on the 3D Tiles team, so I've worked on
these extensions and some other CesiumJS features such as Custom Shaders.

* In the viewer application, I scale the tileset up to be the size of the world.
    I realize now that ENU matrices are easy to compute with CesiumJS' API,
    so I could use that and position fractals somewhere in the world.
* The future of 3D Tiles is glTF, so it would be better to write a `glTF` with
    `POINTS` primitives instead of `pnts` files
* I could make use of some 3D Tiles Next extensions:
  * `3DTILES_implicit_tiling` -- these tilesets are always octrees, so implicit
    tiling is a natural choice
  * `3DTILES_metadata` -- I could use this to encode some information about
    the fractal in tileset metadata. Maybe even tile metadata.
  * `EXT_mesh_features` -- with this glTF extension, I could encode metadata
    about individual points. More on this point below
* CesiumJS now has a [Custom Shaders](https://sandcastle.cesium.com/?src=Custom%20Shaders%20Models.html&label=3D%20Tiles%20Next) feature, I could
    use this to make more interesting styles and animations
* I've learned about JSON Schema, so I could make schema files for the
  parameters to help document and validate things
* I don't like that all the rendering settings are in the
    same file as the fractal parameters. For example, I might want to change
    the number of iterations for a smaller, quicker render. Consider separating
    the fractal parameters from the rendering parameters.
* I should make a gallery of cool examples and make a `gh-pages` branch. I
    don't want to take up too much storage so maybe just 2-3 of the best
    examples
* I recently learned about a different flavor of geometric algebra,
    Conformal Geometric Algebra. This might be a better representation since
    even translations and inversions can be represented directly
* I need a better mechanism to maintain the viewer folder. It would be nice
    to have a script that can put together a JSON file with all the tilesets

Some ideas for metadata:

* Iteration count - Could be used for styling, or even animating the points.
* Transformation IDs. This could be the most recent transformation (or the 
    first?) or even an array of the latest N transformations. Coloring by this
    could be useful
* Color Transformation IDs. (similar to the previous point)

## 2021-12-28 Insights About Multivectors

The past couple days I've been exploring the math of CGA to see how useful it
will be. The biggest concern is efficiency, as a naive multivector
multiplication is 1024 multiplications and 992 additions! I determined using the
3D CGA setting of [this utility on bivector.net](https://bivector.net/tools.html).
Can we avoid this?

I conjecture this is possible in this case. Some observations/hunches:

* The most common operation I need to perform is transforming points. This
    is always a sandwich product `VPV~` where `V` is a versor, `P` is a point
    and `~` indicates the reverse operation.
* To compose transformations I need to multiply versors together.
* Versors have some constraints:
  * Their reverse is equal to their inverse `V^(-1) = V~`
  * Therefore, they have length 1, as `VV~ = VV^(-1) = 1`
  * In practice, versors either have only odd blades or only even blades (more
        on this below)
* if `a, b` are multivectors with only odd blades or only even blades, then
    their product has only odd blades or only even blades. (More on this below)

With this information, I think I can get away with only representing half the
terms at a time. This means storing 16 rather than 32 terms, and the multiplication
table is at worst 16x16 = 256 which is 4 times smaller!

### 3D CGA - Real, Minkowski, and Mixed Blades

One thing I noticed when working through the math is there are two subspaces:

* "Real" blades using only `x`, `y`, and `z`
* "Minkowski" blades using only `n` (-) and `p` (+). (or `inf` and `origin`)
* "Mixed" blades using both real and Minkowski blades

Looking back at [this page from the Python clifford library](https://clifford.readthedocs.io/en/v1.0.3/ConformalGeometricAlgebra.html#Operations), this is what they mean
by "verser purely/partly/out of E0". 

Since real and minkowski blades are orthogonal, you can find plenty of cases
where multiplications commute/anticommute. Some examples:

```
(v)(np) = -nvp = npv // moving a vector 1 place introduces a negative sign
Bnp = nBp = npB // moving a bivector 1 place keeps the same sign
```

The other interesting thing is that when it comes to "blade parity", the parity
of the real vectors can be treated independently, like a tuple of 
`(real parity, minkowski parity)`. It's not very helpful for this application,
but I found it neat that this is isomorphic to `(Z2 x Z2, +)` 
(Z2 is the integers mod 2)

### Odd and Even Versors

For this application, the transformations I will concern myself with are:

* Translations (scalar + mixed bivector)
* Rotations (scalar + real bivector)
* Scales (scalar + minkowski bivector)
* Reflections (real vector)
* Inversions (minkowski vector)
* (and compositions of the above)

Translations, rotations, and scales are all `scalar + bivector` which are both
even blades. Reflections and sphere inversions are `vector` which are odd blades.

When you start composing these together, after a lot of tedious calculations,
you start to notice a pattern:

* Odd blades: vector + trivector + 5-vector
* Even blades: scalar + bivector + quadvector

Multiplication table:

|  | odd | even |
|--|-----|------|
| odd  | even | odd  |
| even | odd  | even |

If the table looks familiar, it's the same as the addition group
for odd/even integers (or integers mod 2 if odd = 1 and even = 0)

The result is always all odd blades or all even blades, never a sum of both.
This means you never reach the worst case of a 32-term multivector. At worst,
you'll have 16 terms (all odd or all even). This is a solid first step in
making the geometric product less expensive.

While investigating the above, I came across the [Cartan-Dieudonné theorem](https://en.wikipedia.org/wiki/Cartan%E2%80%93Dieudonn%C3%A9_theorem) which I think is an 
explanation for this? I don't fully understand the wording of the theorem, but
I think it's saying that isometries can always be decomposed into reflections.
And that's true here. Odd multivectors represent "reflection-like"
transformations. Even sphere inversion is like a reflection in the `p` direction.

Meanwhile, even multivectors represent "rotation-like" transformations. Or
perhaps "exponential" since they are derived from the exponential function.
* rotations are of course rotation-like
* Dilations are hyperbolic rotations in the Minkowski plane. Due to the
    cosh/sinh instead of cos/sin it causes stretching rather than rotation
* Translations are degenerate rotations in a mixed plane (between a real vector
    and the `inf` null vector). Due to the use of a null vector, things cancel
    out and just move points towards infinity.

### Implementation Details

So far I started sketching out a `Versor` class. Some features:

* I only store 16 terms + an indicator of versor parity
* for even versors, the layout is `[scalar, bivector, quadvector]`
* for odd versors, the layout is `[5-vector, trivector, vector]`
* I'm continuing to use the start/stop indices so multiplication lookup tables
    will be as small as possible
* While not strictly a versor (actually they're null vectors), points are
    always odd, so I'm representing them with this same class. I may separate
    this out in the future, not sure yet.

However, I'm still not done:

* At some point I need to bite the bullet and implement the geometric product
    table. While no calculation will use more than a quarter of the table,
    all sections of the table are reachable.
* I need to figure out how to streamline sandwich products. I think I can
    get away with only computing the terms that contribute to a vector,
    but trying to implement that without branching might get tricky.


## 2022-01-06 Chip away at CGA multiplication

Today I started working on the multiplication tables for CGA:

* I renamed `Versor` to `HalfMultivector` since I plan to use the same type for
    both versors (unit multivectors) and points (null vectors)
* I started taking the [3D CGA Cayley table from bivector.net](https://bivector.net/tools.html)
    and turning it into lookup tables. Though first I copied the table into
    Google Sheets to reorder the components to group the components by parity.
    I think at least in terms of the components there's some rotation symmetry
    in the table that can be exploited (it seems like the even * even components
    are the same as the odd * odd components?). This is not true of the sign
    though, I'm still trying to figure out the relationship here.
* I also reordered the components to better match the table. The odd vectors
    are listed backwards so components line up with their dual component
  * Even half-multivectors: `[scalar, quadvector, bivector]`
  * Odd half-multivectors: `[5-vector, vector, trivector]`
* I also swapped the order of the negative and positive unit vectors to match
    the cayley table (which puts the negative component last)

The intent of matching the table is it's easier to get correct, it's mostly
copy in the table and do regex replaces (e.g. `e123 -> XYZ`)

## 2022-01-07 Finish Tables

Today I finished adding the tables. I verified that the program now compiles
and runs, but I haven't yet verified its correctness.

Next Steps:

* Add unit tests to make sure that multiplication works correctly. The tables
    are large, so data entry errors are likely
* Design a series of simple IFSs to test that the transformations are visually
    correct.
* I want to continue thinking about the sandwich product, I think there's some
    optimizations that can be made by making use of the 
    commutivity/anticommutivity of various blades, but still working through
    the math on paper.

## 2022-01-08 Started Unit Testing

Today I started testing some of the `HalfMultivector` functionality with unit
tests and with running some of the fractals. 

The random walk (only translations) seems to be working well.

The Sierpinski tetrahedron seems to be producing the right shape, though many
points are being marked as out-of-bounds which seems peculiar. Perhaps the
handling of the homogeneous coordinate is off somehow? I'll have to investigate.