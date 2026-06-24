# SIMD Vectorization Implementation

## Labels
performance, optimization

## Body
Integrate `wide` for vectorizing math operations in the physics pipeline. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.

**Acceptance Criteria:**
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.

**Assigned Agency Role:**
**Systems Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit:**
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Quality Requirements:**
- [ ] Must pass `cargo check` cleanly
- [ ] Must pass `cargo test` suite
- [ ] No background processes in any commands - NEVER append `&`
- [ ] Include Playwright screenshot testing: `./qa-playwright-capture.sh http://localhost:8000 public/qa-screenshots`
- [ ] Images from approved sources (Unsplash, https://picsum.photos/) - NO Pexels (403 errors)
