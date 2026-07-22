# Title
Multithreading Implementation

## Labels
performance, optimization

## Body
**Description**: Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.
- SIMD vectorization utilizes a Structure of Arrays (SoA) approach rather than AoS on individual math primitives.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/world.rs

**Assignment**: Multithreading Implementation needs to be resolved/issued/tested by the Systems Engineer
