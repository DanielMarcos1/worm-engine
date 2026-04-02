---
name: Data-Oriented Design (DOD) & ECS Refactoring
about: Refactor core engine structures to support Data-Oriented Design.
title: 'Data-Oriented Design (DOD) & ECS Refactoring'
labels: 'architecture, refactoring'
assignees: ''
---

## Description
Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.

## Acceptance Criteria
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

## Assigned Agency Role
Task 2 Data-Oriented Design needs to be resolved/issued/tested by the Architecture Lead.

## Files to Create/Edit
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

## Reference
Data-Oriented Design (DOD) & ECS Compatibility from `ISSUE_STATE_OF_THE_ART.md`