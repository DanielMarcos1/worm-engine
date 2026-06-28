# Strategic SOTA Alignment: Worm Engine Project Scope Expansion

## Labels
strategic, enhancement, sota, project-management

## Body
**Strategic Objectives**: Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art (SOTA), high-performance solution. Align technical milestones to deliver hyper-performance, deterministic simulation, and memory efficiency without derailing the 1.0.0 roadmap (95% on-time delivery benchmark).

### Tier 1 Strategic Initiatives (Priority)
1. **Continuous Collision Detection (CCD)**
   - *Requirement*: Prevent "tunneling" at high velocities (0% tunneling at 1000m/s).
   - *Scope*: Implement as a modular add-on to existing collision systems.

2. **Data-Oriented Design (DOD) & ECS Compatibility**
   - *Requirement*: Refactor core engine structures to flat arrays/SoA.
   - *Scope*: Ensure memory layout is optimized for cache coherency and API allows standard ECS (Bevy, Flecs) integration under 2 hours.

3. **Multithreading and SIMD Vectorization**
   - *Requirement*: Integrate task-based parallelism and SIMD instructions.
   - *Technical Constraints*:
     - Use `rayon` for parallel iteration, explicitly chaining `.par_iter_mut().zip(...)` over large mutable SoA arrays in `World::step`.
     - Defer SIMD until DOD is complete. Use the `wide` crate for SoA optimization, explicitly avoiding `std::simd` and avoiding AoS on individual math primitives like `Vector3d`.
   - *Scope*: Scale linearly up to 16 threads.

### Tier 2 Growth Initiatives (Experimental/Future-Proofing)
4. **Cross-Platform Determinism**
   - *Requirement*: Implement strict floating-point math control via `libm` and deterministic solver execution across architectures.
   - *Scope*: Robust CI testing and fallback mechanisms for non-deterministic math.

5. **GPU Acceleration (Compute Shaders)**
   - *Requirement*: Integrate `wgpu` (~v0.19) and WGSL for massive scale simulations.
   - *Technical Constraints*:
     - Avoid 16-byte alignment crashes in WGSL by using flat `array<f32>` (indexing by 3).
     - Data sent via `bytemuck` (e.g., `Vector3d`) must use `#[repr(C)]` with `Pod` and `Zeroable` derives.

### Assigned Agency Roles
- **Physics Engineer**: Implement Continuous Collision Detection (CCD) to resolve high-speed tunneling.
- **Architecture Lead**: Execute DOD & ECS Refactoring to transition to flat arrays/SoA layouts.
- **Systems Engineer**: Implement Multithreading (`rayon`), SIMD Vectorization (`wide` crate over SoA), and Cross-Platform Determinism (`libm`).
- **Graphics Engineer**: Integrate GPU Acceleration (Compute Shaders) with `wgpu` and handle memory alignment constraints.
