const UNLIT = new Cesium.CustomShader({
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,  
});

const COLOR_ITERATIONS = new Cesium.CustomShader({
    uniforms: {
        u_iterations: {
            type: Cesium.UniformType.FLOAT,
            value: 1
        }
    },
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        float iter_normalized = fsInput.attributes.featureId_0 / (u_iterations - 1.0);
        //vec3 rgb = czm_HSBToRGB(vec3(iter_normalized, 0.8, 1.0));
        material.diffuse = iter_normalized * vec3(1.0, 0.5, 0.01);
    }
    `
})

const COLOR_CLUSTER_COPIES = new Cesium.CustomShader({
    uniforms: {
        u_cluster_copies: {
            type: Cesium.UniformType.FLOAT,
            value: 1
        }
    },
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        float copy_normalized = fsInput.attributes.featureId_1 / u_cluster_copies;
        vec3 rgb = czm_HSBToRGB(vec3(copy_normalized, 0.8, 1.0));
        material.diffuse = rgb;
    }
    `
});

const COLOR_LAST_XFORMS = new Cesium.CustomShader({
    uniforms: {
        u_xform_count: {
            type: Cesium.UniformType.FLOAT,
            value: 1
        }
    },
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        float copy_normalized = fsInput.attributes.last_xform / u_xform_count;
        vec3 rgb = czm_HSBToRGB(vec3(copy_normalized, 0.8, 1.0));
        material.diffuse = rgb;
    }
    `
});

const HIGHLIGHT_FIRST = new Cesium.CustomShader({
    uniforms: {
        u_cluster_copies: {
            type: Cesium.UniformType.FLOAT,
            value: 1
        }
    },
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        float id = vsInput.attributes.featureId_1;
        vsOutput.pointSize = mix(4.0, 8.0, float(id == 0.0));
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        float id = fsInput.attributes.featureId_1;
        float is_first = float(id == 0.0);
        material.diffuse = mix(material.diffuse, vec3(1.0), is_first);
    }
    `
});

const COLOR_BY_DISTANCE = new Cesium.CustomShader({
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        float dist_from_center = length(fsInput.attributes.positionMC);
        float freq = 4.0;
        float wave = 0.5 + 0.5 * cos(2.0 * czm_pi * freq * dist_from_center);
        material.diffuse *= wave;
    }
    `
});

const COLOR_OCTANTS = new Cesium.CustomShader({
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        vec3 position = fsInput.attributes.positionMC;
        vec3 octants = step(0.0, position);
        material.diffuse = octants;
    }
    `
});


const VIEW_CLUSTER_COORDINATES = new Cesium.CustomShader({
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        material.diffuse = fsInput.attributes.cluster_coordinates;
    }
    `
});

const TRIANGLE_EDGES = new Cesium.CustomShader({
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        vec3 barycentric = fsInput.attributes.cluster_coordinates;
        float min_coordinate = min(min(barycentric.x, barycentric.y), barycentric.z);
        float edge = smoothstep(0.1, 0.0, min_coordinate);
        material.diffuse = mix(material.diffuse, vec3(1.0), edge);
    }
    `
});

const GYROID = new Cesium.CustomShader({
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 8.0;
    }
    `,
    fragmentShaderText: `
    float xnor(float a, float b) {
        float a_and_b = min(a, b);
        float not_a_and_not_b = min(1.0 - a, 1.0 - b);
        return max(a_and_b, not_a_and_not_b);
    }

    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        vec3 pos = fsInput.attributes.positionMC;
        vec3 cell_id = floor((pos + 1.0) / 2.0);
        float diagonal = cell_id.x + cell_id.y + cell_id.z;
        float parity = mod(diagonal, 2.0);
        float is_back = float(fsInput.attributes.featureId_2 > 1.0);
        vec3 color1 = vec3(0.5, 0.0, 1.0);
        vec3 color2 = vec3(0.0, 1.0, 1.0);
        material.diffuse = mix(color1, color2, xnor(parity, is_back));

        vec3 barycentric = fsInput.attributes.cluster_coordinates;
        float min_coordinate = min(min(barycentric.x, barycentric.y), barycentric.z);
        float edge = smoothstep(0.1, 0.0, min_coordinate);
        material.diffuse = mix(material.diffuse, vec3(1.0), edge);
    }
    `
});

const ANIMATE_CUMULATIVE = new Cesium.CustomShader({
    uniforms: {
        u_iterations: {
            type: Cesium.UniformType.FLOAT,
            value: 1
        },
        u_time: {
            type: Cesium.UniformType.FLOAT,
            value: 0
        }
    },
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        float iter_normalized = vsInput.attributes.featureId_0 / (u_iterations - 1.0);
        float t = mod(0.1 * u_time, 1.0);

        vsOutput.pointSize = 4.0;

        // hide points by multiplying by NaN
        if (iter_normalized > t) {
            vsOutput.positionMC = vec3(0.0) / 0.0;
        }
    }
    `,

    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        float iter_normalized = fsInput.attributes.featureId_0 / (u_iterations - 1.0);
        float t = mod(0.1 * u_time, 1.0);

        float hotspot = smoothstep(0.2, 0.1, t - iter_normalized);
        float brightness = smoothstep(0.5, 0.3, t - iter_normalized);
        vec3 color = 0.8 * vec3(brightness) + 0.2;
        color = mix(color, vec3(1.0, 0.5, 0.0), hotspot);
        material.diffuse = color;
    }
    `
});

const ANIMATE_PULSE = new Cesium.CustomShader({
    uniforms: {
        u_iterations: {
            type: Cesium.UniformType.FLOAT,
            value: 1
        },
        u_time: {
            type: Cesium.UniformType.float,
            value: 0
        }
    },
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        material.diffuse = fsInput.attributes.cluster_coordinates;
    }
    `
});

