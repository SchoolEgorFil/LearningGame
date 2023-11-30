use bevy::{
    prelude::{Component, Handle, Image, Asset},
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Component)]
pub struct PostProcessQuadMarker;

#[derive(Component)]
pub struct CapturingCameraMarker;

#[derive(Component)]
pub struct PostProcessCameraMarker;

// Region below declares of the custom material handling post processing effect

/// Our custom post processing material
#[derive(Asset, TypePath, AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-73f1-a908-54871ea597d5"]
pub struct FirstPassMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    pub(crate) source_image: Handle<Image>,
}

impl Material2d for FirstPassMaterial {
    fn fragment_shader() -> ShaderRef {
        "internal/shaders/post_process_prepare_outline.wgsl".into()
    }
    
}

#[derive(Asset, TypePath, AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f28eb-c0fb-43f1-a908-54871ea597d5"]
pub struct SecondPassMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    pub(crate) source_image: Handle<Image>,
    #[uniform(2)]
    pub(crate) intensity: f32,
}

impl Material2d for SecondPassMaterial {
    fn fragment_shader() -> ShaderRef {
        "internal/shaders/post_process_outline_pass.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc8f28eb-c0fb-43f1-a908-54821ea557e5"]
pub struct ThirdPassMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub(crate) source_image: Handle<Image>,
    #[uniform(2)]
    pub(crate) redius: f32,
}

impl Material2d for ThirdPassMaterial {
    fn fragment_shader() -> ShaderRef {
        "internal/shaders/post_process_finish_outline.wgsl".into()
    }
}
