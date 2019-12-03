Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI0ZTVjMjExYy0wYWVlLTRmNzktODM4Ni0zNzRlMjdjZDIxZmMiLCJpZCI6MTg5MTksInNjb3BlcyI6WyJhc3IiLCJnYyJdLCJpYXQiOjE1NzQ2ODg1MzJ9.aeY06JtwkvYi5MylE_cJd8QveIxvjUIb-E4HtGJ6gbg';

const viewer = new Cesium.Viewer('cesiumContainer', {
    globe: false
});

function set_model(model_id) {
    const url = `${model_id}/tileset.json`;
    const tileset = new Cesium.Cesium3DTileset({url});
    tileset.pointCloudShading.attenuation = true;

    viewer.scene.primitives.removeAll();
    viewer.scene.primitives.add(tileset);
}

let model_select = document.getElementById('model');
model_select.addEventListener('change', (e) => {
    let model_id = e.target.value;
    set_model(model_id);
});

set_model('sierpinski');
