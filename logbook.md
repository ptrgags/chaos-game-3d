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

## 2022-01-02 Refactoring 3D Tiles Output

Today I started working on the 3D Tiles Next output, but I soon realized that
this requires refactoring things a bit. I pulled out a `TilesetWriter` struct
(since I will likely have variations on this in the future for implicit tiling)
and I stubbed out a `GlbWriter` class to parallel the older `PntsWriter`. I
will likely keep both around for a while since the 3D Tiles Next extensions
are still experimental and may change.

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



## 2022-01-09 Continue working on GLB output

Today I continued to work on GLB output. Along the way, I noticed I was passing
a lot of variables for each point, so I refactored out a `Point` class to store
the position, color, and metadata for each point. Overall, it works nicely,
though there are some parts where I end up copying data a lot. I'll have to
revisit this again in the future.

At this point I can compile things, but it's not yet producing valid glTF files.

## 2022-01-10 Debug GLB output

Today I started working on debugging my GLB output. I had quite a few incorrect
offsets/byte lengths to fix.

I'm still having trouble loading the results in CesiumJS. The latest error
seems to have something to do with the resource cache not being able to find
the buffer. I'll keep digging around

## 2022-01-11 glTF Validator to the Rescue

This morning I tried running the GLB output through the official
[glTF validator](https://github.khronos.org/glTF-Validator/), which revealed
some issues with alignment. I also forgot that vertex attributes need to be
aligned to 4-byte offsets. Now I'm able to load things in CesiumJS!

## 2022-01-12 Decimation

Today I implemented basic decimation to create levels of detail. I simply
take 1/4th of the points in each child tile and add them to the parent. This
is done just before the tileset is saved to disk.

Something seems a little weird with the bounding volumes, the largest ones
don't seem to be showing up. I should check the bounding volumes and recursive
logic.

## 2022-01-13 Fix y-up issue

Today I merged in `main` to pull in the decimation features. I then added
the z-up to y-up transform in the root node of each GLB so it renders properly
in CesiumJS.

I also reviewed [the PR](https://github.com/ptrgags/chaos-game-3d/pull/7) to
point out several things I think could be better and use more of 3D Tiles Next.
I think adding the `3DTILES_metadata` extension will be helpful here.

## 2022-01-16 Add tileset metadata, update parameters

The past couple days I wrapped up the 3D Tiles Next stuff and updated most of
the parameter files to include an id and name. There's a few I didn't get to,
but I'll handle those later when I go to make a script to index the files.

## 2022-01-24 Automatically populate the UI

This morning I added a script, `viewer/make_index.py` that looks through the
tileset directories and puts together an index file, `fractals.json`. This
is now used by the viewer to populate the dropdown. This way, I don't have
to edit `index.html` every time I add a new fractal.

## 2022-01-30 Wrapping up CGA branch

The past several days I've been learning more about conformal geometric
algebra and updating my implementation. The highlights:

* I tried using Python's `clifford` module to check my math. The `cga_checker/`
    directory has some scripts I used for checking my implementation
* I tested the geometric product, and after fixing a typo in the lookup
    tables, now the transformations work as expected.
* I learned some cool new things about 3D CGA, see the 
    "Poloidal and Double Rotations" section below
* Add some new parameter files
* Added a dropdown for choosing a shader
* Added some screenshots to the README
* Simplified the binary to always generate the result in `viewer/` - this is
    because the viewer assumes the tileset will be there.

### Poloidal and Double Rotations!

I was looking through my old fractal parameters from 2020 and tried to see how
I bent shapes around the unit sphere. 

I was taking the 2D Möbius transformations I had learned about in the book
_Indra's Pearls_ by David Mumford, Caroline Series and David Wright and trying
to generalize them to 3D. It was a messy chain of transformations, and
I actually used the wrong type of reflection. It should have looked something
like this:

```
M = T * S * V * H * T

where 
  T = translate(1, 0, 0) -- translate in the X direction
  H = reflect(1, 0, 0) -- reflection in plane normal to x axis
  V = inversion() -- sphere inversion
  S = dilation(2) -- 

Now compute

M' = M * A * ~M

where A is some other transformation... messy right?
```

Messy, right? in 3D (vector space) geometric algebra, the translations and
inversion means that I can't easily simplify this into a sandwich product.

However, in 3D conformal geometric algebra, the extra dimensions mean that
translations and inversions are handled more sanely. I tried multiplying this
long chain of versors together, and, much to my surprise, it simplified to the
elegant:

```
-sqrt(2)/2 - sqrt(2)/2 * e14
```

In other words, this is a _rotation_ in the e14 plane (I prefer `xp` since e1 is
the `x` direction and e4 is the "plus" direction (usually written `e+` in the
literature, but I use `p` to avoid confusion when doing calculations on paper).
I had a hunch it was like a higher dimensional rotation, but in this case, it
_literally_ is a higher-dimensional rotation. Strange, but elegant!

From our 3D perspective, this rotates points in perfect 
[vortex rings](https://en.wikipedia.org/wiki/Vortex_ring)
(but fixed in place, not moving, like
[this image in the same article](https://en.wikipedia.org/wiki/Vortex_ring#/media/File:Vortex_ring.gif))
I call it a "poloidal rotation" for lack of a better name. Poloidal refers to
the direction around a torus through the hole. [See here](https://en.wikipedia.org/wiki/Toroidal_and_poloidal_coordinates). However, it's a direct generalization of
elliptic Möbius transformations, so I suppose "elliptic rotation" would work
too... Or just "rotation" if you're 4-dimensional. 

To check my work, I wrote `cga_checker/poloidal4.py` and confirmed that this
rotation works for other angles.

From there, I also realized that you can make a _double_ rotation, by rotating
in the `xp` plane simultaneously with the `yz` plane. This choice is not
unique, any 2 orthogonal planes in the `x, y, z, and p` directions.

To try to visualize this, and try other things, I tried a few things with
ganja.js. See these examples:

* [Parabolic transformation](https://enkimute.github.io/ganja.js/examples/coffeeshop.html#RPzAmg3bS)
* [Double Rotation](https://enkimute.github.io/ganja.js/examples/coffeeshop.html#ysnayUxhH)
* [More Double rotations](https://enkimute.github.io/ganja.js/examples/coffeeshop.html#wS8uznb-d)
* [Hopf fibration (angles the same)](https://enkimute.github.io/ganja.js/examples/coffeeshop.html#wS8uznb-d)

From double rotations, I learned a bit about Hopf fibrations.
[This YouTube video](https://youtu.be/lHT9xI01sqw) was quite helpful.

### Future Steps

I'm not quite done with this project, there's a few more things I want to do:

* The initial sets look kinda messy. I'd rather use a determnistic shape. I
    want to make lines, planes, disks, and spheres with regularly spaced
    points. for the disk and sphere, I want to try a
    [Fibonacci lattice](http://extremelearning.com.au/how-to-evenly-distribute-points-on-a-sphere-more-effectively-than-the-canonical-fibonacci-lattice/)
    since they look cool!
* I might consider making more types of transformation choosers. It would be
    helpful to make it less random. Some possible ideas:
    * Don't allow backtracking (applying `A` and `A^(-1)` in a row), that's a
        waste of iterations
    * Allow a Markov chain to weight the transitions. This is essentially like
        the `xaos` parameters in [Apophysis 7x](https://sourceforge.net/projects/apophysis7x/).
    * For tilings, it would be nice to choose transformations like iterating
        over a 1-, 2- or 3-dimensional grid, that way you get nice even-looking
        results. The only tricky part is choosing the iteration bounds so it
        doesn't explode in complexity.
* Instead of scaling the tileset to the size of the earth, I should position it
    correctly in the world. 
* It would be cool to have something like an arc-ball camera. Maybe use the
    arrow keys or a gamepad?
* Once CesiumJS's implementation of 3D Tiles Next is further along, there's
    plenty of cool styling opportunities

## 2022-02-05 More Cluster Types

The past couple of days I've been working on adding a few more cluster types:

* evenly spaced lines, circles, quads, boxes
* Fibonacci lattice disks and spheres. Formulas were found in [this article](http://extremelearning.com.au/how-to-evenly-distribute-points-on-a-sphere-more-effectively-than-the-canonical-fibonacci-lattice/)
* A list of specific points.

By being more precise in the initial clusters, the final results will look
sharper.

The next thing I want to do is add a cluster type of "many" that lets me
list multiple starting shapes. Sometimes it helps to start with multiple
shapes to get a better idea of the overall shape

## 2022-02-07 Finishing Touches

For this cluster branch, I added a couple of finishing touches:

1. added the `ManyClusters` struct so I can list multiple cluster shapes in
    a single fractal/tiling
2. renaming `initial_set` to `cluster` in the JSON

I did notice a couple things I'll need to investigate when I do a self-review:

1. When a point gets mapped to infinity (when a point at (0, 0, 0) gets 
inverted), I need to handle that carefully. I need to see if the multivector
representation handles this cleanly or if it leads to divide by zero errors.
2. For some reason `hopf_fibration` gives me an empty tileset. I'll have to
    debug that one.

## 2022-02-08 No Backtracking Chooser

When exploring tilings where the transforms come in pairs of inverses
(think: up/down & left/right), iterations get wasted by applying transformations
that cancel (e.g. up then down). A simple way to do this is to disallow
inverses. If I store the transformations as `[A, A^(-1), B, B^(-1), ...]`, then
all that needs to be done is to keep track of the last transformation that was
applied and forbid the adjacent index (+1 if the last index was even, -1 
otherwise)

### Next Steps:

* I need to change how the `chaos_sets` algorithm to iterate each copy of the
    initial set separately, and call `chooser.reset()` afterwards. Right now,
    if there is only 1 pair of inverses, everything winds up going down the
    same path, which defeats the purpose.
* I also want to try a more general Markov chain chooser, similar to the `xaos`
    parameters in Apophysis. The parameters will be a matrix of weights that
    will be turned into cumulative probability distributions for easy random
    generation.

## 2022-02-10 Markov Chooser

Today I wrapped up my Markov chain chooser. I realized it would also be helpful
to have a set of `initial_weights` so I can control the first transformation
that gets applied.

Apophysis has a `random` variation that can be used to bring points
back to the center of the pattern. This program does not have that, but
by increasing the number of initial clusters (and perhaps reducing the number of
iterations) you can achieve similar goals.

I also wanted to make a depth-first-search chooser, but I think it's better to
implement that as an algorithm instead that ignores its chooser. But I'll leave
that for another branch.

## 2022-02-12 No Backtracking Chooser - Probability Analysis.

Yesterday I explored the math of the no-backtracking chooser. I confirmed
what my intuition was telling me, that it works well for 1 or 2 pairs of
transformations, but not very well above that.

For 1 pair of transformations, the uniform probability for each transform is
`1/2`. With then no-backtracking rule, the only valid transformation has
probability 1. The difference here is large, `1 - 1/2 = 1/2`

For 2 pairs, the uniform probability is `1/4`. With the no-backtracking
rule each valid transformation has probability 1/3. The difference is quite
a bit smaller, `1/3 - 1/4 = 1/12`.

For `p` pairs, the unform probability is `1/(2p)`. With no-backtracking,
the probabilities become `1/(2p - 1)`. The difference is
`1/(2p - 1) - 1/(2p) = 1/(4p^2 - 2p)`. This falls off quadratically, so
you quickly get diminishing returns by this method. 

The conclusion here is no-backtracking is great for 1 pair, okay for 2-pairs,
and not much better than uniformly random above this.

## 2022-02-13 A Productive Weekend

Yesterday and today I added quite a few things:

1. I added cluster coordinates to each cluster. For 1D shapes like a line, this
    is a single `u` coordinate. For 2D shapes like a grid or disk, this is a
    `uv` coordinate pair. For 3D shapes like a box, this is a `uvw` triple.
2. I refactored most of `glb_writer.rs` to A. refactor the accessor and buffer
    view management since it was messy, B. to add the cluster coordinates and
    other ids, and C. To switch from `EXT_mesh_features` to custom
    glTF attributes. This last bit is temporary to prepare for future changes
    to the spec, and it allows me to use the values in Custom Shaders
3. Now that I have more variables available in the shader, I tried making some
    new shaders, including one that's animated!
    
There's so much cool stuff to explore here, and more to add, but I think this
is a good stopping point for one weekend.

### 2022-02-16 New Metadata Extensions

Recently, `EXT_mesh_features` was split into two extensions: `EXT_mesh_features`
(just the feature IDs now) and `EXT_structural_metadata` (just the metadata).
After this weekend's changes to use custom glTF attributes, this was a lot
easier to implement.

I'm not going to merge this branch for a while though since these extensions
are still being implemented in CesiumJS.

## 2022-02-17 Steps towards implicit tiling

Another feature of 3D Tiles Next that will be helpful is using implicit tiling
for more compact tileset descriptions. In order to do this though, tiles
need to be identified by `(tile, x, y, z)` coordinates. The filenames have to
be listed accordingly too. So far I've done this much.

In another branch, the next step is to handle the `3DTILES_implicit_tiling`
extension and subtree files.

## 2022-02-18 Triangles and Tetrahedra

Yesterday I added a `Triangle` chooser to make a grid of points arranged
in a triangle. The UVs are barycentric coordinates. Today I added a
`Tetrahedron`, very similar just with an extra dimension.

I also came across the 2019 Bridges math art paper, "Discrete Gyroid Surface" by
Ulrich Reitebuch, Martin Skrodzki and Konrad Polthier, which makes a simplified
gyroid mesh that focuses on the symmetries of the structure. I figured this
would be a cool test of my triangle chooser, so I implemented it. I had to get
clever to show the "front" and "back" sides, both by adding extra triangles
offset by a tiny bit, and updating the shader to fix the coloring scheme
so it matches the paper. see `gyroid_sides.json` (I might rename this one to 
`gyroid` and remove the other one later, this one turned out better.)

I'm also realizing that some fractals would benefit from specialized shaders.
Maybe one dropdown option selects a specialized shader when available?
I suppose it would also be possible to include the shader text in the fractal,
but in JSON form you'd have to escape all the newlines, it would be hard to
read. I'll think about this... maybe not in the `clusters` branch though, it's
already quite bloated.

## 2022-03-13 Wrap up Clusters branch

Yikes, I haven't updated the logbook on this branch in many weeks... I've been
slowly chipping away at cleaning up the `clusters` branch. This branch grew
rather large, so cleaning it up gets tedious. And tedious means motivation to
start is low. But finally I got it wrapped up!

I updated the paramter files and also added some new screenshots. The new
shaders + better clusters really help to improve the visual quality.

## 2022-06-21 3D Tiles 1.1 and Copyright Clarification

Today I merged in the `3d-tiles-1.1` branch, which updates the schema to the
latest 3D Tiles specification draft.

I also added a `LICENSE` and noted that the copyright belongs to Cesium given
an Assignment of Inventions Agreement I signed when I started at Cesium.