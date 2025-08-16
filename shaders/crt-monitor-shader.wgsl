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

fn rand2(n: vec2<f32>) -> f32 {
    return fract(sin(dot(n, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}
 
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
    var uv = in.uv;
    let time_seconds = f32(global.tick) / 60.0;
    let wave = sin(uv.y * 120.0 + time_seconds * 5.0) * 0.0005;
    uv.x += wave;
    
    let pixel_f: vec4<f32> = textureLoad(t_canvas, vec2<i32>(0, 0), 0);
    let shift = pixel_f.g * 255.0 * 0.003;

    // Small random jitter per line
    let jitter = (rand2(vec2<f32>(floor(uv.y * 240.0), time_seconds)) - 0.5) * 0.0005;
    uv.x += jitter;
    
        // Chromatic aberration: sample RGB separately
    let colR = textureSample(t_canvas, s_canvas, uv + vec2<f32>( shift, 0.0)).r;
    let colG = textureSample(t_canvas, s_canvas, uv).g;
    let colB = textureSample(t_canvas, s_canvas, uv + vec2<f32>(-shift, 0.0)).b;

    var color = vec3<f32>(colR, colG, colB);

    // Scanlines
    let scanline = 0.97 + 0.03 * sin(uv.y * 500.0);
    color *= scanline;

    // Slight desaturation
    let gray = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    color = mix(vec3<f32>(gray), color, 0.98);

    return vec4<f32>(color, 1.0);
}

fn applyColorCycle(color: vec4<f32>, uv: vec2<f32>, tick: u32) -> vec4<f32> {
    let time: f32 = f32(tick) * 0.1;
    let r: f32 = 0.5 + 0.5 * sin(time + uv.x);
    let g: f32 = 0.5 + 0.5 * sin(time + uv.y + 2.0);
    let b: f32 = 0.5 + 0.5 * sin(time + uv.x + 4.0);
    return mix(vec4<f32>(r, g, b, color.a), color, 0.8);
}