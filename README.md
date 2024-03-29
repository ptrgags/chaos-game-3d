## Chaos Game 3D (2019-2020, 2021-2022)

This program can be used to generate fractals using the Chaos Game as a
3D point cloud. The input are JSON parameter files, and the output is a
3D Tiles point cloud tileset.

⚠️ ARCHIVED - Due to a Assignment of Inventions Agreement I signed when I started working at Cesium, this copyright for this project was assigned to my employer Cesium GS, Inc. This project is still open-source under the Apache 2.0 license. However, I choose not to maintain this repo anymore, and have archived it.

The `viewer/` subdirectory contains a small static page one can use to view
the output.

![Sierpinski Tetrahedron fractal](figures/sierpinski.png)

### Gallery

These images correspond to some of the parameter files found in `params/`

|||
|---|---|
| `sierpinski` | `bent_sierpinski` |
| ![Sierpinski Tetrahedron](figures/sierpinski.png) | ![Bent Sierpinski Tetrahedron](figures/bent_sierpinski.png) |
| `grid_3d` | `seaweed` |
| ![3D Grid](figures/grid_3d.png) | ![Seaweed tiling](figures/seaweed.png) |
| `spiky_ball` | `discrete_gyroid_surface` |
| ![Spiky Ball](figures/spiky_ball.png) | ![Discrete Gyroid Surface](figures/discrete_gyroid.png) |
| `loxodromic` | `pillows` |
| ![Loxodromic transformation](figures/loxodromic.png) | ![Pillows](figures/pillows.png) |
| `hopf_fibration` | Animated |
| ![Hopf Fibration](figures/hopf_fibration.png) | ![Hopf Fibration Animated](figures/hopf_fibration_animated.gif)|
| `torus_knots` | Animated |
| ![Torus Knots](figures/torus_knots.png) | ![Torus Knots Animated](figures/torus_knots_animated.gif) |
| `aa_loxodromic` | Animated |
| ![Axis Aligned Loxodromic](figures/aa_loxodromic.png) | ![Axis Aligned Loxodromic Animated](figures/aa_loxodromic_animated.gif)

### Usage

Generating fractals:

```
cargo run PARAMETER_FILE
```

Where: 

* PARAMTER_FILE is a JSON file describing the fractal (see the `params/`
    directory for examples)

The script will create a new directory `viewer/<fractal_id>` containing the
fractal as a 3D Tiles tileset. The fractal ID comes from the `id` property
from the paramters JSON file.

Viewer:

* Generate fractals in `viewer/<fractal_id>`
* From the `viewer/` directory, run  `python make_index.py` to generate 
    `fractals.json`. This is a list of fractal names, IDs and descriptions
    pulled from the tileset JSON files (`viewer/<fractal_id>/tileset.json`).
    This only works for newer GLB fractals.
* Run the `viewer` directory as a static site (e.g. via `http-server` (Node.js)
    or `python -m http.server`)