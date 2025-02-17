use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 40.0;


#[derive(Component)]
struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app:&mut App){
    app
      .add_systems(Startup, spawn_camera);
      //.add_systems(Update, rotate_camera);
  }
}
/*
fn rotate_camera(mut camera:Query<&mut Transform, With<MainCamera>>, time: Res<Time>){
  let cam_transform = camera.single_mut().into_inner();
  cam_transform.rotate_around(
    Vec3::ZERO, 
    Quat::from_axis_angle(Vec3::Y, 45f32.to_radians() * time.delta_seconds()),
  );
  cam_transform.look_at(Vec3::ZERO, Vec3::Z);
}
 */
fn spawn_camera(mut commands:Commands){
  commands.spawn((
    Camera3dBundle{
      transform:Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0)
      .looking_at(Vec3::ZERO, Vec3::Z),
      ..default()
    },
    MainCamera
  ));
}