/// This file defines simple physics projectile motion.
///
/// Example projectile usage:
///
/// ```
/// use natura::{Projectile, Point, Vector};
/// use std::borrow::{BorrowMut};
/// let fps = 60;
/// let time = &crate::fps(fps);
/// let mut initial_acceleration = Vector { x: 0.0, y: 9.81, z: 0.0 };
/// let mut initial_position = Point { x: 0.0, y: 0.0, z: 0.0 };
/// let mut initial_velocity = Vector { x: 5.0, y: 5.0, z: 0.0 };
/// let mut projectile = Projectile::new(
///     time,
///     initial_position.borrow_mut(),
///     initial_velocity.borrow_mut(),
///     initial_acceleration.borrow_mut());
///
/// // Update on every frame.
/// some_update_loop(|| {
///     let pos:&Point = projectile.update();
/// });
/// ```
///
/// For background on projectile motion see:
/// https://en.wikipedia.org/wiki/Projectile_motion

/// Projectile is the representation of a projectile that has a position on
/// a plane, an acceleration, and velocity.
pub struct Projectile<'a> {
    /// position on a plane
    pos: &'a mut Point,
    /// velocity of the projectile
    vel: &'a mut Vector,
    /// acceleration of projectile
    acc: &'a mut Vector,

    /// delta time (usually engines provides this)
    delta_time: &'a f64,
}

/// Point represents a point containing the x, y, z coordinates of the point on
/// a plane.
#[derive(Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Vector represents a vector carrying a magnitude and a direction. We
/// represent the vector as a point from the origin (0, 0) where the magnitude
/// is the euclidean distance from the origin and the direction is the direction
/// to the point from the origin.
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// GRAVITY is a utility vector that represents gravity in 2D and 3D contexts,
/// assuming that your coordinate plane looks like in 2D or 3D:
///
///   y             y ±z
///   │             │ /
///   │             │/
///   └───── ±x     └───── ±x
///
/// (i.e. origin is located in the bottom-left corner)
pub const GRAVITY: Vector = Vector {
    x: 0.0,
    y: -9.81,
    z: 0.0,
};

/// TERMINAL_GRAVITY is a utility vector that represents gravity where the
/// coordinate plane's origin is on the top-right corner
pub const TERMINAL_GRAVITY: Vector = Vector {
    x: 0.0,
    y: 9.81,
    z: 0.0,
};

impl Projectile<'_> {
    /// new creates a new projectile. It accepts a frame rate and initial
    /// values for [position, velocity, and acceleration. It returns a new projectile.
    ///
    /// # Arguments
    ///
    /// * `delta_time` — delta time
    /// * `initial_position` - initial position
    /// * `initial_velocity` - initial velocity
    /// * `initial_acceleration` - initial acceleration
    ///
    /// # Examples
    ///
    /// ```
    /// use natura::{Projectile, Point, Vector};
    /// use std::borrow::{BorrowMut};
    /// let fps = 60;
    /// let time = &crate::fps(fps);
    /// let mut initial_acceleration = Vector { x: 0.0, y: 9.81, z: 0.0 };
    /// let mut initial_position = Point { x: 0.0, y: 0.0, z: 0.0 };
    /// let mut initial_velocity = Vector { x: 5.0, y: 5.0, z: 0.0 };
    /// let mut projectile = Projectile::new(
    ///     time,
    ///     initial_position.borrow_mut(),
    ///     initial_velocity.borrow_mut(),
    ///     initial_acceleration.borrow_mut());
    /// ```
    pub fn new<'a>(
        delta_time: &'a f64,
        initial_position: &'a mut Point,
        initial_velocity: &'a mut Vector,
        initial_acceleration: &'a mut Vector,
    ) -> Projectile<'a> {
        return Projectile {
            pos: initial_position,
            vel: initial_velocity,
            acc: initial_acceleration,
            delta_time,
        };
    }

    /// update updates the position and velocity values for the given projectile.
    /// call this after calling [Projectile::new] to update values.
    pub fn update(&mut self) -> &Point {
        self.pos.x += self.vel.x * self.delta_time;
        self.pos.y += self.vel.y * self.delta_time;
        self.pos.z += self.vel.z * self.delta_time;

        self.vel.x += self.acc.x * self.delta_time;
        self.vel.y += self.acc.y * self.delta_time;
        self.vel.z += self.acc.z * self.delta_time;

        return self.pos;
    }

    /// position returns the position of the projectile.
    pub fn position(&mut self) -> &Point {
        return self.pos;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, Projectile, Vector};
    use std::borrow::BorrowMut;

    #[test]
    fn test_update_gravity() {
        let fps = 60;
        let time = &crate::fps(fps);
        let mut initial_acceleration = Vector {
            x: 0.0,
            y: 9.81,
            z: 0.0,
        };
        let mut initial_position = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut initial_velocity = Vector {
            x: 5.0,
            y: 5.0,
            z: 0.0,
        };
        let mut projectile = Projectile::new(
            time,
            initial_position.borrow_mut(),
            initial_velocity.borrow_mut(),
            initial_acceleration.borrow_mut(),
        );

        let coordinates = [
            Point {
                x: 5.0,
                y: 9.82,
                z: 0.0,
            },
            Point {
                x: 10.0,
                y: 29.46,
                z: 0.0,
            },
            Point {
                x: 15.0,
                y: 58.90,
                z: 0.0,
            },
            Point {
                x: 20.0,
                y: 98.15,
                z: 0.0,
            },
            Point {
                x: 25.0,
                y: 147.22,
                z: 0.0,
            },
            Point {
                x: 30.0,
                y: 206.09,
                z: 0.0,
            },
            Point {
                x: 35.0,
                y: 274.77,
                z: 0.0,
            },
        ];

        for item in coordinates.iter().enumerate() {
            let (_, c): (usize, &Point) = item;
            let mut pos = &Point::default();

            for _ in 1..fps {
                pos = projectile.update();
            }

            let x1 = relative_eq!(pos.x, c.x, epsilon = 1e-2);
            let y1 = relative_eq!(pos.y, c.y, epsilon = 1e-2);
            assert_eq!(x1, true);
            assert_eq!(y1, true);
        }
    }
}
