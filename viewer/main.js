Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiI0ZTVjMjExYy0wYWVlLTRmNzktODM4Ni0zNzRlMjdjZDIxZmMiLCJpZCI6MTg5MTksInNjb3BlcyI6WyJhc3IiLCJnYyJdLCJpYXQiOjE1NzQ2ODg1MzJ9.aeY06JtwkvYi5MylE_cJd8QveIxvjUIb-E4HtGJ6gbg';

const defined = Cesium.defined;
const Cartesian3 = Cesium.Cartesian3;

const viewer = new Cesium.Viewer('cesiumContainer', {
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
    frustum.near = 0.001;
    frustum.far = 1e11;
}

// Not sure why this isn't working... 
const BIGGER_THAN_EARTH = 1000000.0;
let unit_sphere;
function add_reference_geometry() {
    /*
    const options = {
        name: 'Unit Sphere',
        position: new Cartesian3(0.0, 0.0, 0.0),
        ellipsoid: {
            radii: new Cartesian3(
                BIGGER_THAN_EARTH, 
                BIGGER_THAN_EARTH, 
                BIGGER_THAN_EARTH),
            material: Cesium.Color.WHITE.withAlpha(0.3),
            outline: true,
            outlineColor: Cesium.Color.BLACK
        }
    };
    const entity = new Cesium.Entity(options);
    viewer.entities.add(entity);
    */

    const scene = viewer.scene;
    
    // Draw a blue ellipsoid and position it on the globe surface.

var radii = new Cesium.Cartesian3(10000000.0, 10000000.0, 10000000.0);
// Create a ellipsoid geometry.
var ellipsoidGeometry = new Cesium.EllipsoidGeometry({
    vertexFormat : Cesium.PerInstanceColorAppearance.VERTEX_FORMAT,
    radii : radii
});

var outline = new Cesium.EllipsoidOutlineGeometry({
    radii : radii,
    stackPartitions : 16,
    slicePartitions : 8
});

// Create a geometry instance using the geometry
// and model matrix created above.
var ellipsoidInstance = new Cesium.GeometryInstance({
    geometry : ellipsoidGeometry,
    attributes : {
        color : Cesium.ColorGeometryInstanceAttribute.fromColor(Cesium.Color.BLUE)
    }
});

// Create a geometry instance using the geometry
// and model matrix created above.
var outlineInstance = new Cesium.GeometryInstance({
    geometry : outline,
    attributes : {
        color : Cesium.ColorGeometryInstanceAttribute.fromColor(Cesium.Color.WHITE)
    }
});


// Add the geometry instance to primitives.
scene.primitives.add(new Cesium.Primitive({
    geometryInstances : ellipsoidInstance,
    asynchronous: true,
    appearance : new Cesium.PerInstanceColorAppearance({
        translucent : true,
        closed : true
    })
}));

// Add the geometry instance to primitives.
scene.primitives.add(new Cesium.Primitive({
    geometryInstances : outlineInstance,
    asynchronous: true,
    appearance : new Cesium.PerInstanceColorAppearance({
        flat: true,
        lineWidth : Math.min(2.0, scene.maximumAliasedLineWidth)
    })
}));

    /*
    unit_sphere = viewer.entities.add(new Cesium.Entity({
        name: 'Red sphere with black outline',
        position: Cesium.Cartesian3(1.0, 0.0, 0.0),
        ellipsoid: {
            radii : new Cesium.Cartesian3(1000000.0, 1000000.0, 1000000.0),
            material : Cesium.Color.RED,
            outline : true,
            outlineColor : Cesium.Color.BLACK
        }
    }));
    */
}

function remove_reference_geometry() {
    viewer.entities.removeAll();
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
add_reference_geometry();
