# Task 1: Continuous Collision Detection (CCD)
## Labels
enhancement, physics
## Body
### Description
Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.

### Acceptance Criteria
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

### Files to Create/Edit
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

### Reference
Issue Task 1 CCD

### Assigned Agency Role
Continuous Collision Detection (CCD) needs to be resolved/issued/tested by the **Physics Engineer**.
