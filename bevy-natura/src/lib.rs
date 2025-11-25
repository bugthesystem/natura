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
///             AngularFrequency(6.0),
///             DampingRatio(0.7),
///         ),
///         NaturaTarget { x: 100.0, y: 200.0, z: 0.0 },
///     ));
/// }
/// ```
pub struct NaturaAnimationPlugin;

/// Component that stores the spring animation state for an entity.
/// Each entity with this component will have independent spring physics.
/// 
/// Supports 3D positions (x, y, z) for both 2D and 3D games.
#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct NaturaSprite {
    pub x: f64,
    pub x_velocity: f64,
    pub y: f64,
    pub y_velocity: f64,
    pub z: f64,
    pub z_velocity: f64,
}

impl NaturaSprite {
    /// Creates a new NaturaSprite with the specified initial position.
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        NaturaSprite {
            x,
            y,
            z,
            x_velocity: 0.0,
            y_velocity: 0.0,
            z_velocity: 0.0,
        }
    }

    /// Creates a new 2D NaturaSprite (z = 0).
    #[must_use]
    pub fn new_2d(x: f64, y: f64) -> Self {
        Self::new(x, y, 0.0)
    }

    /// Returns true if the sprite has effectively stopped moving.
    /// Uses a threshold to determine if velocities are negligible.
    #[must_use]
    pub fn is_at_rest(&self, velocity_threshold: f64) -> bool {
        self.x_velocity.abs() < velocity_threshold
            && self.y_velocity.abs() < velocity_threshold
            && self.z_velocity.abs() < velocity_threshold
    }
}

impl From<NaturaSpriteCore> for NaturaSprite {
    fn from(sprite: NaturaSpriteCore) -> Self {
        NaturaSprite {
            x: sprite.x,
            x_velocity: sprite.x_velocity,
            y: sprite.y,
            y_velocity: sprite.y_velocity,
            z: 0.0,
            z_velocity: 0.0,
        }
    }
}

/// Component that stores the spring configuration for an entity.
/// Each entity can have its own spring parameters.
/// 
/// The spring uses Bevy's Time resource for frame-rate independent animation.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct NaturaSpring {
    /// Angular frequency - controls animation speed (higher = faster)
    pub angular_frequency: f64,
    /// Damping ratio - controls springiness (< 1 bouncy, = 1 smooth, > 1 sluggish)
    pub damping_ratio: f64,
    /// Cached spring for the current frame's delta time
    #[reflect(ignore)]
    cached_spring: Option<(f64, Spring)>,
}

impl Clone for NaturaSpring {
    fn clone(&self) -> Self {
        // Don't clone the cached spring - it will be recreated on first use
        NaturaSpring {
            angular_frequency: self.angular_frequency,
            damping_ratio: self.damping_ratio,
            cached_spring: None,
        }
    }
}

impl std::fmt::Debug for NaturaSpring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NaturaSpring")
            .field("angular_frequency", &self.angular_frequency)
            .field("damping_ratio", &self.damping_ratio)
            .finish()
    }
}

impl Default for NaturaSpring {
    fn default() -> Self {
        NaturaSpring {
            angular_frequency: 6.0,
            damping_ratio: 0.7,
            cached_spring: None,
        }
    }
}

impl NaturaSpring {
    /// Creates a new NaturaSpring with the specified parameters.
    /// 
    /// # Arguments
    /// * `angular_frequency` - Controls the speed of the animation (higher = faster)
    /// * `damping_ratio` - Controls the springiness (< 1 bouncy, = 1 smooth, > 1 sluggish)
    #[must_use]
    pub fn new(angular_frequency: AngularFrequency, damping_ratio: DampingRatio) -> Self {
        NaturaSpring {
            angular_frequency: angular_frequency.0,
            damping_ratio: damping_ratio.0,
            cached_spring: None,
        }
    }

