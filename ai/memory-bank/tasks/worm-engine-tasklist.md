# Worm Engine Task List

## 📋 Quality Requirements
- Verify changes with `cargo check` and `cargo test` for Rust.
- Ensure that memory layout optimizations are cache coherent.
- Maintain a 0% tunneling rate at velocities up to 1000m/s.
- Avoid compilation regressions and handle `#[repr(C)]` for `bytemuck` accurately.

## 🛠️ Task List

### Tier 1 Projects - Continuous Collision Detection (CCD)
- [ ] 1. Continuous Collision Detection (CCD) Pipeline Integration needs to be resolved/issued/tested by the Physics Engineer
      - Create a modular TOI calculation pipeline for moving bodies.
      - Ensure 0% tunneling observed at velocities up to 1000m/s.
      - Edit `src/physics/ccd.rs`, `src/physics/mod.rs`, and `src/physics/world.rs`.
      - **Time Estimate:** 60 minutes.

### Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility
- [ ] 2. Core Structures DOD Refactoring needs to be resolved/issued/tested by the Architecture Lead
      - Refactor `World` to avoid redundant SoA fields and delegate to `RigidBodyComponents`.
      - Ensure API allows integration with a standard ECS in under 2 hours.
      - Edit `src/physics/rigid_body.rs`, `src/physics/world.rs`, and `src/physics/components.rs`.
      - **Time Estimate:** 60 minutes.

### Tier 1 Projects - Multithreading and SIMD Vectorization
- [ ] 3. SIMD Instruction Vectorization Integration needs to be resolved/issued/tested by the Systems Engineer
      - Integrate `rayon` for task parallelism and `wide::f32x4` (avoiding `std::simd`) for vectorizing math operations in the physics pipeline.
      - Fix `f32x4` import compilation errors in `src/geometry/vector.rs`.
      - Ensure engine scales linearly up to 16 threads.
      - Edit `Cargo.toml`, `src/geometry/vector.rs`, and `src/physics/world.rs`.
      - **Time Estimate:** 60 minutes.

### Tier 2 Projects - Cross-Platform Determinism
- [ ] 4. Cross-Platform Deterministic Execution Setup needs to be resolved/issued/tested by the Systems Engineer
      - Implement strict floating-point math control and deterministic solver execution across architectures.
      - Edit `src/physics/math.rs`, `src/physics/constants.rs`, and related CI testing files.
      - **Time Estimate:** 60 minutes.

### Tier 2 Projects - GPU Acceleration (Compute Shaders)
- [ ] 5. WGPU Compute Shaders Integration needs to be resolved/issued/tested by the Graphics Engineer
      - Future-proof the engine by establishing a basic WGPU context for compute operations.
      - Send `Vector3d` data to WGSL using a flat `array<f32>` to avoid memory alignment crashes.
      - Edit `Cargo.toml`, `src/physics/gpu.rs`, and `shaders/compute.wgsl`.
      - **Time Estimate:** 60 minutes.
