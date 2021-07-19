use bevy_render::{pipeline::{BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrite, CompareFunction, CullMode, DepthBiasState, DepthStencilState, FrontFace, MultisampleState, PipelineDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, StencilFaceState, StencilState}, shader::ShaderStages, texture::TextureFormat};


pub fn shadertoy_pipeline(shader_stages : ShaderStages) -> PipelineDescriptor {
    PipelineDescriptor {
        name: None,
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: CullMode::Back,
                polygon_mode: PolygonMode::Fill,
            },
            layout: None,
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
                clamp_depth: false,
            }),
            color_target_states: vec![ColorTargetState {
                format: TextureFormat::default(),
                color_blend: BlendState::REPLACE,
                alpha_blend: BlendState::REPLACE,
                write_mask: ColorWrite::empty(),
            }],
            multisample: MultisampleState {
                count: 0,
                mask: 0,
                alpha_to_coverage_enabled: true,
            },
        shader_stages,
    }
}
