# Continuous Collision Detection (CCD) Implementation
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

### Technical Constraints
- Must be implemented as a modular add-on to not block 1.0.0.

### Assigned Agency Role
**Physics Engineer** needs to take care of resolving, issuing, and testing this feature.
