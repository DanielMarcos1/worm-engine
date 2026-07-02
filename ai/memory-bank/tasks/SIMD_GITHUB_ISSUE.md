# Multithreading and SIMD Vectorization
## Labels
performance, optimization
## Body
**Description**:
Integrate `rayon` for task-based parallelism and the `wide` crate for vectorizing math operations in the physics pipeline. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)`.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions leveraging the `wide` crate over Structure of Arrays (SoA) chunks.
- Explicitly avoid Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.
- Avoid `std::simd`.
- Thread synchronization does not introduce unresolvable latency.

**Assigned Agency Role**:
**Systems Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**:
Tier 1 Projects - Multithreading and SIMD Vectorization
