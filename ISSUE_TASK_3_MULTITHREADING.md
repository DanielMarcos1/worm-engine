---
name: Multithreading via rayon
about: Integrate rayon for task-based parallelism.
title: 'Multithreading via rayon'
labels: 'performance, optimization'
assignees: ''
---

## Description
Integrate `rayon` for task-based parallelism to scale linearly up to 16 threads.

## Acceptance Criteria
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.
- Iterating over multiple mutable SoA arrays in `rayon` chains `.par_iter_mut().zip(...)` or `.par_chunks_mut(4).zip(...)` instead of passing a tuple.

## Assigned Agency Role
Task 3 Multithreading needs to be resolved/issued/tested by the Systems Engineer.

## Files to Create/Edit
- Cargo.toml
- src/physics/world.rs

## Reference
Multithreading and SIMD Vectorization from `ISSUE_STATE_OF_THE_ART.md`