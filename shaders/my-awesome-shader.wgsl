// Global uniform with viewport and tick fields
struct Global {
    camera: vec3<f32>,
    tick: u32,
    viewport: vec2<f32>,
}
 
@group(0) @binding(0)
var<uniform> global: Global;
 
// Vertex input to the shader
struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
};
 
// Output color fragment from the shader
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) uv: vec2<f32>,
};
 
// Main vertex shader function
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(in.pos, 0., 1.);
    out.uv = in.uv;
    return out;
}
 
// Bindings for the texture
@group(1) @binding(0)
var t_canvas: texture_2d<f32>;
 
// Sampler for the texture
@group(1) @binding(1)
var s_canvas: sampler;
 
// Main fragment shader function
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32> = textureSample(t_canvas, s_canvas, in.uv);
    //color = applyColorCycle(color, in.uv, global.tick);
    return color;
}

fn applyColorCycle(color: vec4<f32>, uv: vec2<f32>, tick: u32) -> vec4<f32> {
    let time: f32 = f32(tick) * 0.1;
    let r: f32 = 0.5 + 0.5 * sin(time + uv.x);
    let g: f32 = 0.5 + 0.5 * sin(time + uv.y + 2.0);
    let b: f32 = 0.5 + 0.5 * sin(time + uv.x + 4.0);
    return mix(vec4<f32>(r, g, b, color.a), color, 0.8);
}