const ANIMATE_HIGHLIGHT = new Cesium.CustomShader({
    uniforms: {
        u_iterations: {
            type: Cesium.UniformType.FLOAT,
            value: 1
        },
        u_time: {
            type: Cesium.UniformType.float,
            value: 0
        }
    },
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        material.diffuse = fsInput.attributes.cluster_coordinates;
    }
    `
});

const SHADERS = {
    unlit: UNLIT,
    iterations: COLOR_ITERATIONS,
    cluster_copies: COLOR_CLUSTER_COPIES,
    last_xform: COLOR_LAST_XFORMS,
    first: HIGHLIGHT_FIRST,
    distance: COLOR_BY_DISTANCE,
    octant: COLOR_OCTANTS,
    cluster_coordinates: VIEW_CLUSTER_COORDINATES,
    triangle_edges: TRIANGLE_EDGES,
    gyroid: GYROID,
    animate_cumulative: ANIMATE_CUMULATIVE,
    animate_pulse: ANIMATE_PULSE,
    animate_highlight: ANIMATE_HIGHLIGHT
};

const OPTIONS = [
    {
        name: "Unlit",
        value: "unlit"
    },
    {
        name: "Color by iteration",
        value: "iterations"
    },
    {
        name: "Color by cluster copy",
        value: "cluster_copies"
    },
    {
        name: "Color by last xform",
        value: "last_xform"
    },
    {
        name: "View cluster coordinates",
        value: "cluster_coordinates"
    },
    {
        name: "Emphasize Triangle Edges",
        value: "triangle_edges"
    },
    {
        name: "Discrete Gyroid Surface",
        value: "gyroid"
    },
    {
        name: "Highlight first cluster",
        value: "first"
    },
    {
        name: "Color by Distance from Origin",
        value: "distance"
    },
    {
        name: "Color by Octant",
        value: "octant"
    },
    {
        name: "Animate iterations (cumulatively)",
        value: "animate_cumulative"
    },
    {
        name: "Animate iterations (pulse)",
        value: "animate_pulse"
    },
    {
        name: "Animate iterations (highlight)",
        value: "animate_highlight"
    }
];


class FractalShading {
    constructor() {
        this.current_shader = "unlit";
    }

    apply_shader(tileset) {
        const shader = SHADERS[this.current_shader];
        tileset.customShader = shader;
    }

    make_dropdown() {
        const dropdown = document.getElementById("shader");
        dropdown.addEventListener('change', (e) => {
            this.current_shader
        });

        for (const option of options) {
            const element = document.createElement("option")
        }

        dropdown.dispatchEvent(new Event("change"));
    }

    update_metadata(metadata) {
        const iterations = metadata.getProperty("iterations");
        COLOR_ITERATIONS.setUniform("u_iterations", iterations);
        ANIMATE_CUMULATIVE.setUniform("u_iterations", iterations);
        ANIMATE_PULSE.setUniform("u_iterations", iterations);
        ANIMATE_HIGHLIGHT.setUniform("u_iterations", iterations);

        const ifs_xform_count = metadata.getProperty("ifs_xform_count");
        COLOR_LAST_XFORMS.setUniform("u_xform_count", ifs_xform_count);

        //"cluster_point_count":500,"cluster_copies":5,"ifs_xform_count":6,"color_ifs_xform_count":1,"algorithm":"chaos_sets","node_capacity":5000
        const cluster_copies = metadata.getProperty("cluster_copies");
        COLOR_CLUSTERS.setUniform("u_cluster_copies", cluster_copies);
    }

    update_time(time_sec) {
        ANIMATE_CUMULATIVE.setUniform("u_time", time_sec);
        ANIMATE_PULSE.setUniform("u_time", time_sec);
        ANIMATE_HIGHLIGHT.setUniform("u_time", time_sec);
    }

    populate_dropdown(dropdown_element) {
        for (const option of OPTIONS) {
            const element = document.createElement("option");
            element.textContent = option.name;
            element.value = option.value;
            dropdown_element.appendChild(element);
        }
    }
}

export {
    FractalShading
};