# Copilot Instructions

This repository contains **Natura**, a Rust spring animation library for smooth, natural motion in 2D and 3D contexts.

## Project Overview

- **natura**: Core library implementing damped harmonic oscillator (spring) physics
- **bevy-natura**: Plugin for integration with the Bevy game engine
- **examples/**: Example applications using coffee, Bevy, and standalone Rust

## Build Commands

```bash
# Build entire workspace
cargo build

# Build only the core library
cargo build -p natura

# Build specific example
cargo run -p simple
cargo run -p coffee-2d
cargo run -p bevy-simple
```

## Test Commands

```bash
# Run all tests
cargo test

# Run tests for core library only
cargo test -p natura
```

## Code Style and Conventions

- Follow standard Rust idioms and the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for code formatting
- Add documentation comments (`///`) for public APIs
- Include doc tests in documentation when illustrating API usage
- Use descriptive variable names that reflect physics terminology (e.g., `angular_frequency`, `damping_ratio`, `equilibrium_pos`)

## Architecture

The core spring animation is based on Ryan Juckett's damped simple harmonic oscillator algorithm. Key types:

- `Spring`: Main struct for computing spring animation coefficients
- `DeltaTime`, `AngularFrequency`, `DampingRatio`: Newtype wrappers for type safety
- `Projectile`: Simulator for projectile/particle motion
- `Point`, `Vector`: Basic geometry types

## Making Changes

- Keep changes minimal and focused
- Maintain backward compatibility for public APIs
- Add tests for new functionality
- Update documentation when modifying public APIs
- Consider both 2D and 3D use cases when making changes to the core library
