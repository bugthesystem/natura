use bevy::prelude::*;
use natura::{Spring, Sprite as NaturaSpriteCore};

// ==================== Animation Events ====================

/// Event emitted when an animation starts moving towards its target.
/// This is sent when an entity begins animating from rest or when the target changes.
#[derive(Event, Debug, Clone)]
pub struct AnimationStarted {
    /// The entity that started animating
    pub entity: Entity,
    /// The target position the entity is moving towards
    pub target: Vec3,
}

/// Event emitted when an animation completes (reaches its target and comes to rest).
#[derive(Event, Debug, Clone)]
pub struct AnimationCompleted {
    /// The entity that completed its animation
    pub entity: Entity,
    /// The final position of the entity
    pub final_position: Vec3,
}

// ==================== Animation State ====================

/// Tracks the animation state for event emission
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum AnimationState {
    /// Animation is not moving (at rest)
    #[default]
    Idle,
    /// Animation is actively moving towards target
    Animating,
    /// Animation just completed this frame
    JustCompleted,
}

// ==================== Pause/Resume ====================

/// Component to pause an individual entity's animation.
/// Add this component to pause, remove it to resume.
#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct AnimationPaused;

/// Resource to globally pause all Natura animations.
/// Insert this resource to pause all animations, remove to resume.
#[derive(Resource, Default, Debug, Clone)]
pub struct GlobalAnimationPaused;

// ==================== Animation Groups ====================

/// Component to group animations together.
/// Entities with the same group ID can be controlled together.
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct AnimationGroup(pub u32);

impl AnimationGroup {
    /// Creates a new animation group with the specified ID.
    #[must_use]
    pub fn new(id: u32) -> Self {
        AnimationGroup(id)
    }
}

/// Resource to pause specific animation groups.
/// Groups listed here will not animate.
#[derive(Resource, Default, Debug, Clone)]
pub struct PausedGroups {
    /// Set of paused group IDs
    pub groups: std::collections::HashSet<u32>,
}

impl PausedGroups {
    /// Pauses the specified group.
    pub fn pause(&mut self, group_id: u32) {
        self.groups.insert(group_id);
    }

    /// Resumes the specified group.
    pub fn resume(&mut self, group_id: u32) {
        self.groups.remove(&group_id);
    }

    /// Returns true if the group is paused.
    #[must_use]
    pub fn is_paused(&self, group_id: u32) -> bool {
        self.groups.contains(&group_id)
    }
}

// ==================== Easing Curves ====================

/// Easing curve types for animation modification.
/// These curves modify how the spring animation progresses over time.
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub enum EasingCurve {
    /// No easing modification - pure spring physics
    #[default]
    None,
    /// Ease in - starts slow, speeds up
    EaseIn,
    /// Ease out - starts fast, slows down
    EaseOut,
    /// Ease in and out - slow start and end
    EaseInOut,
    /// Quadratic ease in
    QuadraticIn,
    /// Quadratic ease out
    QuadraticOut,
    /// Cubic ease in
    CubicIn,
    /// Cubic ease out
    CubicOut,
    /// Elastic bounce effect
    Elastic,
    /// Bounce effect at the end
    Bounce,
}

impl EasingCurve {
    /// Applies the easing curve to a progress value (0.0 to 1.0).
    /// Returns the eased progress value.
    #[must_use]
    pub fn apply(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingCurve::None => t,
            EasingCurve::EaseIn => t * t,
            EasingCurve::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingCurve::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            EasingCurve::QuadraticIn => t * t,
            EasingCurve::QuadraticOut => t * (2.0 - t),
            EasingCurve::CubicIn => t * t * t,
            EasingCurve::CubicOut => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            EasingCurve::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    (2.0_f64).powf(-10.0 * t) * ((t - s) * (2.0 * std::f64::consts::PI / p)).sin() + 1.0
                }
            }
            EasingCurve::Bounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
        }
    }
}

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
    pub state: AnimationState,
    pub easing: EasingCurve,
}

