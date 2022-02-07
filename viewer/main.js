import { ReferenceGeometry } from './ReferenceGeometry.js';
import { FractalShading } from './FractalShading.js';

const defined = Cesium.defined;
const Matrix4 = Cesium.Matrix4;
const Cartesian3 = Cesium.Cartesian3;

// My Cesium ion access token. Yes, it's normal practice to put this in
// public-facing code
Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI0ZTVjMjExYy0wYWVlLTRmNzktODM4Ni0zNzRlMjdjZDIxZmMiLCJpZCI6MTg5MTksInNjb3BlcyI6WyJhc3IiLCJnYyJdLCJpYXQiOjE1NzQ2ODg1MzJ9.aeY06JtwkvYi5MylE_cJd8QveIxvjUIb-E4HtGJ6gbg';

Cesium.ExperimentalFeatures.enableModelExperimental = true;

// Create the viewer. Hide the globe, I'm using Cesium for the 3D tiles
// rendering, not geospatial data.
const viewer = new Cesium.Viewer('cesiumContainer');

// The currently loaded tileset (if there is one)
let tileset;

// flags that are toggleable by checkboxes.
let attenuation = true;
let show_bboxes = false;

const shading = new FractalShading();

// Put the tileset right above City Hall in Philadelphia, PA, U.S.
const position = Cartesian3.fromDegrees(-75.1635996, 39.9523789, 50);
const modelMatrix = Cesium.Transforms.eastNorthUpToFixedFrame(position);

// Switch to a new model
function set_model(model_id) {
    viewer.scene.primitives.remove(tileset);

    const url = `${model_id}/tileset.json`;    

    tileset = viewer.scene.primitives.add(new Cesium.Cesium3DTileset({
        url,
        debugShowBoundingVolume: show_bboxes,
        modelMatrix: modelMatrix
    }));
    shading.apply_shader(tileset);
    
    viewer.zoomTo(tileset);
    

    // Sparse point clouds look better with this on, but it's toggleable
    // because sometimes the fractal structure is clearer with smaller points.
    tileset.pointCloudShading.attenuation = attenuation;
    tileset.pointCloudShading.maximumAttenuation = 4;

    tileset.readyPromise.then(() => {
        const metadata = tileset.metadata.tileset;
        shading.update_metadata(metadata);
    });

}

const ref_geometry = new ReferenceGeometry(viewer.scene, modelMatrix);
function init_reference_geometry() {
    for (const id of ReferenceGeometry.GEOMETRY_IDS) {
        const checkbox = document.getElementById(id);
        checkbox.addEventListener('change', (e) => {
            const show = e.target.checked;
            ref_geometry.toggle_geometry(id, show);
        });
    }
}

// Select a model
const model_select = document.getElementById('model');
model_select.addEventListener('change', (e) => {
    let model_id = e.target.value;
    set_model(model_id);
});

// Toggle point cloud attenuation. When enabled, the points are rendered
// larger to fill in gaps so the structure looks more dense
const attenuation_checkbox = document.getElementById('attenuation');
attenuation_checkbox.addEventListener('change', (e) => {
    attenuation = e.target.checked;

    if (defined(tileset)) {
        tileset.pointCloudShading.attenuation = attenuation;
    }
});

// Toggle showing bounding boxes. This is really more for debugging, but
// it looks neat!
const bbox_checkbox = document.getElementById('show-bboxes');
bbox_checkbox.addEventListener('change', (e) => {
    show_bboxes = e.target.checked;

    if (defined(tileset)) {
        tileset.debugShowBoundingVolume = show_bboxes;
    }
});

const reload_button = document.getElementById('reload');
reload_button.addEventListener('click', () => {
    const model_id = model_select.value;
    set_model(model_id);
});

function make_dropdown_option(fractal) {
    const option = document.createElement("option");
    option.textContent = fractal.name;
    option.value = fractal.id;
    model_select.appendChild(option);
}

const shader_dropdown = document.getElementById("shader");
shader_dropdown.addEventListener('change', (e) => {
    shading.current_shader = e.target.value;
    if (defined(tileset)) {
        shading.apply_shader(tileset);
    }
});
shading.populate_dropdown(shader_dropdown);



fetch("./fractals.json")
    .then((response) => response.json())
    .then((json) => {
        for (const fractal of json.fractals) {
            make_dropdown_option(fractal);
        }
        model_select.dispatchEvent(new Event("change"));
    });

init_reference_geometry();

window.viewer = viewer;
