# Multithreading and SIMD Vectorization

## Labels
performance, optimization

## Body
**Description**: Integrate `rayon` for task-based parallelism and `std::simd` (or `wide` crate for SoA) for vectorizing math operations in the physics pipeline. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples. Defer SIMD vectorization until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization

### Assigned Agency Role
**Systems Engineer** needs to resolve/issue/test this feature.
