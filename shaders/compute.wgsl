@group(0) @binding(0) var<storage, read_write> data: array<f32>;

// A prototype compute shader that just modifies data to demonstrate
// round-trip GPU execution. It treats the flat array as groups of 3 (Vector3d).
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x * 3u;

    // Bounds check to avoid reading/writing past the array limit.
    if (index + 2u < arrayLength(&data)) {
        // Prototype transformation: scale X, Y, Z by 2.0
        data[index + 0u] = data[index + 0u] * 2.0;
        data[index + 1u] = data[index + 1u] * 2.0;
        data[index + 2u] = data[index + 2u] * 2.0;
    }
}
