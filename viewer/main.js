Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI0ZTVjMjExYy0wYWVlLTRmNzktODM4Ni0zNzRlMjdjZDIxZmMiLCJpZCI6MTg5MTksInNjb3BlcyI6WyJhc3IiLCJnYyJdLCJpYXQiOjE1NzQ2ODg1MzJ9.aeY06JtwkvYi5MylE_cJd8QveIxvjUIb-E4HtGJ6gbg';

const defined = Cesium.defined;


const viewer = new Cesium.Viewer('cesiumContainer', {
    globe: false
});

let tileset;
let attenuation = true;
function set_model(model_id) {
    const url = `${model_id}/tileset.json`;
    tileset = new Cesium.Cesium3DTileset({url});
    tileset.pointCloudShading.attenuation = attenuation;

    viewer.scene.primitives.removeAll();
    viewer.scene.primitives.add(tileset);
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

set_model('sierpinski');
