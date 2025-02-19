use bevy::core::Name;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::contrast_adaptive_sharpening::ContrastAdaptiveSharpeningSettings;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::core_pipeline::prepass::{DepthPrepass, NormalPrepass};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::core_pipeline::Skybox;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::{AmbientLight, UiCameraConfig, Res};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        shape::{self, Quad},
        AssetServer, Assets, BuildChildren, Camera, Camera2d, Camera2dBundle, Camera3d,
        Camera3dBundle, Color, Commands, Entity, EnvironmentMapLight, EventReader, Image, Mesh,
        Query, ResMut, Transform, Vec2, Vec3, With,
    },
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
    window::Window,
};

use crate::lib::tools::resources::AllSettings;
use crate::lib::tools::{events, markers};

use super::materials::{FirstPassMaterial, SecondPassMaterial, ThirdPassMaterial};

pub fn setup(
    mut commands: Commands,
    windows_q: Query<&Window>,
    mut meshes_a: ResMut<Assets<Mesh>>,
    mut first_pass_material_a: ResMut<Assets<FirstPassMaterial>>,
    mut second_pass_material_a: ResMut<Assets<SecondPassMaterial>>,
    mut third_pass_material_a: ResMut<Assets<ThirdPassMaterial>>,
    // mut materials_a: ResMut<Assets<StandardMaterial>>,
    mut images_a: ResMut<Assets<Image>>,
    player: Res<AllSettings>,

    mut camera_ev_r: EventReader<events::SpawnPlayerCamera>,
    camera_continer_q: Query<Entity, With<markers::PlayerCameraContainerMarker>>,

    asset_server: ResMut<AssetServer>, // mut prepare_camera_w: EventWriter<events::camera::PlayerCameraPrepareEvent>,
) {
    let Ok(container) = camera_continer_q.get_single() else {
        return;
    };

    if camera_ev_r.len() != 1 {
        return;
    }

    let ev = camera_ev_r.iter().next();

    // This assumes we only have a single window
    let window = windows_q.single();

    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..Default::default()
    };

    let image_handle = {
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
            ..Default::default()
        };
        image.resize(size);
        images_a.add(image)
    };

    let mut id = Entity::PLACEHOLDER;

    let mut clear_color = Color::BLACK;
    {
        // Cameras
        commands.entity(container).with_children(|p| {
            id = p
                .spawn((
                    markers::PlayerCamera,
                    markers::PlayerMainCamera,
                    UiCameraConfig {
                        show_ui: true
                    },
                    Camera3dBundle {
                        camera: Camera {
                            target: RenderTarget::Window(bevy::window::WindowRef::Primary),
                            hdr: true,
                            ..Default::default()
                        },
                        projection: bevy::prelude::Projection::Perspective(
                            bevy::prelude::PerspectiveProjection {
                                fov: std::f32::consts::FRAC_PI_2/90.*player.fov,
				far: 20.,
                                ..Default::default()
                            },
                        ),
                        camera_3d: Camera3d {
                            clear_color: ClearColorConfig::Custom(clear_color),
                            ..Default::default()
                        },
                        tonemapping: Tonemapping::TonyMcMapface,
                        dither: DebandDither::Enabled,
                        ..Default::default()
                    },
                    // EnvironmentMapLight {
                    //     diffuse_map: asset_server
                    //         .load("internal/environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
                    //     specular_map: asset_server
                    //         .load("internal/environment_maps/pisa_specular_rbg9e5_zstd.ktx2"),
                    // },
                     BloomSettings::NATURAL,
                     (
                        bevy::pbr::ScreenSpaceAmbientOcclusionSettings { quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Medium },
                        // depth_prepass: DepthPrepass,
                        NormalPrepass
                     ),
                     TemporalAntiAliasBundle::default(),
                     ContrastAdaptiveSharpeningSettings {
                        ..Default::default()
                     },
                    // TemporalAntiAliasBundle::default(), // UiCameraConfig { show_ui: false },
                    // RenderLayers::layer(1),
                ))
                .id();

            // p.spawn((
            //     markers::PlayerCamera,
            //     markers::PlayerBorderPostProcessCamera,
            //     Camera3dBundle {
            //         camera: Camera {
            //             target: RenderTarget::Image(image_handle.clone()),
            //             hdr: true,
            //             ..Default::default()
            //         },
            //         projection: bevy::prelude::Projection::Perspective(
            //             bevy::prelude::PerspectiveProjection {
            //                 fov: std::f32::consts::FRAC_PI_2,
            //                 ..Default::default()
            //             },
            //         ),
            //         camera_3d: Camera3d {
            //             clear_color: ClearColorConfig::Custom(Color::BLACK),
            //             ..Default::default()
            //         },
            //         ..Default::default()
            //     },
            //     EnvironmentMapLight {
            //         diffuse_map: asset_server
            //             .load("internal/environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            //         specular_map: asset_server
            //             .load("internal/environment_maps/pisa_specular_rbg9e5_zstd.ktx2"),
            //     },
            //     UiCameraConfig { show_ui: false },
            //     // RenderLayers::layer(1),
            // ));
            // commands.spawn(CameraBun {
            //         ..Default::default()
            //     });
            // });
        });
    }
    if let Some(spawnP) = ev {
        if let Some(amb) = spawnP.camera_params.0 {
            clear_color = amb.1;
            commands.insert_resource(AmbientLight {
                brightness: amb.0,
                color: amb.1,
            });
        }
        if let Some(sky) = &spawnP.camera_params.1 {
            let lx = asset_server.load(sky);
            commands.entity(id).insert((
                Skybox(lx.clone()),
            ));
        }
        match (&spawnP.camera_params.2, &spawnP.camera_params.3) {
            (Some(a), Some(b)) => {
                let ax = asset_server.load(a);
                let bx = asset_server.load(b);
                commands.entity(id).insert((
                    EnvironmentMapLight {
                        diffuse_map: ax.clone(),
                        specular_map: bx.clone(), //todo lol
                        // ..Default::default()
                    },
                ));
            }
            _ => {}
        }
    }

    return;
    {
        // First pass
        let quad_handle = meshes_a.add(Mesh::from(Quad::new(Vec2::new(
            size.width as f32,
            size.height as f32,
        ))));

        let first_pass_material_handle = first_pass_material_a.add(FirstPassMaterial {
            source_image: image_handle.clone(),
        });

        // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: quad_handle.into(),
                material: first_pass_material_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.5),
                    ..Default::default()
                },
                ..Default::default()
            },
            RenderLayers::layer(3),
            markers::PostProcessMeshEntityMarker,
            Name::new("Post process 2d quad #1"),
        ));

        // The post-processing pass camera.
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    target: RenderTarget::Image(image_handle.clone()),
                    hdr: true,
                    // renders after the first main camera which has default value: 0.
                    order: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            RenderLayers::layer(3),
            markers::PlayerBorderPostProcessCamera,
            Name::new("Post process camera #1"),
        ));
    }

    const LEN: u8 = 5;

    for i in 1..=LEN {
        // Second pass (passes)
        let second_quad_handle = meshes_a.add(Mesh::from(Quad::new(Vec2::new(
            size.width as f32,
            size.height as f32,
        ))));

        // This material has the texture that has been rendered.
        let second_material_handle = second_pass_material_a.add(SecondPassMaterial {
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
                    ..Default::default()
                },
                ..Default::default()
            },
            RenderLayers::layer(3 + i),
            markers::PostProcessMeshEntityMarker,
            Name::new("Post process 2d quad #2"),
        ));

        // The post-processing pass camera.
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    // renders after the first main camera which has default value: 0.
                    target: RenderTarget::Image(image_handle.clone()),
                    order: 1 + i as isize,
                    hdr: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            RenderLayers::layer(3 + i),
            markers::PlayerBorderPostProcessCamera,
            Name::new("Post process camera #2"),
        ));
    }

    {
        // Third pass
        let quad_handle = meshes_a.add(Mesh::from(shape::Quad::new(Vec2::new(
            size.width as f32,
            size.height as f32,
        ))));

        // This material has the texture that has been rendered.
        let material_handle = third_pass_material_a.add(ThirdPassMaterial {
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
                    ..Default::default()
                },
                ..Default::default()
            },
            RenderLayers::layer(3 + LEN + 1),
            markers::PostProcessMeshEntityMarker,
            Name::new("Post process 2d quad #3"),
        ));

        // The post-processing pass camera.
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    // renders after the first main camera which has default value: 0.
                    order: 2 + LEN as isize,
                    hdr: true,
                    ..Default::default()
                },
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::None,
                },
                ..Default::default()
            },
            RenderLayers::layer(3 + LEN + 1),
            markers::PlayerBorderPostProcessCamera,
            Name::new("Post process camera #3"),
        ));
    }

    camera_ev_r.clear();
}

