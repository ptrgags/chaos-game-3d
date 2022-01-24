import {ReferenceGeometry} from './ReferenceGeometry.js';

const defined = Cesium.defined;
const Matrix4 = Cesium.Matrix4;
const Cartesian3 = Cesium.Cartesian3;

// My Cesium ion access token. Yes, it's normal practice to put this in
// public-facing code
Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI0ZTVjMjExYy0wYWVlLTRmNzktODM4Ni0zNzRlMjdjZDIxZmMiLCJpZCI6MTg5MTksInNjb3BlcyI6WyJhc3IiLCJnYyJdLCJpYXQiOjE1NzQ2ODg1MzJ9.aeY06JtwkvYi5MylE_cJd8QveIxvjUIb-E4HtGJ6gbg';

Cesium.ExperimentalFeatures.enableModelExperimental = true;

// Create the viewer. Hide the globe, I'm using Cesium for the 3D tiles
// rendering, not geospatial data.
const viewer = new Cesium.Viewer('cesiumContainer', {
    globe: false
});

// The currently loaded tileset (if there is one)
let tileset;

// flags that are toggleable by checkboxes.
let attenuation = true;
let show_bboxes = false;

const customShader = new Cesium.CustomShader({
    uniforms: {
        u_initial_set_copies: {
            type: Cesium.UniformType.FLOAT,
            value: 1
        }
    },
    LightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        //Sometimes it's helpful to visualize depth
        //float dist_from_center = length(fsInput.attributes.positionMC);
        //float wave = 0.5 + 0.5 * cos(2.0 * 3.1415 * 2.5 * dist_from_center);
        float id_normalized = (fsInput.attributes.featureId_0 + 1.0) / u_initial_set_copies;
        vec3 rgb = czm_HSBToRGB(vec3(id_normalized, 0.8, 1.0));
        material.diffuse = rgb;
    }
    `
})

// Switch to a new model
function set_model(model_id) {
    viewer.scene.primitives.remove(tileset);

    const url = `${model_id}/tileset.json`;

    // Until I figure out better camera settings, scale up the tileset
    // to be larger than the radius earth, which is 6.37 million meters
    // 
    // Scaling it here with a model matrix is a lot easier than baking the
    // scale into the tileset, though it causes some issues
    const BIGGER_THAN_EARTH = 10000000.0;
    const scaleAmount = new Cartesian3(
        BIGGER_THAN_EARTH, BIGGER_THAN_EARTH, BIGGER_THAN_EARTH);
    const scale = Matrix4.fromScale(scaleAmount);

    // Create a new tileset. The old tileset is discarded to the garbage
    // collector.
    tileset = new Cesium.Cesium3DTileset({
        url,
        debugShowBoundingVolume: show_bboxes,
        modelMatrix: scale,
        customShader: customShader
    });

    // Sparse point clouds look better with this on, but it's toggleable
    // because sometimes the fractal structure is clearer with smaller points.
    tileset.pointCloudShading.attenuation = attenuation;

    tileset.readyPromise.then(() => {
        const metadata = tileset.metadata.tileset;
        const initial_set_copies = metadata.getProperty("initial_set_copies");
        customShader.setUniform("u_initial_set_copies", initial_set_copies);
    });

    // Force all tiles to load. This is a bit dangerous for large tilesets,
    // but until I fix some camera issues, this is the only way to render
    // things properly
    //tileset.maximumScreenSpaceError = 0;

    viewer.scene.primitives.add(tileset);
}

// Something to experiment with later.
viewer.scene.logarithmicDepthBuffer = false;

function configure_camera() {
    const camera = viewer.scene.camera;
    const frustum = camera.frustum;

    // Prevent clipping when we zoom in close to see details
    frustum.near = 1e-4;
    frustum.far = 1e11;
}

const ref_geometry = new ReferenceGeometry(viewer.scene);
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

fetch("./fractals.json")
    .then((response) => response.json())
    .then((json) => {
        for (const fractal of json.fractals) {
            make_dropdown_option(fractal);
        }
        model_select.dispatchEvent(new Event("change"));
    });

configure_camera();
init_reference_geometry();

window.viewer = viewer;
