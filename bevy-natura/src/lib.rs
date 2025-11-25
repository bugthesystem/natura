use bevy::prelude::*;
use natura::{Spring, Sprite as NaturaSpriteCore};

/// Plugin that enables Natura spring animations for multiple entities.
/// 
/// Unlike the previous implementation which only supported a single sprite,
/// this plugin works with Bevy's ECS pattern, allowing each entity to have
/// its own spring animation state.
/// 
/// # Usage
/// 
/// ```rust,ignore
/// use bevy::prelude::*;
/// use bevy_natura::{NaturaAnimationPlugin, NaturaSpringBundle, NaturaTarget};
/// use natura::{DeltaTime, AngularFrequency, DampingRatio};
/// 
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(NaturaAnimationPlugin)
///         .run();
/// }
/// 
/// fn setup(mut commands: Commands) {
///     // Spawn multiple entities with spring animations
///     commands.spawn((
///         SpriteBundle { /* ... */ },
///         NaturaSpringBundle::new(
///             DeltaTime(60.0),
///             AngularFrequency(6.0),
///             DampingRatio(0.7),
///         ),
///         NaturaTarget { x: 100.0, y: 200.0 },
///     ));
/// }
/// ```
pub struct NaturaAnimationPlugin;

/// Component that stores the spring animation state for an entity.
/// Each entity with this component will have independent spring physics.
#[derive(Component, Default)]
pub struct NaturaSprite {
    pub x: f64,
    pub x_velocity: f64,
    pub y: f64,
    pub y_velocity: f64,
}

impl From<NaturaSpriteCore> for NaturaSprite {
    fn from(sprite: NaturaSpriteCore) -> Self {
        NaturaSprite {
            x: sprite.x,
            x_velocity: sprite.x_velocity,
            y: sprite.y,
            y_velocity: sprite.y_velocity,
        }
    }
}

/// Component that stores the spring configuration for an entity.
/// Each entity can have its own spring parameters.
#[derive(Component, Default)]
pub struct NaturaSpring {
    spring: Spring,
}

impl NaturaSpring {
    /// Creates a new NaturaSpring with the specified parameters.
    /// 
    /// # Arguments
    /// * `delta_time` - The time step for the animation (use `natura::fps(60)` for 60 FPS)
    /// * `angular_frequency` - Controls the speed of the animation
    /// * `damping_ratio` - Controls the springiness (< 1 bouncy, = 1 smooth, > 1 sluggish)
    #[must_use]
    pub fn new(
        delta_time: DeltaTime,
        angular_frequency: AngularFrequency,
        damping_ratio: DampingRatio,
    ) -> Self {
        NaturaSpring {
            spring: Spring::new(
                DeltaTime(natura::fps(delta_time.0 as u64)),
                angular_frequency,
                damping_ratio,
            ),
        }
    }

    /// Updates the position and velocity based on the spring physics.
    /// Returns the new (position, velocity) tuple.
    pub fn update(&mut self, pos: f64, vel: f64, equilibrium_pos: f64) -> (f64, f64) {
        self.spring.update(pos, vel, equilibrium_pos)
    }
}

/// Component that specifies the target position for spring animation.
/// The entity will animate towards this position.
#[derive(Component, Default)]
pub struct NaturaTarget {
    pub x: f64,
    pub y: f64,
}

/// Bundle containing all components needed for Natura spring animation.
/// Add this bundle to any entity that should have spring-based movement.
/// 
/// # Example
/// 
/// ```rust,ignore
/// commands.spawn((
///     SpriteBundle { /* ... */ },
///     NaturaSpringBundle::new(
///         DeltaTime(60.0),
///         AngularFrequency(6.0),
///         DampingRatio(0.7),
///     ),
///     NaturaTarget { x: 100.0, y: 200.0 },
/// ));
/// ```
#[derive(Bundle, Default)]
pub struct NaturaSpringBundle {
    pub sprite: NaturaSprite,
    pub spring: NaturaSpring,
}

impl NaturaSpringBundle {
    /// Creates a new bundle with the specified spring parameters.
    #[must_use]
    pub fn new(
        delta_time: DeltaTime,
        angular_frequency: AngularFrequency,
        damping_ratio: DampingRatio,
    ) -> Self {
        NaturaSpringBundle {
            sprite: NaturaSprite::default(),
            spring: NaturaSpring::new(delta_time, angular_frequency, damping_ratio),
        }
    }
}

/// System that updates all entities with Natura spring animations.
/// This system queries all entities that have NaturaSprite, NaturaSpring,
/// NaturaTarget, and Transform components, and applies spring physics
/// to animate them towards their targets.
fn natura_animation_system(
    mut query: Query<(
        &mut NaturaSprite,
        &mut NaturaSpring,
        &NaturaTarget,
        &mut Transform,
    )>,
) {
    for (mut sprite, mut spring, target, mut transform) in query.iter_mut() {
        // Update X position with spring physics
        let (new_x, new_x_vel) = spring.update(sprite.x, sprite.x_velocity, target.x);
        sprite.x = new_x;
        sprite.x_velocity = new_x_vel;

        // Update Y position with spring physics
        let (new_y, new_y_vel) = spring.update(sprite.y, sprite.y_velocity, target.y);
        sprite.y = new_y;
        sprite.y_velocity = new_y_vel;

        // Apply the animated position to the transform
        transform.translation.x = sprite.x as f32;
        transform.translation.y = sprite.y as f32;
    }
}

impl Plugin for NaturaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, natura_animation_system);
    }
}

// Re-export natura types for convenience
pub use natura::{AngularFrequency, DampingRatio, DeltaTime};
