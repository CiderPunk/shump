#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

#import bevy_shader_utils::simplex_noise_2d::simplex_noise_2d


@fragment
fn fragment(mesh: VertexOutput,) -> @location(0) vec4<f32> {
  var value = simplex_noise_2d(mesh.uv * 100.);
  var col = smoothstep(0.99,1.0, value);
  return vec4(vec3(col),1.0);
}
