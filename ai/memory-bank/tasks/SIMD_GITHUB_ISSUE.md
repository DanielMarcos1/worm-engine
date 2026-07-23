# Title
Multithreading and SIMD Vectorization Implementation

## Labels
performance, optimization

## Body
**Description**: Integrate `rayon` for task-based parallelism and `wide` for vectorizing math operations in the physics pipeline.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.

**Technical Constraints**:
- When using `rayon` for multithreading over Data-Oriented Design (DOD) Structure of Arrays (SoA) layouts, parallel iteration over multiple mutable arrays must be chained using `.par_iter_mut().zip(...)` instead of passing tuples.
- Integrate `wide` for vectorizing math operations. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach.
- Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.
- Do not use `std::simd`.

**Assignment**:
Multithreading and SIMD Vectorization Implementation needs to be resolved/issued/tested by the Systems Engineer.
