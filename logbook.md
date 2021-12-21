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