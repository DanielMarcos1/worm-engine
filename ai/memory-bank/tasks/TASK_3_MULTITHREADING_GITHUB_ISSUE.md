# Multithreading Implementation

## Labels
performance, optimization, v0.6.0

## Body
### Description
Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.

### Acceptance Criteria
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.
- SIMD vectorization utilizes a Structure of Arrays (SoA) approach rather than AoS on individual math primitives.

### Technical Constraints
- Iterating multiple mutable SoA arrays in `rayon` must chain `.par_iter_mut().zip(...)` or `.par_chunks_mut(chunk_size).zip(...)` for SIMD chunks.
- Fallback to single-threaded, discrete collision detection if multithreading synchronization introduces unresolvable latency.

### Assigned Agency Role
**Systems Engineer** needs to take care of resolving, issuing, and testing the feature.

### Files to Create/Edit
- Cargo.toml
- src/physics/world.rs

### Project Specs
- **Budget**: [Budget]
- **Timeline**: v0.6.0
- **Expected ROI**: [Expected ROI]
- **Resource Allocation**: [Resource Allocation]
