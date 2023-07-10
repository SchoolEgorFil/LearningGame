// pub fn gltf_load_sun(
//     mut commands: Commands,
//     gltf_obj: Query<(Entity, &Name, &Transform, Option<&Children>), Without<Inserted>>,
//     gltf_m: Query<Entity, (With<Parent>, With<Handle<Mesh>>)>,
//     asset_server: Res<AssetServer>,
// ) {
//     for m in gltf_obj.iter() {
//         if m.1.as_str().contains(TAGS::SUN) {
//             commands
//                 .spawn(DirectionalLightBundle {
//                     directional_light: DirectionalLight {
//                         shadows_enabled: true,
//                         illuminance: 32000.0,
//                         ..Default::default()
//                     },
//                     // This is a relatively small scene, so use tighter shadow
//                     // cascade bounds than the default for better quality.
//                     // We also adjusted the shadow map to be larger since we're
//                     // only using a single cascade.
//                     cascade_shadow_config: CascadeShadowConfigBuilder {
//                         num_cascades: 2,
//                         maximum_distance: 126.,
//                         ..Default::default()
//                     }
//                     .into(),
//                     ..Default::default()
//                 })
//                 .insert(TransformBundle::from_transform(Transform::from_rotation(
//                     m.2.clone().rotation,
//                 )));
//             commands.entity(m.0).despawn();
//         } else if m.1.as_str().contains(TAGS::SKYBOX) && m.3.is_some() {
//             m.3.unwrap().iter().for_each(|e| {
//                 if gltf_m.contains(e.clone()) {
//                     commands.entity(e.clone()).insert(NotShadowCaster);
//                 }
//             });
//         }
//     }
// }
