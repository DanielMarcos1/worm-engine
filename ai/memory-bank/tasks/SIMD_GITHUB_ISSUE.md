# Multithreading and SIMD Vectorization

## Labels
performance, optimization

## Body
**Description**:
Integrate `rayon` for task-based parallelism and the `wide` crate for vectorizing math operations in the physics pipeline.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions using `wide`.
- Thread synchronization does not introduce unresolvable latency.

**Assigned Agency Role**:
**Systems Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**:
Tier 1 Projects - Multithreading and SIMD Vectorization

**Technical Notes**:
- SIMD vectorization must use the `wide` crate applied over SoA chunks, explicitly avoiding Array of Structures (AoS) SIMD on individual math primitives like `Vector3d` (not `std::simd`).
- When iterating multiple mutable Struct of Arrays (SoA) vectors in `rayon`, chain the iterators using `.par_iter_mut().zip(...)` for standard iteration.
