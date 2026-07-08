---
name: SOTA Scope Maintenance
about: Analyze project scope and detail tasks to achieve state-of-the-art level for Worm Engine.
title: 'Strategic Portfolio Plan: Worm Engine Scope & SOTA Alignment'
labels: 'strategic, management'
assignees: ''
---

# Strategic Portfolio Plan: v1.0.0 Roadmap & SOTA Alignment

## Executive Summary
**Strategic Objectives**: Ensure the Worm Engine maintains its v1.0.0 roadmap (Rigid/Soft bodies, Collisions, Integrators) while aggressively evolving into a State-of-the-Art (SOTA) hyper-performance physics solution.
**Portfolio Value**: Projected 35% ROI by securing competitive licensing and integration capabilities, while strictly maintaining our 95% on-time delivery benchmark for core milestones.
**Market Opportunity**: The current physics engine landscape demands hyper-performance, memory efficiency, and deterministic simulation. By adding critical missing architectural features, we position Worm Engine perfectly for the anticipated market shift toward highly scalable, data-oriented multiplayer experiences.
**Resource Strategy**: Mobilize senior systems programming talent and optimize R&D budget for advanced physics algorithms and Rust-native optimizations, ensuring core deliveries are not blocked.

## Project Portfolio Overview
**Tier 1 Projects** (Strategic Priority):
- **Continuous Collision Detection (CCD)**:
  - Budget: 15% R&D, Timeline: v0.4.0, Expected ROI: 20% market share increase, Strategic Impact: Prevents tunneling at high velocities (0% tunneling at 1000m/s).
  - Resource allocation and success metrics: 2 dedicated Physics Engineers; success measured by 0% tunneling at 1000m/s. Must be implemented as a modular add-on to not block v1.0.0.

- **Data-Oriented Design (DOD) & ECS Compatibility**:
  - Budget: 25% R&D, Timeline: v0.6.0, Expected ROI: 40% increase in integration adoption, Strategic Impact: Ensures seamless integration with modern ECS architectures (Bevy, Flecs).
  - Resource allocation and success metrics: 1 Architecture Lead; success measured by API integration time under 2 hours. Front-loading investment to minimize technical debt.

- **Multithreading and SIMD Vectorization**:
  - Budget: 20% R&D, Timeline: v0.6.0, Expected ROI: Secures performance leadership, Strategic Impact: Maximizes CPU utilization using `rayon` and `wide` (avoiding `std::simd` instability).
  - Resource allocation and success metrics: Systems Engineering team; success measured by linear scaling up to 16 threads, leveraging SoA layouts exclusively.

**Tier 2 Projects** (Growth Initiatives):
- **Cross-Platform Determinism**:
  - Budget: 10% R&D, Timeline: v0.7.0/v1.0.0, Expected ROI: 15% premium licensing increase, Market Impact: Essential for competitive multiplayer and rollback netcode.
  - Dependencies and risk assessment: Strict floating-point math control and deterministic solver execution using `libm`. Requires robust CI testing. Risk of extended R&D phase.

**Innovation Pipeline**:
- **GPU Acceleration (Compute Shaders)**:
  - Experimental initiatives with learning objectives: Future-proofing for massive scale simulations (soft-bodies, fluids) via WGPU integration.
  - Technology adoption and capability development: Requires stabilization of the core CPU physics pipeline first. Risk of scope creep, thus kept modular. Vector3d must use `#[repr(C)]` with `Pod`/`Zeroable` derives, and WGSL must use flat `array<f32>` (indexing by 3).

## Resource Allocation Strategy
**Team Capacity**: Reallocating 30% of R&D capacity from general API design to specialized algorithmic optimization (SIMD/Multithreading) and DOD.
**Skill Development**: Upskilling current engineering team in advanced DOD patterns, deterministic mathematics, and parallel execution (`rayon`).
**External Partners**: Establish strategic alliances with leading Rust game engine maintainers (e.g., Bevy, WGPU communities) for early integration testing.
**Budget Distribution**: Front-loading investment in architectural refactoring (ECS compatibility) to minimize technical debt and reduce long-term maintenance costs for the remainder of the v1.0.0 roadmap.

## Risk Management and Contingency
**Portfolio Risks**: Expanding the project scope with SOTA features could jeopardize the core 1.0.0 delivery timeline and increase complexity.
**Mitigation Strategies**: Implement strict agile milestones. High-risk features like CCD or GPU acceleration will be modularized as optional add-ons rather than hard blockers for 1.0.0 core functionality.
**Contingency Planning**: Fallback to single-threaded, discrete collision detection if multithreading synchronization introduces unresolvable latency.
**Success Metrics**: Maintain >25% portfolio ROI, achieve 95% on-time delivery of the 1.0.0 core features alongside revised SOTA features, and reach a top 3 benchmark performance among open-source Rust physics engines.
