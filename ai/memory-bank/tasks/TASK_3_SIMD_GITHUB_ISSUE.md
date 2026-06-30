# Multithreading and SIMD Vectorization

## Labels
performance, optimization

## Body
**Description**:
Integrate `rayon` for task-based parallelism and the `wide` crate for vectorizing math operations in the physics pipeline. The project uses a Data-Oriented Design (DOD) with a Structure of Arrays (SoA) layout. SIMD vectorization using the `wide` crate must be applied over SoA chunks, explicitly avoiding Array of Structures (AoS) SIMD on individual math primitives like `Vector3d`. When iterating multiple mutable Struct of Arrays (SoA) vectors in `rayon`, chain the iterators using `.par_iter_mut().zip(...)` for standard iteration, or `.par_chunks_mut(chunk_size).zip(...)` when processing SIMD chunks. Note that `par_iter_mut()` takes exactly zero arguments.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions via the `wide` crate.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.
- Thread synchronization does not introduce unresolvable latency.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**:
Tier 1 Projects - Multithreading and SIMD Vectorization

### Assigned Agency Role
Systems Engineer needs to take care of resolving, issuing, and testing the feature.
