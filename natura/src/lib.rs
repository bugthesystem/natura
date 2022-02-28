//! crate natura is a set of physics-based animation tools for 2D and 3D
//! applications. There's a spring animation simulator for smooth, realistic
//! motion and a projectile simulator well suited for projectiles and particles.
//!
//! //! # Examples
//!
//! Spring usage:
//!```
//! use natura::{Spring, Vector, Point, DeltaTime, AngularFrequency, DampingRatio};
//! // Run once to initialize.
//! let mut spring = Spring::new(DeltaTime(natura::fps(60)), AngularFrequency(6.0), DampingRatio(0.5));
//!
//! // Update on every frame.
//! let mut pos = 0.0;
//! let mut velocity = 0.0;
//! const TARGET_POS:f64 = 100.0;
//! some_update_loop(|| {
//!    let (pos_new, velocity_new) = spring.update(pos, velocity, TARGET_POS);
//! });
//!```
//!
//! Projectile usage:
//!
//! ```
//! use natura::{Projectile, Point, Vector};
//! use std::borrow::{BorrowMut};
//! let fps = 60;
//! let time = &crate::fps(fps);
//! let mut initial_acceleration = Vector { x: 0.0, y: 9.81, z: 0.0 };
//! let mut initial_position = Point { x: 0.0, y: 0.0, z: 0.0 };
//! let mut initial_velocity = Vector { x: 5.0, y: 5.0, z: 0.0 };
//! let mut projectile = Projectile::new(
//!     time,
//!     initial_position.borrow_mut(),
//!     initial_velocity.borrow_mut(),
//!     initial_acceleration.borrow_mut());
//!
//! // Update on every frame.
//! some_update_loop(|| {
//!     let pos:&Point = projectile.update();
//! });
//! ```
mod projectile;
mod spring;
mod sprite;

pub use projectile::*;
pub use spring::*;
pub use sprite::*;

#[cfg(test)]
#[macro_use]
extern crate approx;
