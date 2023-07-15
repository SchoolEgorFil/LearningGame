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
    let Ok(mut camera) = main_camera.get_single_mut() else {
        return;
    };

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
