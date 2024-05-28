#import bevy_pbr::{
    mesh_view_bindings::globals,
    utils::hsv2rgb,
    forward_io::VertexOutput,
}

//@group(0) @binding(0) var<uniform> globals: Globals;
//@group(2) @binding(1) var<uniform> material_color: vec4<f32>;

fn Rot(a:f32)-> mat2x2<f32>{
  let s = sin(a);
  let c = cos(a);
  return mat2x2(c,-s,s,c);
}


fn Star(uv:vec2f, flare:f32 )->f32{
  let len:f32 = length(uv);
  let ruv= uv * Rot(3.14192 / 4);
  //let m = smoothstep(.2,.5,len);
  var m = 0.003 / len;
  m += (((0.2 * (max(0., 1.- abs(uv.x * uv.y *2000.0)))) + (0.05 * max(0., 1.- abs(ruv.x * ruv.y *2000.0)))) * flare);
  return m * smoothstep(1.0,.3, len);
}

fn Hash21(p:vec2f)->f32{
  let q = fract(p*vec2f(46407.4789,235.789));
  let r = q + dot(q,q+45.32);
  return fract(r.x * r.y);
}

fn Layer(p:vec2f, scale:f32, flarechance:f32)->vec3f{
 let suv:vec2f = p * scale;
  //let col = Star(suv, 0.2);
  let gv = fract(suv) - 0.5;
  let id = floor(suv);
  //let rnum = Hash21(id);
  var  col = vec3f();

  for (var x=-1; x<=1; x++){
    for (var y=-1; y<=1; y++){
      let offs = vec2<f32>(f32(x),f32(y));  
      let rnum = Hash21(id + offs);
      let size = fract(rnum * 13459.487);
      let tint = hsv2rgb(fract(rnum * 645.81677),0.5,1.0);
      col += tint * (Star(gv - offs - vec2f(rnum, fract(rnum * 357.74)) + 0.5, smoothstep(flarechance,1.,size)) * size);
    }
  }

  return col;
}



@fragment
fn fragment(mesh: VertexOutput,) -> @location(0) vec4<f32> {
  let p:vec2f = mesh.uv - 0.5;
  var col = vec3f();
  col += Layer(p + vec2(0, globals.time * 0.023), 8.0, 0.95) * 0.5;
  col += Layer(p + vec2(-2.379, globals.time * 0.01), 12.0, 0.97) * 0.3;
  col += Layer(p + vec2(2.657, globals.time * 0.003), 18.0, 0.99) *0.1 ;
  return vec4(vec3(col),1.0);
}
