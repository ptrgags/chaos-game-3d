const defined = Cesium.defined;
const Matrix4 = Cesium.Matrix4;
const Cartesian3 = Cesium.Cartesian3;

// My Cesium ion access token. Yes, it's normal practice to put this in
// public-facing code
Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI0ZTVjMjExYy0wYWVlLTRmNzktODM4Ni0zNzRlMjdjZDIxZmMiLCJpZCI6MTg5MTksInNjb3BlcyI6WyJhc3IiLCJnYyJdLCJpYXQiOjE1NzQ2ODg1MzJ9.aeY06JtwkvYi5MylE_cJd8QveIxvjUIb-E4HtGJ6gbg';

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

// Switch to a new model
function set_model(model_id) {
    // Remove the old tileset from the scene
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
        modelMatrix: scale
    });

    // Sparse point clouds look better with this on, but it's toggleable
    // because sometimes the fractal structure is clearer with smaller points.
    tileset.pointCloudShading.attenuation = attenuation;

    // Force all tiles to load. This is a bit dangerous for large tilesets,
    // but until I fix some camera issues, this is the only way to render
    // things properly
    tileset.maximumScreenSpaceError = 0;

    viewer.scene.primitives.add(tileset);
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

set_model('sierpinski');
