---
name: Strategic Portfolio Plan - Worm Engine SOTA Alignment
title: "Strategic Portfolio Plan: Worm Engine SOTA Scope & Architecture"
labels: ["strategic", "portfolio-plan", "sota-alignment", "architecture"]
---

# Strategic Portfolio Plan: Fiscal Year 2024

## Executive Summary
**Strategic Objectives**: Evolve the Worm Engine into a State-of-the-Art (SOTA) 3D physics engine, positioning it as the premier Rust-native solution with full ECS compatibility. This requires executing a strategic transition toward Continuous Collision Detection (CCD), Data-Oriented Design (DOD), SIMD Vectorization, Cross-Platform Determinism, and GPU Acceleration.
**Portfolio Value**: Significant investment in core architectural refactoring and algorithmic optimization, projecting a 25%+ portfolio ROI through increased adoption in modern multiplayer and high-performance simulation markets.
**Market Opportunity**: The modern game engine ecosystem (e.g., Bevy, Flecs) demands highly scalable, deterministic, and data-oriented backends. Capturing this market segment offers a competitive advantage over legacy engines.
**Resource Strategy**: Dedicating engineering capacity to advanced DOD patterns, multithreading, and low-level optimization, while maintaining strict adherence to the 1.0.0 roadmap.

## Project Portfolio Overview
**Tier 1 Projects** (Strategic Priority):
- **Data-Oriented Design (DOD) Refactoring**: Re-architect the `World` struct to delegate state to `RigidBodyComponents` using a Structure of Arrays (SoA) layout. Essential for ECS compatibility and cache-friendly memory access.
  - *Metrics & Goals*: 95% on-time delivery; unlocks scalable SIMD and parallelization.
- **Continuous Collision Detection (CCD)**: Implement time-of-impact (TOI) calculations to eliminate tunneling in high-speed simulations.
  - *Metrics & Goals*: Achieve 0% tunneling at extreme velocities; critical for robust simulation fidelity.

**Tier 2 Projects** (Growth Initiatives):
- **Multithreading & SIMD Vectorization**: Integrate `rayon` (using `.par_iter_mut().zip(...)` for large mutable tuples) and the `wide` crate (strictly avoiding `std::simd` for stable builds) for linear scaling up to 16 threads. Must be deferred until the DOD refactoring establishes an SoA baseline.
  - *Metrics & Goals*: Exponential performance gains in broad-phase and narrow-phase calculations.
- **Cross-Platform Determinism**: Integrate `libm` for deterministic math operations, replacing standard library floats, to support competitive multiplayer and rollback netcode.
  - *Metrics & Goals*: Assure cross-platform consistency across architectures.

**Innovation Pipeline**:
- **GPU Acceleration (WGPU)**: Experimental offloading of compute-heavy tasks to the GPU using `wgpu` (~v0.19) and WGSL compute shaders (utilizing flat `array<f32>` to avoid 16-byte alignment crashes).
  - *Objectives*: Prove viability of massively parallel collision resolution and soft body dynamics.

## Resource Allocation Strategy
**Team Capacity**: Cross-functional alignment of systems engineers, math specialists, and graphics programmers. Focus on core mechanics first, then algorithmic optimization.
**Skill Development**: Upskilling the team in strict SoA patterns, deterministic floating-point operations, and SIMD/Rayon paradigms.
**External Partners**: Early integration testing with leading Rust game engine maintainers to validate ECS compatibility and API ergonomics.
**Budget Distribution**: 40% allocated to Core Architecture (DOD/CCD), 30% to Optimization (SIMD/Threading), 20% to Determinism, and 10% to R&D (GPU Compute).

## Risk Management and Contingency
**Portfolio Risks**: The complexity of CCD and DOD refactoring threatens the 1.0.0 delivery timeline. Using unstable Rust features (like `std::simd`) could break builds. Memory alignment crashes in WGSL.
**Mitigation Strategies**: Strictly mandate the `wide` crate for SIMD to guarantee build stability. Ensure WGSL shaders use flat float arrays. Modularize high-risk features like GPU acceleration as optional engine add-ons.
**Contingency Planning**: If SOTA features jeopardize the 1.0.0 release, delay non-critical optimizations (GPU compute, SIMD) to post-1.0.0 milestones, ensuring core kinematics and dynamics ship on time.
**Success Metrics**: 25%+ portfolio ROI, 95% on-time delivery across core milestones, maintaining a clean, stable codebase ready for widespread industry adoption.