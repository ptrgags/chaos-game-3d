Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI0ZTVjMjExYy0wYWVlLTRmNzktODM4Ni0zNzRlMjdjZDIxZmMiLCJpZCI6MTg5MTksInNjb3BlcyI6WyJhc3IiLCJnYyJdLCJpYXQiOjE1NzQ2ODg1MzJ9.aeY06JtwkvYi5MylE_cJd8QveIxvjUIb-E4HtGJ6gbg';

const viewer = new Cesium.Viewer('cesiumContainer', {
    globe: false
});

const tileset = viewer.scene.primitives.add(
    new Cesium.Cesium3DTileset({
        url: 'tileset_test/tileset.json'
    })
);
tileset.pointCloudShading.attenuation = true;
