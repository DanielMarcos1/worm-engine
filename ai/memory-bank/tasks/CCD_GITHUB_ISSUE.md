# Continuous Collision Detection (CCD)

## Labels
enhancement, physics

## Body
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.

**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Assigned Agency Role**:
Physics Engineer needs to take care of resolving, issuing, and testing this feature.

**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

**Quality Requirements**:
- Must pass `cargo check` cleanly
- Must pass `cargo test` suite
- No background processes in any commands - NEVER append `&`
- No server startup commands - assume development server running
- Include Playwright screenshot testing: `./qa-playwright-capture.sh http://localhost:8000 public/qa-screenshots`
- Images from approved sources (Unsplash, https://picsum.photos/) - NO Pexels (403 errors)
- Iterating multiple mutable SoA arrays in `rayon` must chain `.par_iter_mut().zip(...)`
- WGSL shaders must avoid 16-byte alignment crashes by using flat `array<f32>` and Rust structs must use `#[repr(C)]`, `Pod`, and `Zeroable`.
