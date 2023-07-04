//! A custom post processing effect, using two cameras, with one reusing the render texture of the first one.
//! Here a chromatic aberration is applied to a 3d scene containing a rotating cube.
//! This example is useful to implement your own post-processing effect such as
//! edge detection, blur, pixelization, vignette... and countless others.

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResized,
};

#[derive(Component)]
pub struct PostProcessQuadMarker;

#[derive(Component)]
pub struct CapturingCameraMarker;

#[derive(Component)]
pub struct PostProcessCameraMarker;

pub fn setup(
    mut commands: Commands,
    windows_query: Query<&Window>,
    mut meshes_asset: ResMut<Assets<Mesh>>,
    mut first_pass_material_asset: ResMut<Assets<FirstPassMaterial>>,
    mut second_pass_material_asset: ResMut<Assets<SecondPassMaterial>>,
    mut third_pass_material_asset: ResMut<Assets<ThirdPassMaterial>>,
    mut materials_asset: ResMut<Assets<StandardMaterial>>,
    mut images_asset: ResMut<Assets<Image>>,
) {
    // This assumes we only have a single window
    let window = windows_query.single();

    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba32Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images_asset.add(image);

    // Main camera, first to render
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        },
        // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
        // the cost of rendering the UI without any post processing effects.
        UiCameraConfig { show_ui: false },
        // RenderLayers::layer(1),
        RenderLayers::layer(2),
        CapturingCameraMarker,
    ));

    // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.

    {
        let quad_handle = meshes_asset.add(Mesh::from(shape::Quad::new(Vec2::new(
            size.width as f32,
            size.height as f32,
        ))));

        // This material has the texture that has been rendered.
        let material_handle = first_pass_material_asset.add(FirstPassMaterial {
            source_image: image_handle.clone(),
        });

        // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: quad_handle.into(),
                material: material_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.5),
                    ..default()
                },
                ..default()
            },
            RenderLayers::layer(3),
            PostProcessQuadMarker,
        ));

        // The post-processing pass camera.
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    target: RenderTarget::Image(image_handle.clone()),
                    // renders after the first main camera which has default value: 0.
                    order: 1,
                    ..default()
                },
                ..Camera2dBundle::default()
            },
            RenderLayers::layer(3),
            PostProcessCameraMarker,
        ));
    }

    const LEN: u8 = 5;

    for i in 1..=LEN {
        let second_quad_handle = meshes_asset.add(Mesh::from(shape::Quad::new(Vec2::new(
            size.width as f32,
            size.height as f32,
        ))));

        // This material has the texture that has been rendered.
        let second_material_handle = second_pass_material_asset.add(SecondPassMaterial {
            source_image: image_handle.clone(),
            intensity: (16 >> i) as f32,
        });

        // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: second_quad_handle.into(),
                material: second_material_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.5),
                    ..default()
                },
                ..default()
            },
            RenderLayers::layer(3 + i),
            PostProcessQuadMarker,
        ));

        // The post-processing pass camera.
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    // renders after the first main camera which has default value: 0.
                    target: RenderTarget::Image(image_handle.clone()),
                    order: 1 + i as isize,
                    ..default()
                },
                ..Camera2dBundle::default()
            },
            RenderLayers::layer(3 + i),
            PostProcessCameraMarker,
        ));
    }

    {
        let quad_handle = meshes_asset.add(Mesh::from(shape::Quad::new(Vec2::new(
            size.width as f32,
            size.height as f32,
        ))));

        // This material has the texture that has been rendered.
        let material_handle = third_pass_material_asset.add(ThirdPassMaterial {
            source_image: image_handle.clone(),
            redius: 6.0,
        });

        // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: quad_handle.into(),
                material: material_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.5),
                    ..default()
                },
                ..default()
            },
            RenderLayers::layer(3 + LEN + 1),
            PostProcessQuadMarker,
        ));

        // The post-processing pass camera.
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    // renders after the first main camera which has default value: 0.
                    order: 2 + LEN as isize,
                    ..default()
                },
                ..Camera2dBundle::default()
            },
            RenderLayers::layer(3 + LEN + 1),
            PostProcessCameraMarker,
        ));
    }
}

pub fn resize(
    mut commands: Commands,
    mut main_camera: Query<
        (Entity, &mut Camera),
        (
            With<Camera3d>,
            With<CapturingCameraMarker>,
            Without<PostProcessCameraMarker>,
        ),
    >,
    mut post_process_camera: Query<
        (Entity, &mut Camera),
        (
            With<PostProcessCameraMarker>,
            Without<CapturingCameraMarker>,
        ),
    >,
    windows: Query<&Window>,
    resized_window: Res<Events<WindowResized>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_mesh: Query<(&mut Mesh2dHandle, &Handle<FirstPassMaterial>), With<PostProcessQuadMarker>>,
    mut post_processing_materials: ResMut<Assets<FirstPassMaterial>>,
) {
    let mut reader = resized_window.get_reader();
    let mut camera = main_camera.single_mut();

    for window in reader.iter(&resized_window) {
        let RenderTarget::Image(handle) = &camera.1.target else {
            return;
        };
        if !images.contains(handle) {
            return;
        };

        let Ok(w) = windows.get(window.window) else {
            return;
        };

        let new_height = w.physical_height();
        let new_width = w.physical_width();

        if new_height == 0 || new_width == 0 {
            return;
        }

        let size = Extent3d {
            width: new_width,
            height: new_height,
            ..Default::default()
        };

        let hande = images.set(
            handle,
            Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::bevy_default(),
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                ..default()
            },
        );
        images.get_mut(&hande).unwrap().resize(size);

        camera.1.target = RenderTarget::Image(hande.clone());
        post_process_camera.for_each_mut(|mut p| {
            p.1.target = RenderTarget::Image(hande.clone());
        });

        q_mesh.for_each_mut(|p| {
            let Some(mesh) = meshes.get_mut(&p.0.0) else {
                return;
            };
            let Some(texture) = post_processing_materials.get_mut(p.1) else {
                return;
            };

            texture.source_image = hande.clone();

            meshes.set(
                &p.0 .0,
                Mesh::from(shape::Quad::new(Vec2::new(
                    size.width as f32,
                    size.height as f32,
                ))),
            );
        })
    }
}

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut q_cam: Query<&mut Transform, With<CapturingCameraMarker>>,
) {
    if keys.pressed(KeyCode::H) {
        q_cam.single_mut().translation.z += 0.1;
    }
}

// Region below declares of the custom material handling post processing effect

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-73f1-a908-54871ea597d5"]
pub struct FirstPassMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
}

impl Material2d for FirstPassMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/post_process_prepare_outline.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f28eb-c0fb-43f1-a908-54871ea597d5"]
pub struct SecondPassMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
    #[uniform(2)]
    intensity: f32,
}

impl Material2d for SecondPassMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/post_process_outline_pass.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc8f28eb-c0fb-43f1-a908-54821ea557e5"]
pub struct ThirdPassMaterial {
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
    #[uniform(2)]
    redius: f32,
}

impl Material2d for ThirdPassMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/post_process_finish_outline.wgsl".into()
    }
}
