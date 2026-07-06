# Title
Multithreading and SIMD Vectorization

## Labels
performance, optimization

## Body
**Description**: Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples. Integrate `wide` for vectorizing math operations in the physics pipeline, ensuring a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Assigned Agency Role**:
**Systems Engineer** needs to resolve/issue/test this feature.

**Reference**:
Tier 1 Projects - Multithreading and SIMD Vectorization
