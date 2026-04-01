---
name: Multithreading and SIMD Vectorization
about: Integrate rayon and std::simd for performance scaling.
title: 'Multithreading and SIMD Vectorization'
labels: 'performance, optimization'
assignees: ''
---

## Description
Integrate `rayon` for task-based parallelism and the `wide` crate for vectorizing math operations in the physics pipeline using a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.

## Acceptance Criteria
- Engine scales linearly up to 16 threads on supported hardware. When iterating over multiple mutable SoA arrays in parallel with `rayon` (e.g., in `World::step`), chain `.par_iter_mut().zip(...)` instead of passing a tuple to `.into_par_iter()`.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions using the `wide` crate.
- Thread synchronization does not introduce unresolvable latency.
- Defer SIMD vectorization until the Data-Oriented Design (DOD) refactoring is complete to use a Structure of Arrays (SoA) approach.

## Assigned Agency Role
Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer

## Files to Create/Edit
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

## Reference
Tier 1 Projects - Multithreading and SIMD Vectorization