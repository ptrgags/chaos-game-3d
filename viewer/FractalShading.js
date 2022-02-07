const UNLIT = new Cesium.CustomShader({
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        float id = vsInput.attributes.featureId_0;
        vsOutput.pointSize = 4.0;
    }
    `,  
});

const COLOR_CLUSTERS = new Cesium.CustomShader({
    uniforms: {
        u_cluster_copies: {
            type: Cesium.UniformType.FLOAT,
            value: 1
        }
    },
    lightingModel: Cesium.LightingModel.UNLIT,
    vertexShaderText: `
    void vertexMain(VertexInput vsInput, inout czm_modelVertexOutput vsOutput) {
        float id = vsInput.attributes.featureId_0;
        vsOutput.pointSize = 4.0;
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        float id_normalized = (fsInput.attributes.featureId_0 + 1.0) / u_cluster_copies;
        vec3 rgb = czm_HSBToRGB(vec3(id_normalized, 0.8, 1.0));
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
        float id = vsInput.attributes.featureId_0;
        vsOutput.pointSize = mix(4.0, 8.0, float(id == 0.0));
    }
    `,
    fragmentShaderText: `
    void fragmentMain(FragmentInput fsInput, inout czm_modelMaterial material) {
        float id = fsInput.attributes.featureId_0;
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
        float freq = 8.0;
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

const SHADERS = {
    unlit: UNLIT,
    cluster: COLOR_CLUSTERS,
    first: HIGHLIGHT_FIRST,
    distance: COLOR_BY_DISTANCE,
    octant: COLOR_OCTANTS,
};

const OPTIONS = [
    {
        name: "Unlit",
        value: "unlit"
    },
    {
        name: "Color by Cluster",
        value: "cluster"
    },
    {
        name: "Highlight First Cluster",
        value: "first"
    },
    {
        name: "Color by Distance from Origin",
        value: "distance"
    },
    {
        name: "Color by Octant",
        value: "octant"
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
        const cluster_copies = metadata.getProperty("cluster_copies");
        COLOR_CLUSTERS.setUniform("u_cluster_copies", cluster_copies);
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