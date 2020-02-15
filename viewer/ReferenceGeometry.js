const Cartesian3 = Cesium.Cartesian3;
const defined = Cesium.defined;

// A bit bigger than the radius of earth. Eventually I want to change
// this to be a lot smaller
const UNIT_LENGTH = 10000000.0;
const LINE_COLOR = Cesium.Color.YELLOW;

/**
 * This class keeps track of some geometry added to the scene
 * to help visualize where points are in the scene relative to the origin.
 * This will help me explore the structure of tilings and fractals
 */
class ReferenceGeometry {
    constructor(scene) {
        this._scene = scene;
        // Map of checkbox id -> Primitive
        this._primitives = new Map();
        this._create_geometry();
    }

    toggle_geometry(id, show) {
        const primitive = this._primitives.get(id);
        if (defined(primitive)) {
            primitive.show = show;
        }
    }

    _create_geometry() {
        this._primitives.set('unit-sphere', this._make_unit_sphere());
        this._primitives.set('unit-cube', this._make_unit_cube());
        this._primitives.set(
            'x-axis', this._make_axis(new Cartesian3(1, 0, 0)));
        this._primitives.set(
            'y-axis', this._make_axis(new Cartesian3(0, 1, 0)));
        this._primitives.set(
            'z-axis', this._make_axis(new Cartesian3(0, 0, 1)));

        for (const primitive of this._primitives.values()) {
            this._scene.primitives.add(primitive);
        }
    }

    _make_unit_sphere() {
        const outline = new Cesium.EllipsoidOutlineGeometry({
            radii: new Cartesian3(UNIT_LENGTH, UNIT_LENGTH, UNIT_LENGTH),
            stackPartitions: 8,
            slicePartitions: 8
        });
        return this._make_outline_primitive(outline);
    }

    _make_unit_cube() {
        const outline = new Cesium.BoxOutlineGeometry({
            minimum: new Cartesian3(-UNIT_LENGTH, -UNIT_LENGTH, -UNIT_LENGTH),
            maximum: new Cartesian3(UNIT_LENGTH, UNIT_LENGTH, UNIT_LENGTH),
        });
        return this._make_outline_primitive(outline);
    }

    _make_axis(direction) {
        const SIZE = 10;
        const start = Cartesian3.multiplyByScalar(
            direction, 
            SIZE * UNIT_LENGTH, 
            new Cartesian3());
        const end = Cartesian3.multiplyByScalar(
            direction, 
            -SIZE * UNIT_LENGTH, 
            new Cartesian3());
        const line = new Cesium.SimplePolylineGeometry({
            positions: [start, end],
            arcType: Cesium.ArcType.NONE
        });

        return this._make_outline_primitive(line);
    }

    _make_outline_primitive(outline, modelMatrix) {
        const color_attribute = 
            Cesium.ColorGeometryInstanceAttribute.fromColor(LINE_COLOR);
        const instance = new Cesium.GeometryInstance({
            geometry: outline,
            modelMatrix,
            attributes: {
                color: color_attribute
            }
        });

        return new Cesium.Primitive({
            show: false,
            geometryInstances: instance,
            appearance: new Cesium.PerInstanceColorAppearance({
                flat: true,
                lineWidth: Math.min(2.0, this._scene.maximumAliasedLineWidth)
            })
        });
    }
}


ReferenceGeometry.GEOMETRY_IDS = [
    'unit-sphere',
    'unit-cube',
    'x-axis',
    'y-axis',
    'z-axis',
];

export {
    ReferenceGeometry
};
