# Title
Multithreading and SIMD Vectorization

## Labels
performance, optimization

## Body
**Description**: Integrate `rayon` for task-based parallelism and `wide` for vectorizing math operations in the physics pipeline. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.
- Iterating multiple mutable SoA arrays in `rayon` must chain `.par_iter_mut().zip(...)`.

**Assigned Agency Role**:
Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization
