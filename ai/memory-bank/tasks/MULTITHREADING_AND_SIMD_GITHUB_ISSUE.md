# Multithreading and SIMD Vectorization
## Labels
performance, optimization
## Body
### Description
Integrate `rayon` for task-based parallelism and `std::simd` for vectorizing math operations in the physics pipeline. Ensure task is scoped to be implementable by a developer in 30-60 minutes.

### Acceptance Criteria
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.

### Assigned Agency Role
**Systems Engineer** needs to resolve/issue/test this feature.

### Files to Create/Edit
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs
