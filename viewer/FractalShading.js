const SHADERS = {};

SHADERS.unlit = new Cesium.CustomShader({
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        vsOutput.pointSize = 4.0;
    }
    `,  
});

SHADERS.iterations = new Cesium.CustomShader({
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

SHADERS.cluster_copies = new Cesium.CustomShader({
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

SHADERS.last_xform = new Cesium.CustomShader({
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

SHADERS.first = new Cesium.CustomShader({
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

SHADERS.distance = new Cesium.CustomShader({
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

SHADERS.octant = new Cesium.CustomShader({
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


SHADERS.cluster_coordinates = new Cesium.CustomShader({
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

SHADERS.triangle_edges = new Cesium.CustomShader({
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

SHADERS.gyroid = new Cesium.CustomShader({
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

SHADERS.animate_cumulative = new Cesium.CustomShader({
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

SHADERS.animate_pulse = new Cesium.CustomShader({
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
    const float RADIUS = 0.1;
    const float LOOP_TIME = 1.0;

    // loop the animation
    float loop(float x) {
        return mod(x + RADIUS, LOOP_TIME + 2.0 * RADIUS) - RADIUS;
    }

    // Make a bell-curve shape though it uses smoothstep to exactly meet the
    // x axis
    float bell(float x) {
        return smoothstep(1.0, 0.0, abs(x) / RADIUS);
    }

    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        float x = vsInput.attributes.featureId_0 / (u_iterations - 1.0);
        float t = 0.1 * u_time;

        float animation_curve = bell(loop(x - t));

        // Discard points outside the bell curve
        if (animation_curve == 0.0) {
            vsOutput.positionMC = vec3(0.0) / 0.0;
        }

        vsOutput.pointSize = 6.0 * animation_curve;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        material.diffuse = fsInput.attributes.cluster_coordinates;
    }
    `
});

SHADERS.animate_highlight = new Cesium.CustomShader({
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
        SHADERS.iterations.setUniform("u_iterations", iterations);
        SHADERS.animate_cumulative.setUniform("u_iterations", iterations);
        SHADERS.animate_pulse.setUniform("u_iterations", iterations);
        SHADERS.animate_highlight.setUniform("u_iterations", iterations);

        const ifs_xform_count = metadata.getProperty("ifs_xform_count");
        SHADERS.last_xform.setUniform("u_xform_count", ifs_xform_count);

        //"cluster_point_count":500,"cluster_copies":5,"ifs_xform_count":6,"color_ifs_xform_count":1,"algorithm":"chaos_sets","node_capacity":5000
        const cluster_copies = metadata.getProperty("cluster_copies");
        SHADERS.cluster_copies.setUniform("u_cluster_copies", cluster_copies);
    }

    update_time(time_sec) {
        SHADERS.animate_cumulative.setUniform("u_time", time_sec);
        SHADERS.animate_pulse.setUniform("u_time", time_sec);
        SHADERS.animate_highlight.setUniform("u_time", time_sec);
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