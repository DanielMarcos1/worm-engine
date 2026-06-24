# Multithreading and SIMD Vectorization
## Labels
performance, optimization
## Body
**Description**: Integrate `rayon` for task-based parallelism and the `wide` crate for vectorizing math operations in the physics pipeline. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions using `wide`.
- Thread synchronization does not introduce unresolvable latency.
- Iterating multiple mutable SoA arrays in `rayon` must chain `.par_iter_mut().zip(...)`

**Assigned Agency Role**: **Systems Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization
