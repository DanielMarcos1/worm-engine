# SIMD Vectorization Implementation

## Labels
performance, optimization

## Body
**Description**:
Integrate `wide` for vectorizing math operations in the physics pipeline. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.

**Acceptance Criteria**:
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Assigned Agency Role**:
**Systems Engineer** needs to resolve/issue/test this feature.

**Reference**:
Issue Task 3 SIMD (Part 2 - SIMD)
