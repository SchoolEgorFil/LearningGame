use bevy::prelude::Component;

//
// PLAYER
//
// parent
//  |- container
//      |- main
//      |- border post process
//

#[derive(Component)]
pub struct PlayerParentMarker;

#[derive(Component)]
pub struct PlayerCameraContainerMarker;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct PlayerMainCamera;

#[derive(Component)]
pub struct PlayerBorderPostProcessCamera;

//
// Outline Post Process
//
//
#[derive(Component)]
pub struct PostProcessMeshEntityMarker;

//
// GLTF
//
#[derive(Component)]
pub struct ExploredGLTFObjectMarker;

// light
#[derive(Component)]
pub struct ExploredLightObjectMarker;
