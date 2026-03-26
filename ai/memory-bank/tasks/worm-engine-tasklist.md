# Worm Engine SOTA Task List

## Project Objectives
Implement the following State-of-the-Art (SOTA) features to evolve Worm Engine:
- Continuous Collision Detection (CCD)
- Data-Oriented Design (DOD) & ECS Compatibility
- Multithreading and SIMD Vectorization
- Cross-Platform Determinism Setup
- GPU Acceleration (Compute Shaders) Integration

## Task Specifications

### 1. Continuous Collision Detection (CCD)
Implement CCD to prevent high-velocity tunneling. This involves calculating time of impact (TOI) between moving bodies.
- **Goal:** 0% tunneling observed at velocities up to 1000m/s.
- **Estimated Duration:** 60 minutes
- **Files to Modify:** `src/physics/ccd.rs`, `src/physics/mod.rs`, `src/physics/world.rs`
- **Quality Requirements:** Ensure the feature integrates with existing collision detection and pass all relevant tests via `cargo test` and `cargo check`.
- **Assignment:** Continuous Collision Detection (CCD) Implementation needs to be resolved/issued/tested by the Physics Engineer.

### 2. Data-Oriented Design (DOD) & ECS Refactoring
Refactor core engine structures to support Data-Oriented Design, ensuring compatibility with modern ECS architectures.
- **Goal:** Memory layout optimized for cache coherency; API allows ECS integration in under 2 hours.
- **Estimated Duration:** 60 minutes
- **Files to Modify:** `src/physics/rigid_body.rs`, `src/physics/world.rs`, `src/physics/components.rs`
- **Quality Requirements:** Refactored architecture should pass all internal constraints, validated with `cargo test` and `cargo check`.
- **Assignment:** Data-Oriented Design (DOD) & ECS Refactoring needs to be resolved/issued/tested by the Architecture Lead.

### 3. Multithreading and SIMD Vectorization
Integrate `rayon` and `std::simd` to parallelize execution and vectorize math operations.
- **Goal:** Engine scales linearly up to 16 threads; core math utilizes SIMD instructions.
- **Estimated Duration:** 60 minutes
- **Files to Modify:** `Cargo.toml`, `src/geometry/vector.rs`, `src/physics/world.rs`
- **Quality Requirements:** Validate performance optimizations and test integration using `cargo test` and `cargo check`.
- **Assignment:** Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer.

### 4. Cross-Platform Determinism Setup
Implement strict floating-point math control and deterministic solver execution across architectures.
- **Goal:** Yield identical results on different architectures with a fallback mechanism for non-deterministic math.
- **Estimated Duration:** 60 minutes
- **Files to Modify:** `src/physics/math.rs`, `src/physics/constants.rs`, and deterministic tests.
- **Quality Requirements:** Verify cross-platform stability utilizing custom CI testing scripts, alongside standard `cargo test` and `cargo check`.
- **Assignment:** Cross-Platform Determinism Setup needs to be resolved/issued/tested by the Systems Engineer.

### 5. GPU Acceleration (Compute Shaders) Integration
Integrate WGPU for massive scale simulations using compute shaders (e.g., soft-bodies/fluids).
- **Goal:** Establish a basic WGPU context; pass data from a prototype compute shader back to the CPU pipeline.
- **Estimated Duration:** 60 minutes
- **Files to Modify:** `Cargo.toml`, `src/physics/gpu.rs`, `shaders/compute.wgsl`
- **Quality Requirements:** Ensure compute shader integration successfully completes without CPU pipeline degradation; run `cargo check` and `cargo test` to verify.
- **Assignment:** GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer.
