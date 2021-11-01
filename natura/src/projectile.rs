use std::borrow::BorrowMut;

// This file defines simple physics projectile motion.
//
// Example usage:
//
//    // Run once to initialize.
//    projectile := new_projectile(
//        fps(60),
//        Point{6.0, 100.0, 0.0},
//        Vector{2.0, 0.0, 0.0},
//        Vector{2.0, -9.81, 0.0},
//    )
//
//    // update on every frame.
//    someUpdateLoop(func() {
//        pos := projectile.update()
//    })
//
// For background on projectile motion see:
// https://en.wikipedia.org/wiki/Projectile_motion

// Projectile is the representation of a projectile that has a position on
// a plane, an acceleration, and velocity.
pub struct Projectile {
    pos: Point,
    vel: Vector,
    acc: Vector,
    delta_time: f64,
}

// Point represents a point containing the x, y, z coordinates of the point on
// a plane.
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

// Vector represents a vector carrying a magnitude and a direction. We
// represent the vector as a point from the origin (0, 0) where the magnitude
// is the euclidean distance from the origin and the direction is the direction
// to the point from the origin.
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

// GRAVITY is a utility vector that represents gravity in 2D and 3D contexts,
// assuming that your coordinate plane looks like in 2D or 3D:
//
//   y             y ±z
//   │             │ /
//   │             │/
//   └───── ±x     └───── ±x
//
// (i.e. origin is located in the bottom-left corner)
pub const GRAVITY: Vector = Vector { x: 0.0, y: -9.81, z: 0.0 };


// TERMINAL_GRAVITY is a utility vector that represents gravity where the
// coordinate plane's origin is on the top-right corner
pub const TERMINAL_GRAVITY: Vector = Vector { x: 0.0, y: 9.81, z: 0.0 };


impl Projectile {
    // new_projectile creates a new projectile. It accepts a frame rate and initial
// values for position, velocity, and acceleration. It returns a new projectile.
    pub fn new(delta_time: f64,
                          initial_position: Point,
                          initial_velocity: Vector,
                          initial_acceleration: Vector) -> Self {
        return Projectile {
            pos: initial_position,
            vel: initial_velocity,
            acc: initial_acceleration,
            delta_time,
        };
    }

    // update updates the position and velocity values for the given projectile.
    // Call this after calling NewProjectile to update values.
    pub fn update(&mut self) -> &mut Point {
        self.pos.x += self.vel.x * self.delta_time;
        self.pos.y += self.vel.y * self.delta_time;
        self.pos.z += self.vel.z * self.delta_time;

        self.vel.x += self.acc.x * self.delta_time;
        self.vel.y += self.acc.y * self.delta_time;
        self.vel.z += self.acc.z * self.delta_time;

        // todo bug_the_system borrowing
        return self.pos.borrow_mut();
    }

    // position returns the position of the projectile.
    pub fn position(&mut self) -> &mut Point {
        return self.pos.borrow_mut();
    }
}