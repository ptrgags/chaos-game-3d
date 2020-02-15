import {ReferenceGeometry} from './ReferenceGeometry.js';

Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI0ZTVjMjExYy0wYWVlLTRmNzktODM4Ni0zNzRlMjdjZDIxZmMiLCJpZCI6MTg5MTksInNjb3BlcyI6WyJhc3IiLCJnYyJdLCJpYXQiOjE1NzQ2ODg1MzJ9.aeY06JtwkvYi5MylE_cJd8QveIxvjUIb-E4HtGJ6gbg';

const defined = Cesium.defined;
const Cartesian3 = Cesium.Cartesian3;

const viewer = new Cesium.CesiumWidget('cesiumContainer', {
    globe: false
});

let tileset;
let attenuation = true;
function set_model(model_id) {
    viewer.scene.primitives.remove(tileset);

    const url = `${model_id}/tileset.json`;
    tileset = new Cesium.Cesium3DTileset({url});
    tileset.pointCloudShading.attenuation = attenuation;
    viewer.scene.primitives.add(tileset);
}

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

const model_select = document.getElementById('model');
model_select.addEventListener('change', (e) => {
    let model_id = e.target.value;
    set_model(model_id);
});

const checkbox = document.getElementById('attenuation');
checkbox.addEventListener('change', (e) => {
    attenuation = e.target.checked;

    if (defined(tileset)) {
        tileset.pointCloudShading.attenuation = attenuation;
    }
});

const reload_button = document.getElementById('reload');
reload_button.addEventListener('click', () => {
    const model_id = model_select.value;
    set_model(model_id);
});

set_model('sierpinski');
configure_camera();
init_reference_geometry();

window.viewer = viewer;
