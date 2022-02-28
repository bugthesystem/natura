use bevy::prelude::*;
use natura::{
    Spring,
    DeltaTime,
    AngularFrequency,
    DampingRatio,
    Sprite as NaturaSprite,
};

pub struct NaturaAnimationPlugin {
    delta_time: DeltaTime,
    angular_frequency: AngularFrequency,
    damping_ratio: DampingRatio,
}

#[derive(Default)]
pub struct NaturaAnimationBundle {
    pub sprite: NaturaSprite,
    pub spring: Spring,
}

impl NaturaAnimationPlugin {
    #[must_use]
    pub fn new(
        delta_time: DeltaTime,
        angular_frequency: AngularFrequency,
        damping_ratio: DampingRatio,
    ) -> NaturaAnimationPlugin {
        NaturaAnimationPlugin {
            delta_time,
            angular_frequency,
            damping_ratio,
        }
    }
}

impl Plugin for NaturaAnimationPlugin {
    fn build(&self, app: &mut App) {
        let sprite = NaturaSprite::default();

        // Initialize a spring with frame-rate, angular frequency, and damping values.
        let spring = Spring::new(
            DeltaTime(natura::fps(self.delta_time.0 as u64)),
            self.angular_frequency.clone(),
            self.damping_ratio.clone(),
        );
        // add things to your app here
        app.insert_resource(NaturaAnimationBundle { sprite, spring });
    }
}
