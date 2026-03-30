---
name: Worm Engine SOTA Scope Issue
title: "Strategic Portfolio Plan: Worm Engine Scope and State of the Art (SOTA) Level"
labels: ["strategic", "portfolio", "sota", "architecture"]
---

# Strategic Portfolio Plan: 2023-Q4 / v1.0.0 Roadmap

## Executive Summary
**Strategic Objectives**: Evolve the Worm Engine into a State-of-the-Art (SOTA) 3D physics engine capable of delivering hyper-performance, memory efficiency, and deterministic simulation. The objective is to achieve competitive parity and surpass industry leaders by adopting Data-Oriented Design (DOD) via Structure of Arrays (SoA) layout instead of Array of Structures (AoS), leveraging the `wide` crate for cross-platform SIMD vectorization instead of the unstable `std::simd`, and supporting seamless ECS architecture integration.
**Portfolio Value**: Securing a 25%+ expected portfolio ROI through competitive licensing and deep integration capabilities across the Rust ecosystem, including Bevy and Flecs.
**Market Opportunity**: The simulation and gaming markets are shifting towards highly scalable, data-oriented multiplayer experiences. By filling critical architectural gaps, we secure a position among the top 3 open-source Rust physics engines.
**Resource Strategy**: Mobilize senior systems programming talent to execute advanced physics algorithms (e.g., Continuous Collision Detection (CCD), DOD, deterministic math via `libm`), parallel processing via `rayon`, and GPU compute via `wgpu`. Strict risk management ensures we maintain a 95% on-time delivery metric for the core 1.0.0 features.

## Project Portfolio Overview
**Tier 1 Projects** (Strategic Priority):
- **Data-Oriented Design (DOD) & ECS Compatibility Refactor**: [Budget: 25% R&D, Timeline: v0.6.0, Expected ROI: 40% increase in integration adoption, Strategic Impact: Transitioning from AoS to SoA via `RigidBodyComponents`. Enables seamless integration with ECS and prepares the engine for efficient SIMD. Reduces API integration time to under 2 hours.]
- *Resource allocation and success metrics*: 1 architecture lead. Success measured by completion of SoA refactoring and validated API integration without performance overhead from redundant properties on the `World` struct.

- **Multithreading and SIMD Vectorization**: [Budget: 20% R&D, Timeline: v0.6.0, Expected ROI: Performance leadership in Rust physics, Strategic Impact: Leveraging `rayon` for task-based parallelism (.par_iter_mut().zip() over SoA) and the `wide` crate for stable SIMD vectorization. Avoids anti-patterns of AoS SIMD on individual primitives like `Vector3d`.]
- *Resource allocation and success metrics*: Systems Engineering team. Success measured by linear scaling up to 16 threads and compilation success using stable Rust tools (strictly no `std::simd`).

- **Continuous Collision Detection (CCD)**: [Budget: 15% R&D, Timeline: v0.4.0, Expected ROI: 20% market share increase, Strategic Impact: Solves tunneling at high velocities, essential for AAA fast-paced action games.]
- *Resource allocation and success metrics*: 2 physics engineers. Success measured by 0% tunneling at 1000m/s velocities, utilizing `Vector3d::length_squared()` for efficient distance comparisons.

**Tier 2 Projects** (Growth Initiatives):
- **Cross-Platform Determinism**: [Budget: 10% R&D, Timeline: v0.7.0/v1.0.0, Expected ROI: 15% premium licensing increase, Market Impact: Critical for competitive multiplayer and rollback netcode.]
- *Dependencies and risk assessment*: Strict adherence to `libm` for deterministic math. High complexity risk requiring robust cross-architecture CI testing.

**Innovation Pipeline**:
- **GPU Acceleration (Compute Shaders)**: [Experimental initiatives with learning objectives: Scaling massive simulations (soft-bodies, fluids) via `wgpu` (~v0.19) and WGSL.]
- *Technology adoption and capability development*: Requires structured flat arrays (`array<f32>`) in WGSL and `#[repr(C)]` with `Pod` and `Zeroable` derives in Rust (`bytemuck`) to avoid 16-byte memory alignment crashes. Modularized to avoid blocking core 1.0.0 delivery.

## Resource Allocation Strategy
**Team Capacity**: Shift 30% of engineering resources to specialized algorithmic optimization (SIMD/Multithreading) and SoA/DOD implementation.
**Skill Development**: Upskill teams in deterministic math (`libm`), WGSL memory alignment constraints, and task-based parallel iteration with `rayon` (chaining `.par_iter_mut().zip(...)` rather than large mutable tuples).
**External Partners**: Deepen ties with `wgpu`, `wide`, and major Rust game engine communities (e.g., Bevy).
**Budget Distribution**: Front-load investments in DOD architectural foundations to prevent future technical debt and lower long-term maintenance costs.

## Risk Management and Contingency
**Portfolio Risks**: Expanding scope to SOTA features (e.g., CCD, GPU acceleration) could risk the 1.0.0 core delivery timeline.
**Mitigation Strategies**: High-risk features (CCD, GPU acceleration) will be built modularly, allowing fallback to single-threaded discrete collision detection or CPU computation without delaying v1.0.0.
**Contingency Planning**: If `rayon` multi-threading introduces sync latency, fallback to single-threaded SoA execution.
**Success Metrics**: Maintain 25%+ portfolio ROI; achieve 95% on-time delivery; secure top 3 performance benchmarking in the Rust ecosystem.

---
**Studio Producer**: Executive Creative Strategist
**Review Date**: 2023-10-26
**Strategic Leadership**: Executive-level vision with operational excellence
**Portfolio ROI**: 25%+ return with balanced risk management
