mod camera;
mod starfield;

use bevy::prelude::*;
use bevy_shader_utils::ShaderUtilsPlugin;
use camera::CameraPlugin;
use starfield::StarfieldPlugin;


fn main() {
  App::new()
      // bevy built-ins
      .insert_resource(ClearColor(Color::rgb(0.1,0.0,0.15)))
    
    
    
      .insert_resource(AmbientLight{
        color: Color::default(),
        brightness: 750.0,
      })
      .add_plugins((
        DefaultPlugins,     
        ShaderUtilsPlugin,
      ))
      .add_plugins(CameraPlugin)
      .add_plugins(StarfieldPlugin)
      .run();
}
