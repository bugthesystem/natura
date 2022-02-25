/// This file defines a simplified damped harmonic oscillator, colloquially
/// known as a spring. This is ported from Ryan Juckett’s simple damped harmonic
/// motion, originally written in C++.
///
/// Example usage:
///
///```
/// use natura::{Spring, Vector, Point};
/// // Run once to initialize.
/// let mut spring = Spring::new(natura::fps(60), 6.0, 0.5);
///
/// // Update on every frame.
/// let mut pos = 0.0;
/// let mut velocity = 0.0;
/// const TARGET_POS:f64 = 100.0;
/// some_update_loop(|| {
///    let (pos_new, velocity_new) = spring.update(pos, velocity, TARGET_POS);
/// });
///```
//
// For background on the algorithm see:
// https://www.ryanjuckett.com/damped-springs/

/******************************************************************************
  Copyright (c) 2008-2012 Ryan Juckett
  http://www.ryanjuckett.com/
  This software is provided 'as-is', without any express or implied
  warranty. In no event will the authors be held liable for any damages
  arising from the use of this software.
  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:
  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source
     distribution.
********************************************************************************/
use std::fmt;
use std::fmt::Formatter;
use std::time::Duration;

/// Spring contains a cached set of motion parameters that can be used to
/// efficiently update multiple springs using the same time step, angular
/// frequency and damping ratio.
///
/// To use a Spring call ::new with the time delta (that's animation frame
/// length), frequency, and damping parameters, cache the result, then call
/// Update to update position and velocity values for each spring that needs
/// updating.
///
/// # Example:
///
/// ```
/// use natura::{Spring, fps};
///
/// // First precompute spring coefficients based on your settings:
/// let x:f64 =0.0;
/// let x_vel:f64 = 0.0;
/// let y:f64 = 0.0;
/// let y_vel:f64 = 0.0;
///
/// delta_time = fps(60);
/// let mut s = Spring::new(delta_time, 5.0, 0.2);
///
/// // Then, in your update loop:
/// let (x_new, x_vel_new) = s.update(x, x_vel, 10.0); // update the X position
/// let (y_new, y_vel_new) = s.update(y, y_vel, 20.0); // update the Y position
///
#[derive(Default)]
pub struct Spring {
    ///
    pos_pos_coef: f64,

    ///
    pos_vel_coef: f64,

    ///
    vel_pos_coef: f64,

    ///
    vel_vel_coef: f64,
}

/// In calculus ε is, in vague terms, an arbitrarily small positive number. In
/// the original C++ source ε is represented as such:
/// `const float epsilon = 0.0001`;
///
const EPSILON: f64 = 0.00000001;

/// fps returns a time delta for a given number of frames per second. This
/// value can be used as the time delta when initializing a Spring. Note that
/// game engines often provide the time delta as well, which you should use
/// instead of this function, if possible.
///
/// Example:
/// ```
/// use natura::{Spring, fps};
///
/// let mut spring = Spring::new(fps(60), 5.0, 0.2);
/// ```
pub fn fps(n: u64) -> f64 {
    let duration = Duration::new(0, n as u32).as_nanos();
    let second = Duration::from_secs(1).as_nanos();

    (((second / duration) as f64 / 1000000.0) / 1000.0) as f64
}

pub struct DeltaTime(pub u64);

pub struct AngularFrequency(pub f64);

pub struct DampingRatio(pub f64);

impl Spring {
    /// new initializes a new Spring, computing the parameters needed to
    /// simulate a damped spring over a given period of time.
    ///
    /// The delta time is the time step to advance; essentially the framerate.
    ///
    /// The angular frequency is the angular frequency of motion, which affects the
    /// speed.
    ///
    /// The damping ratio is the damping ratio of motion, which determines the
    /// oscillation, or lack thereof. There are three categories of damping ratios:
    ///
    /// Damping ratio > 1: over-damped.
    /// Damping ratio = 1: critically-damped.
    /// Damping ratio < 1: under-damped.
    ///
    /// An over-damped spring will never oscillate, but reaches equilibrium at
    /// a slower rate than a critically damped spring.
    ///
    /// A critically damped spring will reach equilibrium as fast as possible
    /// without oscillating.
    ///
    /// An under-damped spring will reach equilibrium the fastest, but also
    /// overshoots it and continues to oscillate as its amplitude decays over time.
    pub fn new(delta_time: f64, mut angular_frequency: f64, mut damping_ratio: f64) -> Self {
        let mut spring = Spring::default();

        // keep values in a legal range.
        angular_frequency = f64::max(0.0, angular_frequency);
        damping_ratio = f64::max(0.0, damping_ratio);

        // if there is no angular frequency, the spring will not move and we can
        // return identity.
        if angular_frequency < EPSILON {
            spring.pos_pos_coef = 1.0;
            spring.pos_vel_coef = 0.0;
            spring.vel_pos_coef = 0.0;
            spring.vel_vel_coef = 1.0;
            return spring;
        }

        if damping_ratio > 1.0 + EPSILON {
            // Over-damped.
            Self::calculate_over_damped(delta_time, angular_frequency, damping_ratio, &mut spring);
        } else if damping_ratio < 1.0 - EPSILON {
            // Under-damped.
            Self::calculate_under_damped(delta_time, angular_frequency, damping_ratio, &mut spring)
        } else {
            // Critically damped.
            Self::calculate_critically_damped(delta_time, angular_frequency, &mut spring)
        }

        spring
    }

