use std::fmt;
use std::fmt::Formatter;
use std::time::Duration;

#[derive(Default)]
pub struct Spring {
    pos_pos_coef: f64,
    pos_vel_coef: f64,
    vel_pos_coef: f64,
    vel_vel_coef: f64,
}

const EPSILON: f64 = 0.00000001;

pub fn fps(n: u64) -> f64 {
    let duration = Duration::new(0, n as u32).as_nanos();
    let second = Duration::from_secs(1).as_nanos();

    let result = (((second / duration) as f64 / 1000000.0) / 1000.0) as f64;

    return result;
}

impl Spring {
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

        return spring;
    }

    // update updates position and velocity values against a given target value.
    // Call this after calling new_spring to update values.
    pub fn update(&mut self, pos: f64, vel: f64, equilibrium_pos: f64) -> (f64, f64) {
        let old_pos = pos - equilibrium_pos; // update in equilibrium relative space
        let old_vel = vel;

        let new_pos = old_pos * self.pos_pos_coef + old_vel * self.pos_vel_coef + equilibrium_pos;
        let new_vel = old_pos * self.vel_pos_coef + old_vel * self.vel_vel_coef;

        return (new_pos, new_vel);
    }

    #[inline]
    fn calculate_critically_damped(delta_time: f64, angular_frequency: f64, spring: &mut Spring) {
        let exp_term = (-angular_frequency * delta_time).exp(); // math.Exp(-angular_frequency * delta_time);
        let time_exp = delta_time * exp_term;
        let time_exp_freq = time_exp * angular_frequency;

        spring.pos_pos_coef = time_exp_freq + exp_term;
        spring.pos_vel_coef = time_exp;

        spring.vel_pos_coef = -angular_frequency * time_exp_freq;
        spring.vel_vel_coef = -time_exp_freq + exp_term
    }

    #[inline]
    fn calculate_under_damped(delta_time: f64, angular_frequency: f64, damping_ratio: f64, spring: &mut Spring) {
        let omega_zeta = angular_frequency * damping_ratio;
        let alpha = angular_frequency * (1.0 - damping_ratio * damping_ratio).sqrt();

        let exp_term = (-omega_zeta * delta_time).exp(); //math.Exp(-omega_zeta * delta_time);
        let cos_term = (alpha * delta_time).cos(); // math.Cos(alpha * delta_time);
        let sin_term = (alpha * delta_time).sin(); //math.Sin(alpha * delta_time);

        let inv_alpha = 1.0 / alpha;

        let exp_sin = exp_term * sin_term;
        let exp_cos = exp_term * cos_term;
        let exp_omega_zeta_sin_over_alpha = exp_term * omega_zeta * sin_term * inv_alpha;


        spring.pos_pos_coef = exp_cos + exp_omega_zeta_sin_over_alpha;
        spring.pos_vel_coef = exp_sin * inv_alpha;

        spring.vel_pos_coef = -exp_sin * alpha - omega_zeta * exp_omega_zeta_sin_over_alpha;
        spring.vel_vel_coef = exp_cos - exp_omega_zeta_sin_over_alpha
    }

    #[inline]
    fn calculate_over_damped(delta_time: f64, angular_frequency: f64, damping_ratio: f64, spring: &mut Spring) {
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
        write!(f, "Sprite pos_pos_coef:{}, pos_vel_coef:{}, vel_pos_coef:{}, vel_vel_coef:{}",
               self.pos_pos_coef, self.pos_vel_coef, self.vel_pos_coef, self.vel_vel_coef)
    }
}