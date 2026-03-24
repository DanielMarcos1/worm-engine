@group(0) @binding(0)
var<storage, read_write> velocities: array<f32>;

@group(0) @binding(1)
var<storage, read> accelerations: array<f32>;

@group(0) @binding(2)
var<uniform> dt: f32;

@compute
@workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let vec_index = global_id.x;
    let base_idx = vec_index * 3u;

    // Check out of bounds (length is number of floats)
    if (base_idx + 2u >= arrayLength(&velocities)) {
        return;
    }

    // Read manually because Rust Vector3d is [f32; 3] and WGSL vec3<f32> is aligned to 16 bytes.
    let acc_x = accelerations[base_idx];
    let acc_y = accelerations[base_idx + 1u];
    let acc_z = accelerations[base_idx + 2u];

    let vel_x = velocities[base_idx];
    let vel_y = velocities[base_idx + 1u];
    let vel_z = velocities[base_idx + 2u];

    velocities[base_idx]      = vel_x + acc_x * dt;
    velocities[base_idx + 1u] = vel_y + acc_y * dt;
    velocities[base_idx + 2u] = vel_z + acc_z * dt;
}