    /// update updates position and velocity values against a given target value.
    /// call this after calling [Spring::new] to update values.
    pub fn update(&mut self, pos: f64, vel: f64, equilibrium_pos: f64) -> (f64, f64) {
        let old_pos = pos - equilibrium_pos; // update in equilibrium relative space
        let old_vel = vel;

        let new_pos = old_pos * self.pos_pos_coef + old_vel * self.pos_vel_coef + equilibrium_pos;
        let new_vel = old_pos * self.vel_pos_coef + old_vel * self.vel_vel_coef;

        (new_pos, new_vel)
    }

    #[inline(always)]
    fn calculate_critically_damped(delta_time: f64, angular_frequency: f64, spring: &mut Spring) {
        let exp_term = (-angular_frequency * delta_time).exp();
        let time_exp = delta_time * exp_term;
        let time_exp_freq = time_exp * angular_frequency;

        spring.pos_pos_coef = time_exp_freq + exp_term;
        spring.pos_vel_coef = time_exp;

        spring.vel_pos_coef = -angular_frequency * time_exp_freq;
        spring.vel_vel_coef = -time_exp_freq + exp_term
    }

    #[inline(always)]
    fn calculate_under_damped(
        delta_time: f64,
        angular_frequency: f64,
        damping_ratio: f64,
        spring: &mut Spring,
    ) {
        let omega_zeta = angular_frequency * damping_ratio;
        let alpha = angular_frequency * (1.0 - damping_ratio * damping_ratio).sqrt();

        let exp_term = (-omega_zeta * delta_time).exp();
        let cos_term = (alpha * delta_time).cos();
        let sin_term = (alpha * delta_time).sin();

        let inv_alpha = 1.0 / alpha;

        let exp_sin = exp_term * sin_term;
        let exp_cos = exp_term * cos_term;
        let exp_omega_zeta_sin_over_alpha = exp_term * omega_zeta * sin_term * inv_alpha;

        spring.pos_pos_coef = exp_cos + exp_omega_zeta_sin_over_alpha;
        spring.pos_vel_coef = exp_sin * inv_alpha;

        spring.vel_pos_coef = -exp_sin * alpha - omega_zeta * exp_omega_zeta_sin_over_alpha;
        spring.vel_vel_coef = exp_cos - exp_omega_zeta_sin_over_alpha
    }

    #[inline(always)]
    fn calculate_over_damped(
        delta_time: f64,
        angular_frequency: f64,
        damping_ratio: f64,
        spring: &mut Spring,
    ) {
        let za = -angular_frequency * damping_ratio;
        let zb = angular_frequency * (damping_ratio * damping_ratio - 1.0).sqrt();
        let z1 = za - zb;
        let z2 = za + zb;

        let e1 = (z1 * delta_time).exp();
        let e2 = (z2 * delta_time).exp();

        let inv_two_zb = 1.0 / (2.0 * zb); // = 1 / (z2 - z1)

        let e1_over_two_zb = e1 * inv_two_zb;
        let e2_over_two_zb = e2 * inv_two_zb;

        let z1e1_over_two_zb = z1 * e1_over_two_zb;
        let z2e2_over_two_zb = z2 * e2_over_two_zb;

        spring.pos_pos_coef = e1_over_two_zb * z2 - z2e2_over_two_zb + e2;
        spring.pos_vel_coef = -e1_over_two_zb + e2_over_two_zb;

        spring.vel_pos_coef = (z1e1_over_two_zb - z2e2_over_two_zb + e2) * z2;
        spring.vel_vel_coef = -z1e1_over_two_zb + z2e2_over_two_zb;
    }
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Spring(pos_pos_coef:{}, pos_vel_coef:{}, vel_pos_coef:{}, vel_vel_coef:{})",
            self.pos_pos_coef, self.pos_vel_coef, self.vel_pos_coef, self.vel_vel_coef
        )
    }
}