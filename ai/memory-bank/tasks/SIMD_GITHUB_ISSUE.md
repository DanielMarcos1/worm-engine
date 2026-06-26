# SIMD Vectorization Implementation
## Labels
performance, optimization
## Body
### Description
Integrate vectorizing math operations in the physics pipeline. Must be deferred until DOD refactoring is complete to use a Structure of Arrays (SoA) approach.

### Acceptance Criteria
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.

### Files to Create/Edit
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

### Technical Constraints
- Must strictly use the `wide` crate (NOT `std::simd`).
- SIMD vectorization using the `wide` crate must be applied over SoA chunks, explicitly avoiding Array of Structures (AoS) SIMD on individual math primitives like `Vector3d`.
- When processing SIMD chunks in `rayon`, use `.par_chunks_mut(chunk_size).zip(...)`.

### Assigned Agency Role
**Systems Engineer** needs to take care of resolving, issuing, and testing this feature.
