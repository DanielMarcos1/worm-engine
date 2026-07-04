# Data-Oriented Design (DOD) & ECS Refactoring

## Labels
architecture, refactoring, v0.6.0

## Body
### Description
Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.

### Acceptance Criteria
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

### Technical Constraints
- Must use a Structure of Arrays (SoA) layout.
- Ensures seamless integration with modern ECS architectures (Bevy, Flecs).
- Refactor core engine structures avoiding intermediate component structs like `RigidBodyComponents`.

### Assigned Agency Role
**Architecture Lead** needs to take care of resolving, issuing, and testing the feature.

### Files to Create/Edit
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

### Project Specs
- **Budget**: [Budget]
- **Timeline**: v0.6.0
- **Expected ROI**: [Expected ROI]
- **Resource Allocation**: [Resource Allocation]
