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

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}
 
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
    let pixel_f: vec4<f32> = textureLoad(t_canvas, vec2<i32>(0, 0), 0);
    
    let time_frame = (pixel_f.g * 255.) / 60.;
    let strength = 5.0;
    let decay = strength * exp(-time_frame * 5.0); // fades out over ~1 second
    
    // Convert to centered UV (-1..1)
    var uv = in.uv * 2.0 - 1.0;
    let radius = length(uv);

    // Base distortion frequencies (start fast, slow down)
    let freq = mix(30.0, 5.0, min(time_frame / 1.0, 1.0));

    // Add asymmetry: horizontal and vertical warp differ
    let wave_x = sin(radius * freq + time_frame * 8.0) * 0.015;
    let wave_y = cos(radius * (freq * 0.8) - time_frame * 6.0) * 0.015;

    // Add noise-based irregularity
    let n = noise(uv * 3.0 + vec2<f32>(time_frame * 0.5, time_frame * 0.7)) - 0.5;

    uv.x += decay * (wave_x + n * 0.02);
    uv.y += decay * (wave_y + n * 0.02);

    // Back to 0..1 space
    let uv_tex = uv * 0.5 + 0.5;

    // Chromatic aberration (slightly more irregular than before)
    let chroma_shift = decay * radius * 0.004;
    let colR = textureSample(t_canvas, s_canvas, uv_tex + vec2<f32>( chroma_shift, 0.0)).r;
    let colG = textureSample(t_canvas, s_canvas, uv_tex + vec2<f32>(-chroma_shift * 0.5, 0.0)).g;
    let colB = textureSample(t_canvas, s_canvas, uv_tex + vec2<f32>(-chroma_shift, chroma_shift)).b;
    
    // Scanlines
    let scanline = 0.97 + 0.03 * sin(uv.y * 500.0);

    return vec4<f32>(colR * scanline, colG * scanline, colB * scanline, 1.0);
}