impl NaturaSpringBundle {
    /// Creates a new bundle with the specified spring parameters.
    #[must_use]
    pub fn new(angular_frequency: AngularFrequency, damping_ratio: DampingRatio) -> Self {
        NaturaSpringBundle {
            sprite: NaturaSprite::default(),
            spring: NaturaSpring::new(angular_frequency, damping_ratio),
            state: AnimationState::default(),
            easing: EasingCurve::default(),
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
            state: AnimationState::default(),
            easing: EasingCurve::default(),
        }
    }

    /// Creates a new bundle with an easing curve.
    #[must_use]
    pub fn with_easing(
        angular_frequency: AngularFrequency,
        damping_ratio: DampingRatio,
        easing: EasingCurve,
    ) -> Self {
        NaturaSpringBundle {
            sprite: NaturaSprite::default(),
            spring: NaturaSpring::new(angular_frequency, damping_ratio),
            state: AnimationState::default(),
            easing,
        }
    }

    /// Creates a new bundle with a group assignment.
    #[must_use]
    pub fn with_group(
        angular_frequency: AngularFrequency,
        damping_ratio: DampingRatio,
        _group_id: u32,
    ) -> Self {
        // Note: AnimationGroup component should be added separately
        NaturaSpringBundle {
            sprite: NaturaSprite::default(),
            spring: NaturaSpring::new(angular_frequency, damping_ratio),
            state: AnimationState::default(),
            easing: EasingCurve::default(),
        }
    }
}

/// Velocity threshold for determining if an animation is at rest
const REST_VELOCITY_THRESHOLD: f64 = 0.01;
/// Position threshold for determining if an animation has reached its target
const TARGET_POSITION_THRESHOLD: f64 = 0.1;

/// System that updates all entities with Natura spring animations.
/// This system queries all entities that have NaturaSprite, NaturaSpring,
/// NaturaTarget, and Transform components, and applies spring physics
/// to animate them towards their targets.
/// 
/// Supports:
/// - Individual entity pausing via `AnimationPaused` component
/// - Global pausing via `GlobalAnimationPaused` resource
/// - Group pausing via `PausedGroups` resource
/// - Animation events (`AnimationStarted`, `AnimationCompleted`)
/// - Easing curves via `EasingCurve` component
/// 
/// Uses Bevy's Time resource for frame-rate independent animation.
fn natura_animation_system(
    time: Res<Time>,
    global_pause: Option<Res<GlobalAnimationPaused>>,
    paused_groups: Option<Res<PausedGroups>>,
    mut ev_started: EventWriter<AnimationStarted>,
    mut ev_completed: EventWriter<AnimationCompleted>,
    mut query: Query<(
        Entity,
        &mut NaturaSprite,
        &mut NaturaSpring,
        &NaturaTarget,
        &mut Transform,
        &mut AnimationState,
        Option<&EasingCurve>,
        Option<&AnimationGroup>,
        Option<&AnimationPaused>,
    )>,
) {
    // Check for global pause
    if global_pause.is_some() {
        return;
    }

    let delta_seconds = time.delta_secs_f64();
    
    // Skip if delta is too small or too large (e.g., during pause or lag spikes)
    if delta_seconds < 0.0001 || delta_seconds > 0.1 {
        return;
    }

    for (entity, mut sprite, mut spring, target, mut transform, mut state, easing, group, paused) in query.iter_mut() {
        // Skip if individually paused
        if paused.is_some() {
            continue;
        }

        // Skip if group is paused
        if let (Some(group), Some(paused_groups)) = (group, &paused_groups) {
            if paused_groups.is_paused(group.0) {
                continue;
            }
        }

        // Calculate distance to target before update
        let prev_at_rest = sprite.is_at_rest(REST_VELOCITY_THRESHOLD);
        let prev_distance = ((sprite.x - target.x).powi(2) 
            + (sprite.y - target.y).powi(2) 
            + (sprite.z - target.z).powi(2)).sqrt();

        // Get easing curve (default to None if not present)
        let easing_curve = easing.copied().unwrap_or(EasingCurve::None);

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

        // Apply easing curve if present (modifies the interpolation towards target)
        if easing_curve != EasingCurve::None {
            // Calculate progress based on distance to target
            let current_distance = ((sprite.x - target.x).powi(2) 
                + (sprite.y - target.y).powi(2) 
                + (sprite.z - target.z).powi(2)).sqrt();
            
            if prev_distance > TARGET_POSITION_THRESHOLD {
                let raw_progress = 1.0 - (current_distance / prev_distance).min(1.0);
                let eased_progress = easing_curve.apply(raw_progress);
                
                // Blend the spring result with eased interpolation
                let blend_factor = 0.3; // How much easing affects the spring
                let eased_x = sprite.x + (target.x - sprite.x) * eased_progress * blend_factor;
                let eased_y = sprite.y + (target.y - sprite.y) * eased_progress * blend_factor;
                let eased_z = sprite.z + (target.z - sprite.z) * eased_progress * blend_factor;
                
                sprite.x = sprite.x * (1.0 - blend_factor) + eased_x * blend_factor;
                sprite.y = sprite.y * (1.0 - blend_factor) + eased_y * blend_factor;
                sprite.z = sprite.z * (1.0 - blend_factor) + eased_z * blend_factor;
            }
        }

        // Apply the animated position to the transform
        transform.translation.x = sprite.x as f32;
        transform.translation.y = sprite.y as f32;
        transform.translation.z = sprite.z as f32;

        // Check if animation just started
        let now_at_rest = sprite.is_at_rest(REST_VELOCITY_THRESHOLD);
        let at_target = ((sprite.x - target.x).abs() < TARGET_POSITION_THRESHOLD)
            && ((sprite.y - target.y).abs() < TARGET_POSITION_THRESHOLD)
            && ((sprite.z - target.z).abs() < TARGET_POSITION_THRESHOLD);

        // State machine for animation events
        match *state {
            AnimationState::Idle => {
                if !now_at_rest && !at_target {
                    *state = AnimationState::Animating;
                    ev_started.send(AnimationStarted {
                        entity,
                        target: Vec3::new(target.x as f32, target.y as f32, target.z as f32),
                    });
                }
            }
            AnimationState::Animating => {
                if now_at_rest && at_target {
                    *state = AnimationState::JustCompleted;
                    ev_completed.send(AnimationCompleted {
                        entity,
                        final_position: Vec3::new(sprite.x as f32, sprite.y as f32, sprite.z as f32),
                    });
                }
            }
            AnimationState::JustCompleted => {
                // Transition back to Idle after one frame
                *state = AnimationState::Idle;
            }
        }

        // If was at rest and now moving towards a different target, send start event
        if prev_at_rest && !now_at_rest && *state == AnimationState::Idle {
            *state = AnimationState::Animating;
            ev_started.send(AnimationStarted {
                entity,
                target: Vec3::new(target.x as f32, target.y as f32, target.z as f32),
            });
        }
    }
}

