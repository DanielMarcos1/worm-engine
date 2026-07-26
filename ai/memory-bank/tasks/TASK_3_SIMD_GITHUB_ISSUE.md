# Title
Multithreading and SIMD Vectorization

## Labels
performance, optimization

## Body
Integrate `rayon` for task-based parallelism and the `wide` crate for vectorizing math operations in the physics pipeline over Data-Oriented Design (DOD) Structure of Arrays (SoA) layouts. Do not use `std::simd` as it violates established technical memory constraints. Ensure that parallel iteration over multiple mutable arrays is chained using `.par_iter_mut().zip(...)` instead of passing tuples.

### Acceptance Criteria
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions via `wide`.
- Thread synchronization does not introduce unresolvable latency.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.

### Assigned Agency Role
Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer

### Files to Create/Edit
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

### Reference
Tier 1 Projects - Multithreading and SIMD Vectorization
