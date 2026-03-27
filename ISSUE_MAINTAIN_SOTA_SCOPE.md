---
name: Maintain SOTA Scope for Worm Engine
about: Strategic roadmap and technical requirements to secure State-of-the-Art status while protecting v1.0.0 delivery.
title: 'Strategic Portfolio Plan: SOTA Maintenance and Feature Integration'
labels: 'strategic, sota, roadmap'
assignees: ''
---

# Strategic Portfolio Plan: Worm Engine SOTA Integration (v1.0.0 Roadmap)

## Executive Summary
**Strategic Objectives**: To establish Worm Engine as a State-of-the-Art (SOTA) 3D physics engine within the modern Rust ecosystem, while maintaining the scheduled delivery of our core v1.0.0 features. This involves critical technical upgrades including Data-Oriented Design (DOD), Continuous Collision Detection (CCD), SIMD optimizations, Cross-Platform Determinism, and GPU Acceleration.
**Portfolio Value**: Anticipated 35%+ ROI fueled by increased adoption from modern game studios (especially those using ECS frameworks like Bevy/Flecs) and establishing a competitive edge over legacy engines. Front-loading foundational investments protects long-term maintainability.
**Market Opportunity**: The current market demands hyper-performance, deterministic multiplayer capabilities, and cache-coherent memory layouts. Fulfilling these needs captures top-tier market share.
**Resource Strategy**: Dedicating 30% of R&D capacity to advanced algorithmic optimization (SIMD/Multithreading) and DOD. High-risk features will be implemented as modular extensions to safeguard the primary 1.0.0 delivery timeline.

## Project Portfolio Overview
**Tier 1 Projects** (Strategic Priority):
- **Data-Oriented Design (DOD) & ECS Compatibility**:
  - *Budget*: 25% R&D, *Timeline*: Prioritized for v0.6.0.
  - *Expected ROI*: 40% increase in integration adoption.
  - *Strategic Impact*: Critical architectural pivot. Transition rigid body representations to a Structure of Arrays (SoA) layout. Ensures zero-friction integration with Bevy/Flecs.
  - *Resource allocation and success metrics*: Architecture Lead; API integration time under 2 hours.

- **Multithreading and SIMD Vectorization**:
  - *Budget*: 20% R&D, *Timeline*: v0.6.0 (post-DOD refactor).
  - *Expected ROI*: Secures performance leadership in the Rust physics ecosystem.
  - *Strategic Impact*: Must exclusively use the `wide` crate (e.g., `wide::f32x4`) for stable SIMD operations—*never* `std::simd`. Use `rayon` for task parallelism, utilizing `.par_iter_mut().zip(...)` for SoA data.
  - *Resource allocation and success metrics*: Systems Engineer; linear performance scaling up to 16 threads.

- **Continuous Collision Detection (CCD)**:
  - *Budget*: 15% R&D, *Timeline*: v0.4.0.
  - *Expected ROI*: 20% market share increase in high-speed action titles.
  - *Strategic Impact*: Prevents tunneling at extreme speeds. Implemented as a modular add-on to keep v1.0.0 core unblocked.
  - *Resource allocation and success metrics*: Physics Engineer; 0% tunneling at 1000m/s.

**Tier 2 Projects** (Growth Initiatives):
- **Cross-Platform Determinism**:
  - *Budget*: 10% R&D, *Timeline*: v0.7.0/v1.0.0.
  - *Expected ROI*: 15% premium licensing increase.
  - *Market Impact*: Unlocks competitive multiplayer (rollback netcode). Requires exclusive use of deterministic math libraries like `libm`.
  - *Dependencies and risk assessment*: Requires robust CI testing across distinct architectures.

**Innovation Pipeline**:
- **GPU Acceleration (WGPU Compute Shaders)**:
  - *Experimental initiatives with learning objectives*: WGPU (~v0.19) integration targeting massive scale simulations (fluids, soft-bodies). Use `#[repr(C)]` with `bytemuck` and flat `array<f32>` in WGSL to avoid 16-byte alignment crashes.
  - *Technology adoption and capability development*: Post-v0.6.0 timeline. Modular extension to prevent core CPU pipeline destabilization.

## Resource Allocation Strategy
**Team Capacity**: Cross-functional alignment between Architecture, Systems, Physics, and Graphics engineering roles.
**Skill Development**: Upskilling engineers on safe SoA iteration with `rayon`, stable SIMD vectorization with `wide`, and deterministic math execution using `libm`.
**External Partners**: Early integration tests with Rust ECS maintainers (Bevy/Flecs) and the WGPU community.
**Budget Distribution**: Front-loading investment into the core DOD refactor to minimize future technical debt and reduce long-term maintenance overhead.

## Risk Management and Contingency
**Portfolio Risks**: Scope creep threatening the core v1.0.0 timeline; compilation failures from unstable features; synchronization latency in multithreading.
**Mitigation Strategies**:
  - **Strict Dependency Rules**: Strictly ban `std::simd` to prevent compiler errors; mandate the `wide` crate.
  - **Modular Architecture**: Implement high-risk features (CCD, GPU Acceleration) as optional add-ons.
  - **SoA over AoS**: Defer SIMD vectorization until the DOD refactor is complete to utilize a Structure of Arrays (SoA) approach. Avoid applying AoS SIMD to individual math primitives.
**Contingency Planning**: Fallback to single-threaded logic if `rayon` synchronization overhead becomes unresolvable.
**Success Metrics**: Maintain >25% portfolio ROI, 95% on-time delivery of v1.0.0 core milestones, and top 3 benchmark ranking among Rust physics engines.