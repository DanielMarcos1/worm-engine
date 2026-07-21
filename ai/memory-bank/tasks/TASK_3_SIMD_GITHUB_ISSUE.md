# Title
Multithreading and SIMD Vectorization

## Labels
performance, optimization

## Body
**Description**:
Integrate `rayon` for task-based parallelism and `wide` crate for vectorizing math operations in the physics pipeline. When using `rayon` for multithreading over DOD SoA layouts, parallel iteration over multiple mutable arrays must be chained using `.par_iter_mut().zip(...)` instead of passing tuples.

**Assigned Agency Role**:
**Systems Engineer** needs to resolve/issue/test this feature.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions via the `wide` crate.
- Thread synchronization does not introduce unresolvable latency.
- Parallel iterations correctly use `.par_iter_mut().zip(...)`.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**:
Tier 1 Projects - Multithreading and SIMD Vectorization
