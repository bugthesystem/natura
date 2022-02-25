use bevy::prelude::*;
use natura::*;

pub struct NaturaAnimationPlugin {
    delta_time: DeltaTime,
    angular_frequency: AngularFrequency,
    damping_ratio: DampingRatio,
}

// A thing we want to animate.
#[derive(Default, Component)]
pub struct Sprite {
    pub x: f64,
    pub x_velocity: f64,
    pub y: f64,
    pub y_velocity: f64,
}

#[derive(Default)]
pub struct NaturaAnimationBundle {
    pub sprite: Sprite,
    pub spring: Spring,
}

impl NaturaAnimationPlugin {
    pub fn new(delta_time: DeltaTime,
                angular_frequency: AngularFrequency,
                damping_ratio: DampingRatio) -> NaturaAnimationPlugin {
        NaturaAnimationPlugin { delta_time, angular_frequency, damping_ratio }
    }
}

impl Plugin for NaturaAnimationPlugin {
    fn build(&self, app: &mut App) {
        let sprite = Sprite::default();

        // Initialize a spring with frame-rate, angular frequency, and damping values.
        let spring = Spring::new(
            natura::fps(self.delta_time.0),
            self.angular_frequency.0,
            self.damping_ratio.0);
        // add things to your app here
        app.insert_resource(NaturaAnimationBundle { sprite, spring });
    }
}