    /// Gets or creates a spring for the given delta time.
    fn get_spring(&mut self, delta_seconds: f64) -> &mut Spring {
        // Check if we need to recreate the spring (delta time changed significantly)
        let needs_update = match &self.cached_spring {
            Some((cached_dt, _)) => (cached_dt - delta_seconds).abs() > 0.001,
            None => true,
        };

        if needs_update {
            let spring = Spring::new(
                DeltaTime(delta_seconds),
                AngularFrequency(self.angular_frequency),
                DampingRatio(self.damping_ratio),
            );
            self.cached_spring = Some((delta_seconds, spring));
        }

        &mut self.cached_spring.as_mut().unwrap().1
    }

    /// Updates the position and velocity based on the spring physics.
    /// Returns the new (position, velocity) tuple.
    pub fn update(&mut self, pos: f64, vel: f64, equilibrium_pos: f64, delta_seconds: f64) -> (f64, f64) {
        let spring = self.get_spring(delta_seconds);
        spring.update(pos, vel, equilibrium_pos)
    }
}

/// Component that specifies the target position for spring animation.
/// The entity will animate towards this position.
/// 
/// Supports 3D targets (x, y, z) for both 2D and 3D games.
#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct NaturaTarget {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl NaturaTarget {
    /// Creates a new 2D target (z = 0).
    #[must_use]
    pub fn new_2d(x: f64, y: f64) -> Self {
        NaturaTarget { x, y, z: 0.0 }
    }

    /// Creates a new 3D target.
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        NaturaTarget { x, y, z }
    }
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
///         AngularFrequency(6.0),
///         DampingRatio(0.7),
///     ),
///     NaturaTarget::new_2d(100.0, 200.0),
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
    pub fn new(angular_frequency: AngularFrequency, damping_ratio: DampingRatio) -> Self {
        NaturaSpringBundle {
            sprite: NaturaSprite::default(),
            spring: NaturaSpring::new(angular_frequency, damping_ratio),
        }
    }

    /// Creates a new bundle with a custom initial position.
    #[must_use]
    pub fn with_position(
        angular_frequency: AngularFrequency,
        damping_ratio: DampingRatio,
        initial_x: f64,
        initial_y: f64,
        initial_z: f64,
    ) -> Self {
        NaturaSpringBundle {
            sprite: NaturaSprite::new(initial_x, initial_y, initial_z),
            spring: NaturaSpring::new(angular_frequency, damping_ratio),
        }
    }
}

/// System that updates all entities with Natura spring animations.
/// This system queries all entities that have NaturaSprite, NaturaSpring,
/// NaturaTarget, and Transform components, and applies spring physics
/// to animate them towards their targets.
/// 
/// Uses Bevy's Time resource for frame-rate independent animation.
fn natura_animation_system(
    time: Res<Time>,
    mut query: Query<(
        &mut NaturaSprite,
        &mut NaturaSpring,
        &NaturaTarget,
        &mut Transform,
    )>,
) {
    let delta_seconds = time.delta_secs_f64();
    
    // Skip if delta is too small or too large (e.g., during pause or lag spikes)
    if delta_seconds < 0.0001 || delta_seconds > 0.1 {
        return;
    }

    for (mut sprite, mut spring, target, mut transform) in query.iter_mut() {
        // Update X position with spring physics
        let (new_x, new_x_vel) = spring.update(sprite.x, sprite.x_velocity, target.x, delta_seconds);
        sprite.x = new_x;
        sprite.x_velocity = new_x_vel;

        // Update Y position with spring physics
        let (new_y, new_y_vel) = spring.update(sprite.y, sprite.y_velocity, target.y, delta_seconds);
        sprite.y = new_y;
        sprite.y_velocity = new_y_vel;

        // Update Z position with spring physics
        let (new_z, new_z_vel) = spring.update(sprite.z, sprite.z_velocity, target.z, delta_seconds);
        sprite.z = new_z;
        sprite.z_velocity = new_z_vel;

        // Apply the animated position to the transform
        transform.translation.x = sprite.x as f32;
        transform.translation.y = sprite.y as f32;
        transform.translation.z = sprite.z as f32;
    }
}

impl Plugin for NaturaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NaturaSprite>()
            .register_type::<NaturaSpring>()
            .register_type::<NaturaTarget>()
            .add_systems(Update, natura_animation_system);
    }
}

// Re-export natura types for convenience
pub use natura::{AngularFrequency, DampingRatio, DeltaTime};
