struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

@group(0) @binding(0) var input: texture_storage_2d<r32float, write>;
@group(0) @binding(1) var output: texture_storage_2d<r32float, write>;

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn randomFloat(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

const R: f32 = 15.0;      
const T: f32 = 20.0;
const dt: f32 = 1.0/T;
const mu: f32 = 0.14;
const sigma: f32 = 0.014;
const rho: f32 = 0.5;
const omega: f32 = 0.15;

fn bell(x: f32, m: f32, s: f32) -> f32 {
    return exp(-(x - m)*(x-m)/s/s/2.0);
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    
    if location.x > 128 && location.y > 128 && location.x < 384 && location.y < 384 {
        let randomNumber = randomFloat(invocation_id.y << 16u | invocation_id.x);
        let color = vec4<f32>(f32(randomNumber));
        textureStore(output, location, color);
    } else {
        let color = vec4<f32>(0.0);
        textureStore(output, location, color);
    }

}

// @compute @workgroup_size(8, 8, 1)
// fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
//     let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

//     var sum: f32 = 0.0;
//     var total: f32 = 0.0;
//     for (var x: i32 = -i32(R); x<=i32(R); x++) {
//         for (var y: i32 = -i32(R); y<=i32(R); y++)
//         {
//             let r: f32 = sqrt(f32(x*x + y*y)) / R;
//             let txy: vec2<i32> = (location + vec2<i32>(x, y));
//             let val: f32 = textureLoad(input, txy).r;
//             let weight: f32 = bell(r, rho, omega);
//             sum += val * weight;
//             total += weight;
//         }
//     }

//     let avg: f32 = sum / total;
//     let g: f32 = bell(avg, mu, sigma) * 2.0 - 1.0;
//     let val: f32 = textureLoad(input, location).r;
//     let c: f32 = clamp(val + dt * g, 0.0, 1.0);

//     let color = vec4<f32>(c, c, c, 1.0);

//     textureStore(output, location, color);
// }