impl Plugin for NaturaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NaturaSprite>()
            .register_type::<NaturaSpring>()
            .register_type::<NaturaTarget>()
            .register_type::<AnimationState>()
            .register_type::<EasingCurve>()
            .register_type::<AnimationGroup>()
            .register_type::<AnimationPaused>()
            .add_event::<AnimationStarted>()
            .add_event::<AnimationCompleted>()
            .init_resource::<PausedGroups>()
            .add_systems(Update, natura_animation_system);
    }
}

// Re-export natura types for convenience
pub use natura::{AngularFrequency, DampingRatio, DeltaTime};

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== NaturaSprite Tests ====================

    #[test]
    fn test_natura_sprite_new() {
        let sprite = NaturaSprite::new(10.0, 20.0, 30.0);
        assert_eq!(sprite.x, 10.0);
        assert_eq!(sprite.y, 20.0);
        assert_eq!(sprite.z, 30.0);
        assert_eq!(sprite.x_velocity, 0.0);
        assert_eq!(sprite.y_velocity, 0.0);
        assert_eq!(sprite.z_velocity, 0.0);
    }

    #[test]
    fn test_natura_sprite_new_2d() {
        let sprite = NaturaSprite::new_2d(10.0, 20.0);
        assert_eq!(sprite.x, 10.0);
        assert_eq!(sprite.y, 20.0);
        assert_eq!(sprite.z, 0.0);
        assert_eq!(sprite.x_velocity, 0.0);
        assert_eq!(sprite.y_velocity, 0.0);
        assert_eq!(sprite.z_velocity, 0.0);
    }

    #[test]
    fn test_natura_sprite_default() {
        let sprite = NaturaSprite::default();
        assert_eq!(sprite.x, 0.0);
        assert_eq!(sprite.y, 0.0);
        assert_eq!(sprite.z, 0.0);
        assert_eq!(sprite.x_velocity, 0.0);
        assert_eq!(sprite.y_velocity, 0.0);
        assert_eq!(sprite.z_velocity, 0.0);
    }

    #[test]
    fn test_natura_sprite_is_at_rest_true() {
        let sprite = NaturaSprite {
            x: 100.0,
            y: 100.0,
            z: 0.0,
            x_velocity: 0.001,
            y_velocity: 0.001,
            z_velocity: 0.001,
        };
        assert!(sprite.is_at_rest(0.01));
    }

    #[test]
    fn test_natura_sprite_is_at_rest_false() {
        let sprite = NaturaSprite {
            x: 100.0,
            y: 100.0,
            z: 0.0,
            x_velocity: 1.0,
            y_velocity: 0.0,
            z_velocity: 0.0,
        };
        assert!(!sprite.is_at_rest(0.01));
    }

    #[test]
    fn test_natura_sprite_is_at_rest_all_axes() {
        // X velocity too high
        let sprite_x = NaturaSprite {
            x: 0.0, y: 0.0, z: 0.0,
            x_velocity: 1.0, y_velocity: 0.0, z_velocity: 0.0,
        };
        assert!(!sprite_x.is_at_rest(0.5));

        // Y velocity too high
        let sprite_y = NaturaSprite {
            x: 0.0, y: 0.0, z: 0.0,
            x_velocity: 0.0, y_velocity: 1.0, z_velocity: 0.0,
        };
        assert!(!sprite_y.is_at_rest(0.5));

        // Z velocity too high
        let sprite_z = NaturaSprite {
            x: 0.0, y: 0.0, z: 0.0,
            x_velocity: 0.0, y_velocity: 0.0, z_velocity: 1.0,
        };
        assert!(!sprite_z.is_at_rest(0.5));
    }

    #[test]
    fn test_natura_sprite_clone() {
        let sprite = NaturaSprite::new(1.0, 2.0, 3.0);
        let cloned = sprite.clone();
        assert_eq!(sprite.x, cloned.x);
        assert_eq!(sprite.y, cloned.y);
        assert_eq!(sprite.z, cloned.z);
    }

    // ==================== NaturaTarget Tests ====================

    #[test]
    fn test_natura_target_new() {
        let target = NaturaTarget::new(10.0, 20.0, 30.0);
        assert_eq!(target.x, 10.0);
        assert_eq!(target.y, 20.0);
        assert_eq!(target.z, 30.0);
    }

    #[test]
    fn test_natura_target_new_2d() {
        let target = NaturaTarget::new_2d(10.0, 20.0);
        assert_eq!(target.x, 10.0);
        assert_eq!(target.y, 20.0);
        assert_eq!(target.z, 0.0);
    }

    #[test]
    fn test_natura_target_default() {
        let target = NaturaTarget::default();
        assert_eq!(target.x, 0.0);
        assert_eq!(target.y, 0.0);
        assert_eq!(target.z, 0.0);
    }

    #[test]
    fn test_natura_target_clone() {
        let target = NaturaTarget::new(1.0, 2.0, 3.0);
        let cloned = target.clone();
        assert_eq!(target.x, cloned.x);
        assert_eq!(target.y, cloned.y);
        assert_eq!(target.z, cloned.z);
    }

    // ==================== NaturaSpring Tests ====================

    #[test]
    fn test_natura_spring_new() {
        let spring = NaturaSpring::new(AngularFrequency(8.0), DampingRatio(0.5));
        assert_eq!(spring.angular_frequency, 8.0);
        assert_eq!(spring.damping_ratio, 0.5);
    }

    #[test]
    fn test_natura_spring_default() {
        let spring = NaturaSpring::default();
        assert_eq!(spring.angular_frequency, 6.0);
        assert_eq!(spring.damping_ratio, 0.7);
    }

    #[test]
    fn test_natura_spring_clone() {
        let spring = NaturaSpring::new(AngularFrequency(10.0), DampingRatio(0.3));
        let cloned = spring.clone();
        assert_eq!(spring.angular_frequency, cloned.angular_frequency);
        assert_eq!(spring.damping_ratio, cloned.damping_ratio);
    }

    #[test]
    fn test_natura_spring_update() {
        let mut spring = NaturaSpring::new(AngularFrequency(6.0), DampingRatio(0.7));
        let delta_seconds = 1.0 / 60.0; // 60 FPS
        
        // Starting at position 0 with no velocity, target at 100
        let (new_pos, new_vel) = spring.update(0.0, 0.0, 100.0, delta_seconds);
        
        // Position should move towards target
        assert!(new_pos > 0.0);
        // Velocity should be positive (moving towards target)
        assert!(new_vel > 0.0);
    }

    #[test]
    fn test_natura_spring_update_at_target() {
        let mut spring = NaturaSpring::new(AngularFrequency(6.0), DampingRatio(1.0));
        let delta_seconds = 1.0 / 60.0;
        
        // Already at target with no velocity
        let (new_pos, new_vel) = spring.update(100.0, 0.0, 100.0, delta_seconds);
        
        // Should stay at target
        assert!((new_pos - 100.0).abs() < 0.001);
        assert!(new_vel.abs() < 0.001);
    }

    // ==================== NaturaSpringBundle Tests ====================

    #[test]
    fn test_natura_spring_bundle_new() {
        let bundle = NaturaSpringBundle::new(AngularFrequency(8.0), DampingRatio(0.5));
        
        // Check spring parameters
        assert_eq!(bundle.spring.angular_frequency, 8.0);
        assert_eq!(bundle.spring.damping_ratio, 0.5);
        
        // Check sprite is at default position
        assert_eq!(bundle.sprite.x, 0.0);
        assert_eq!(bundle.sprite.y, 0.0);
        assert_eq!(bundle.sprite.z, 0.0);
    }

    #[test]
    fn test_natura_spring_bundle_with_position() {
        let bundle = NaturaSpringBundle::with_position(
            AngularFrequency(8.0),
            DampingRatio(0.5),
            10.0,
            20.0,
            30.0,
        );
        
        // Check spring parameters
        assert_eq!(bundle.spring.angular_frequency, 8.0);
        assert_eq!(bundle.spring.damping_ratio, 0.5);
        
        // Check sprite position
        assert_eq!(bundle.sprite.x, 10.0);
        assert_eq!(bundle.sprite.y, 20.0);
        assert_eq!(bundle.sprite.z, 30.0);
    }

    #[test]
    fn test_natura_spring_bundle_default() {
        let bundle = NaturaSpringBundle::default();
        
        // Check default spring parameters
        assert_eq!(bundle.spring.angular_frequency, 6.0);
        assert_eq!(bundle.spring.damping_ratio, 0.7);
        
        // Check sprite is at origin
        assert_eq!(bundle.sprite.x, 0.0);
        assert_eq!(bundle.sprite.y, 0.0);
        assert_eq!(bundle.sprite.z, 0.0);
    }

    // ==================== Spring Animation Behavior Tests ====================

    #[test]
    fn test_spring_converges_to_target() {
        let mut spring = NaturaSpring::new(AngularFrequency(6.0), DampingRatio(0.7));
        let delta_seconds = 1.0 / 60.0;
        let target = 100.0;
        
        let mut pos = 0.0;
        let mut vel = 0.0;
        
        // Simulate 5 seconds (300 frames at 60 FPS)
        for _ in 0..300 {
            let (new_pos, new_vel) = spring.update(pos, vel, target, delta_seconds);
            pos = new_pos;
            vel = new_vel;
        }
        
        // Should be very close to target after 5 seconds
        assert!((pos - target).abs() < 1.0);
    }

    #[test]
    fn test_under_damped_spring_oscillates() {
        let mut spring = NaturaSpring::new(AngularFrequency(6.0), DampingRatio(0.3));
        let delta_seconds = 1.0 / 60.0;
        let target = 100.0;
        
        let mut pos = 0.0;
        let mut vel = 0.0;
        let mut max_pos = 0.0;
        
        // Simulate for a while and track max position
        for _ in 0..120 {
            let (new_pos, new_vel) = spring.update(pos, vel, target, delta_seconds);
            pos = new_pos;
            vel = new_vel;
            if pos > max_pos {
                max_pos = pos;
            }
        }
        
        // Under-damped spring should overshoot the target
        assert!(max_pos > target);
    }

    #[test]
    fn test_critically_damped_no_overshoot() {
        let mut spring = NaturaSpring::new(AngularFrequency(6.0), DampingRatio(1.0));
        let delta_seconds = 1.0 / 60.0;
        let target = 100.0;
        
        let mut pos = 0.0;
        let mut vel = 0.0;
        let mut max_pos = 0.0;
        
        // Simulate for a while
        for _ in 0..300 {
            let (new_pos, new_vel) = spring.update(pos, vel, target, delta_seconds);
            pos = new_pos;
            vel = new_vel;
            if pos > max_pos {
                max_pos = pos;
            }
        }
        
        // Critically damped spring should not significantly overshoot
        assert!(max_pos <= target + 0.5);
    }

    // ==================== From Trait Tests ====================

    #[test]
    fn test_natura_sprite_from_natura_sprite_core() {
        let core_sprite = natura::Sprite {
            x: 10.0,
            x_velocity: 1.0,
            y: 20.0,
            y_velocity: 2.0,
        };
        
        let sprite: NaturaSprite = core_sprite.into();
        
        assert_eq!(sprite.x, 10.0);
        assert_eq!(sprite.x_velocity, 1.0);
        assert_eq!(sprite.y, 20.0);
        assert_eq!(sprite.y_velocity, 2.0);
        assert_eq!(sprite.z, 0.0);
        assert_eq!(sprite.z_velocity, 0.0);
    }

    // ==================== Debug Trait Tests ====================

    #[test]
    fn test_natura_sprite_debug() {
        let sprite = NaturaSprite::new(1.0, 2.0, 3.0);
        let debug_str = format!("{:?}", sprite);
        assert!(debug_str.contains("NaturaSprite"));
    }

    #[test]
    fn test_natura_spring_debug() {
        let spring = NaturaSpring::new(AngularFrequency(6.0), DampingRatio(0.7));
        let debug_str = format!("{:?}", spring);
        assert!(debug_str.contains("NaturaSpring"));
        assert!(debug_str.contains("angular_frequency"));
        assert!(debug_str.contains("damping_ratio"));
    }

    #[test]
    fn test_natura_target_debug() {
        let target = NaturaTarget::new(1.0, 2.0, 3.0);
        let debug_str = format!("{:?}", target);
        assert!(debug_str.contains("NaturaTarget"));
    }

    // ==================== Easing Curve Tests ====================

    #[test]
    fn test_easing_curve_none() {
        let easing = EasingCurve::None;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.5);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_curve_ease_in() {
        let easing = EasingCurve::EaseIn;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!(easing.apply(0.5) < 0.5); // Should be slower at start
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_curve_ease_out() {
        let easing = EasingCurve::EaseOut;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!(easing.apply(0.5) > 0.5); // Should be faster at start
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_curve_ease_in_out() {
        let easing = EasingCurve::EaseInOut;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!((easing.apply(0.5) - 0.5).abs() < 0.01); // Middle should be close to 0.5
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_curve_quadratic_in() {
        let easing = EasingCurve::QuadraticIn;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.25); // 0.5^2 = 0.25
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_curve_quadratic_out() {
        let easing = EasingCurve::QuadraticOut;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.75); // 0.5 * (2 - 0.5) = 0.75
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_curve_cubic_in() {
        let easing = EasingCurve::CubicIn;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.125); // 0.5^3 = 0.125
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_curve_cubic_out() {
        let easing = EasingCurve::CubicOut;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!((easing.apply(0.5) - 0.875).abs() < 0.001);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_curve_elastic() {
        let easing = EasingCurve::Elastic;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(1.0), 1.0);
        // Elastic should overshoot
        assert!(easing.apply(0.9) > 1.0 || easing.apply(0.8) > 1.0);
    }

    #[test]
    fn test_easing_curve_bounce() {
        let easing = EasingCurve::Bounce;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!((easing.apply(1.0) - 1.0).abs() < 0.001);
        // Bounce should have specific values at certain points
        assert!(easing.apply(0.5) > 0.0);
    }

    #[test]
    fn test_easing_curve_clamps_input() {
        let easing = EasingCurve::None;
        // Values outside 0-1 should be clamped
        assert_eq!(easing.apply(-0.5), 0.0);
        assert_eq!(easing.apply(1.5), 1.0);
    }

    // ==================== Animation Group Tests ====================

    #[test]
    fn test_animation_group_new() {
        let group = AnimationGroup::new(42);
        assert_eq!(group.0, 42);
    }

    #[test]
    fn test_animation_group_equality() {
        let group1 = AnimationGroup::new(1);
        let group2 = AnimationGroup::new(1);
        let group3 = AnimationGroup::new(2);
        assert_eq!(group1, group2);
        assert_ne!(group1, group3);
    }

    #[test]
    fn test_paused_groups_pause_resume() {
        let mut paused = PausedGroups::default();
        assert!(!paused.is_paused(1));
        
        paused.pause(1);
        assert!(paused.is_paused(1));
        assert!(!paused.is_paused(2));
        
        paused.resume(1);
        assert!(!paused.is_paused(1));
    }

    #[test]
    fn test_paused_groups_multiple() {
        let mut paused = PausedGroups::default();
        paused.pause(1);
        paused.pause(2);
        paused.pause(3);
        
        assert!(paused.is_paused(1));
        assert!(paused.is_paused(2));
        assert!(paused.is_paused(3));
        assert!(!paused.is_paused(4));
        
        paused.resume(2);
        assert!(paused.is_paused(1));
        assert!(!paused.is_paused(2));
        assert!(paused.is_paused(3));
    }

    // ==================== Animation State Tests ====================

    #[test]
    fn test_animation_state_default() {
        let state = AnimationState::default();
        assert_eq!(state, AnimationState::Idle);
    }

    #[test]
    fn test_animation_state_equality() {
        assert_eq!(AnimationState::Idle, AnimationState::Idle);
        assert_eq!(AnimationState::Animating, AnimationState::Animating);
        assert_eq!(AnimationState::JustCompleted, AnimationState::JustCompleted);
        assert_ne!(AnimationState::Idle, AnimationState::Animating);
    }

    // ==================== Event Tests ====================

    #[test]
    fn test_animation_started_event() {
        let event = AnimationStarted {
            entity: Entity::from_raw(42),
            target: Vec3::new(100.0, 200.0, 0.0),
        };
        assert_eq!(event.target, Vec3::new(100.0, 200.0, 0.0));
    }

    #[test]
    fn test_animation_completed_event() {
        let event = AnimationCompleted {
            entity: Entity::from_raw(42),
            final_position: Vec3::new(100.0, 200.0, 0.0),
        };
        assert_eq!(event.final_position, Vec3::new(100.0, 200.0, 0.0));
    }

    // ==================== Bundle with Easing Tests ====================

    #[test]
    fn test_natura_spring_bundle_with_easing() {
        let bundle = NaturaSpringBundle::with_easing(
            AngularFrequency(8.0),
            DampingRatio(0.5),
            EasingCurve::EaseOut,
        );
        
        assert_eq!(bundle.spring.angular_frequency, 8.0);
        assert_eq!(bundle.spring.damping_ratio, 0.5);
        assert_eq!(bundle.easing, EasingCurve::EaseOut);
        assert_eq!(bundle.state, AnimationState::Idle);
    }

    #[test]
    fn test_natura_spring_bundle_includes_state_and_easing() {
        let bundle = NaturaSpringBundle::new(AngularFrequency(6.0), DampingRatio(0.7));
        
        // Bundle should include default state and easing
        assert_eq!(bundle.state, AnimationState::Idle);
        assert_eq!(bundle.easing, EasingCurve::None);
    }

    // ==================== Global Pause Resource Tests ====================

    #[test]
    fn test_global_animation_paused_default() {
        let _paused = GlobalAnimationPaused::default();
        // Just ensure it can be created
    }

    // ==================== Animation Paused Component Tests ====================

    #[test]
    fn test_animation_paused_default() {
        let _paused = AnimationPaused::default();
        // Just ensure it can be created
    }
}
