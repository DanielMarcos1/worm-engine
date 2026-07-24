# Title
Multithreading and SIMD Vectorization

## Labels
performance, optimization

## Body
Integrate `rayon` for task-based parallelism and the `wide` crate (instead of std::simd) for vectorizing math operations in the physics pipeline over Data-Oriented Design (DOD) Structure of Arrays (SoA) layouts.

**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions via the `wide` crate.
- Thread synchronization does not introduce unresolvable latency.
- Parallel iteration over multiple mutable arrays must be chained using `.par_iter_mut().zip(...)` instead of passing tuples.

Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer.
