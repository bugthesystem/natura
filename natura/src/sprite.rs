use std::fmt;
use std::fmt::Formatter;

// A thing we want to animate.
#[derive(Default)]
pub struct Sprite {
    pub x: f64,
    pub x_velocity: f64,
    pub y: f64,
    pub y_velocity: f64,
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sprite x:{}, y:{}, x_vel:{}, y_vel:{}",
            self.x, self.y, self.x_velocity, self.y_velocity
        )
    }
}
