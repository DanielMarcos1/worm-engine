# Elevate Worm Engine to State-of-the-Art (SOTA) Scope

## Labels
strategic, portfolio, enhancement

## Body

### Description
To elevate the Worm Engine from a functional 3D physics engine into a state-of-the-art (SOTA) market leader, we need to carefully manage our project scope. We must expand our capabilities to meet modern ECS-native and high-performance demands while maintaining our core 1.0.0 roadmap commitments.

Our core objectives are:
- Ensure 25% portfolio ROI with 95% on-time delivery benchmark for the core roadmap milestones.
- Prioritize balanced risk across strategic initiatives by treating high-risk features as modular add-ons rather than blockers.

### Strategic Initiatives (What Needs to be Done)

#### Tier 1 Projects (Strategic Priority)
1. **Continuous Collision Detection (CCD)** (v0.4.0)
   - *Scope*: Implement Time of Impact (TOI) calculations to prevent tunneling at high velocities (e.g., 0% tunneling at 1000m/s). Keep it modular to avoid blocking core release.
   - *Budget*: 15% R&D
   - *Expected ROI*: 20% market share increase
   - *Assigned Agency Role*: **Physics Engineer** needs to take care of resolving, issuing, and testing this feature.

2. **Data-Oriented Design (DOD) & ECS Refactoring** (v0.6.0)
   - *Scope*: Refactor core engine structures (rigid body updates) to use flat arrays/SoA to support cache coherency and seamless integration with ECS architectures like Bevy and Flecs.
   - *Budget*: 25% R&D
   - *Expected ROI*: 40% increase in integration adoption
   - *Assigned Agency Role*: **Architecture Lead** needs to take care of resolving, issuing, and testing this feature.

3. **Multithreading and SIMD Vectorization** (v0.6.0)
   - *Scope*: Maximize CPU utilization by integrating `rayon` for task-based parallelism and `wide` over SoA chunks. Target linear scaling up to 16 threads.
   - *Budget*: 20% R&D
   - *Expected ROI*: Secures performance leadership
   - *Assigned Agency Role*: **Systems Engineer** needs to take care of resolving, issuing, and testing this feature.

#### Tier 2 Projects (Growth Initiatives)
4. **Cross-Platform Determinism** (v0.7.0/v1.0.0)
   - *Scope*: Implement strict floating-point math control via `libm` to ensure identical simulation results across CPU architectures, essential for competitive multiplayer and rollback netcode.
   - *Budget*: 10% R&D
   - *Expected ROI*: 15% premium licensing increase
   - *Assigned Agency Role*: **Systems Engineer** needs to take care of resolving, issuing, and testing this feature.

5. **GPU Acceleration (Compute Shaders)** (v0.8.0/v1.0.0)
   - *Scope*: Integrate `wgpu` (~v0.19) for massive scale simulations. Maintain strict adherence to flat `array<f32>` structures to prevent 16-byte memory alignment crashes.
   - *Budget*: Experimental initiatives with learning objectives
   - *Expected ROI*: Future-proofing for massive scale simulations
   - *Assigned Agency Role*: **Graphics Engineer** needs to take care of resolving, issuing, and testing this feature.

### Assigned Agency Role
**Studio Producer** needs to take care of resolving, issuing, and testing this feature.
