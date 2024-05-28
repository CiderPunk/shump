use bevy::{prelude::*, render::render_resource::AsBindGroup};

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app:&mut App){
      app
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(Startup, init_starfield);
    }
}

fn init_starfield(
  mut commands:Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<CustomMaterial>>
){
  commands.spawn(MaterialMeshBundle{
    mesh:meshes.add(Cuboid::default()),
    material: materials.add(CustomMaterial{ 
      //color: Color::BLUE,
      alpha_mode:AlphaMode::Blend,
    }),
    transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(80., 5.0, 80.0)),
    ..default()
  });
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
  //#[uniform(1)]
  //color: Color,
  alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
  fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
    "shaders/starfield.wgsl".into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }
}

