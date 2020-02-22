Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI0ZTVjMjExYy0wYWVlLTRmNzktODM4Ni0zNzRlMjdjZDIxZmMiLCJpZCI6MTg5MTksInNjb3BlcyI6WyJhc3IiLCJnYyJdLCJpYXQiOjE1NzQ2ODg1MzJ9.aeY06JtwkvYi5MylE_cJd8QveIxvjUIb-E4HtGJ6gbg';

const defined = Cesium.defined;
const Matrix4 = Cesium.Matrix4;
const Cartesian3 = Cesium.Cartesian3;

const viewer = new Cesium.Viewer('cesiumContainer', {
    globe: false
});

const BIGGER_THAN_EARTH = 10000000.0;

let tileset;
let attenuation = true;
let show_bboxes = false;
function set_model(model_id) {
    const url = `${model_id}/tileset.json`;


    const scaleAmount = new Cartesian3(
        BIGGER_THAN_EARTH, BIGGER_THAN_EARTH, BIGGER_THAN_EARTH);
    const scale = Matrix4.fromScale(scaleAmount);

    tileset = new Cesium.Cesium3DTileset({
        url,
        debugShowBoundingVolume: show_bboxes,
        modelMatrix: scale
    });
    tileset.pointCloudShading.attenuation = attenuation;
    tileset.maximumScreenSpaceError = 0;

    viewer.scene.primitives.removeAll();
    viewer.scene.primitives.add(tileset);
}

const model_select = document.getElementById('model');
model_select.addEventListener('change', (e) => {
    let model_id = e.target.value;
    set_model(model_id);
});

const attenuation_checkbox = document.getElementById('attenuation');
attenuation_checkbox.addEventListener('change', (e) => {
    attenuation = e.target.checked;

    if (defined(tileset)) {
        tileset.pointCloudShading.attenuation = attenuation;
    }
});

const bbox_checkbox = document.getElementById('show-bboxes');
bbox_checkbox.addEventListener('change', (e) => {
    show_bboxes = e.target.checked;

    if (defined(tileset)) {
        tileset.debugShowBoundingVolume = show_bboxes;
    }
});

set_model('sierpinski');