// pub fn add_camera(
//     mut commands: Commands,
//     mut event_r: EventReader<events::camera::PlayerCameraAddEvent>,
//     mut asset_server: ResMut<AssetServer>,
// ) {
//     for x in event_r.iter() {
//         let target = if x.image_handle.is_some() {
//             RenderTarget::Image(x.image_handle.clone().unwrap().clone())
//         } else {
//             RenderTarget::Window(bevy::window::WindowRef::Primary)
//         };
//         commands.entity(x.parent_entity).with_children(|parent| {
//             parent.spawn((
//                 PlayerCameraChildBundle {
//                     marker: PlayerCameraChildMarker,
//                     camera: Camera3dBundle {
//                         camera: Camera {
//                             target,
//                             hdr: true,
//                             ..Default::default()
//                         },
//                         projection: bevy::prelude::Projection::Perspective(
//                             bevy::prelude::PerspectiveProjection {
//                                 fov: std::f32::consts::FRAC_PI_2,
//                                 ..Default::default()
//                             },
//                         ),
//                         transform: Transform {
//                             translation: Vec3::Y * 0.8,
//                             ..Default::default()
//                         },
//                         camera_3d: Camera3d {
//                             clear_color: ClearColorConfig::Custom(Color::BLACK),
//                             ..Default::default()
//                         },
//                         ..Default::default()
//                     },
//                     env_map_light: EnvironmentMapLight {
//                         diffuse_map: asset_server
//                             .load("internal/environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
//                         specular_map: asset_server
//                             .load("internal/environment_maps/pisa_specular_rbg9e5_zstd.ktx2"),
//                     },
//                 },
//                 UiCameraConfig { show_ui: false },
//                 // RenderLayers::layer(1),
//                 // RenderLayers::layer(2),
//                 CapturingCameraMarker, //todo remove for second camera
//             ));
//             // Main camera, first to render
//             // commands.spawn((
//             //     Camera3dBundle {
//             //         camera_3d: Camera3d {
//             //             clear_color: ClearColorConfig::Custom(Color::BLACK),
//             //             ..default()
//             //         },
//             //         camera: Camera {
//             //             target: RenderTarget::Image(image_handle.clone()),
//             //             hdr: true,
//             //             ..default()
//             //         },
//             //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
//             //             .looking_at(Vec3::default(), Vec3::Y),
//             //         ..default()
//             //     },
//             //     // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
//             //     // the cost of rendering the UI without any post processing effects.
//             // ));
//         });
//     }
// }
