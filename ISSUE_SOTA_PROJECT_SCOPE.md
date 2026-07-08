---
name: Maintain SOTA Scope and Core 1.0.0 Delivery
about: Actions required to keep project scope and achieve state of the art level
title: 'Strategic Alignment: Maintain SOTA Scope and Core 1.0.0 Delivery'
labels: 'strategic, enhancement'
assignees: ''
---

## Description
To ensure the Worm Engine accomplishes its State of the Art (SOTA) level while maintaining the project scope and a 95% on-time delivery benchmark for our core v1.0.0 roadmap, we need to implement advanced features as modular add-ons. This will secure our target of 25%+ portfolio ROI without jeopardizing existing milestones.

## What Needs to Be Done
1. **Deliver Core v1.0.0 Roadmap**: Continue execution of Rigid/Soft Body Dynamics, Kinematics, Collision Detection, Constraints, and Integrators.
2. **Implement Tier 1 SOTA Features as Modular Add-ons**:
   - **Continuous Collision Detection (CCD)**: Implement TOI calculation to ensure 0% tunneling at 1000m/s.
   - **Data-Oriented Design (DOD) & ECS Refactoring**: Transition to flat arrays/SoA to guarantee under 2-hour API integration with modern ECS (Bevy, Flecs).
   - **Multithreading & SIMD**: Maximize CPU utilization using `rayon` (linear scaling up to 16 threads) and `wide` (for SoA SIMD).
3. **Manage Tier 2 Experimental Initiatives**: Keep Cross-Platform Determinism (`libm`) and GPU Acceleration (`wgpu`/WGSL) as experimental pipelines to avoid scope creep.

## Assigned Agency Role
**Architecture Lead** needs to resolve/issue/test this strategic alignment.
