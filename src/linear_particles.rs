//! # LinearParticles
//!
//! Particle effect system generated along a single line.

use macroquad::color::Color;
use macroquad::math::Vec3;
use rand::rngs::ThreadRng;
use rand::{rng, Rng};
use std::time::Instant;

use crate::particle::Particle;

/// # LinearParticles
///
/// LinearParticle system. User should be in charge of setting
/// appropriate `locations`, `densities`, `colors`, `sizes`
/// such that their values are interpolated over the defined `period`
/// in seconds through provided methods.
#[derive(Debug, Clone)]
pub struct LinearParticles {
    particles: Vec<Particle>,
    start_location: Vec3,
    end_location: Vec3,
    pub locations: Vec<f32>,
    pub densities: Vec<f32>,
    pub colors: Vec<Color>,
    pub sizes: Vec<f32>,
    pub period: f32,
    pub decay: f32,
    initialized: bool,
    looping: bool,
    active: bool,
    start_time: Instant,
    rand_generator: ThreadRng,
}

impl LinearParticles {
    /// Create a new LinearParticles struct with a starting location of
    /// `start_loc` and an ending location of `end_loc`.
    pub fn new(start_loc: Vec3, end_loc: Vec3) -> Self {
        LinearParticles {
            start_location: start_loc,
            end_location: end_loc,
            particles: Vec::new(),
            locations: Vec::new(),
            densities: Vec::new(),
            colors: Vec::new(),
            sizes: Vec::new(),
            period: 0.,
            decay: 0.2,
            initialized: false,
            looping: false,
            active: false,
            start_time: Instant::now(),
            rand_generator: rng(),
        }
    }

    /// Check if LinearParticles is active.
    /// Returns `true` if LinearParticles is in active state. Else `false`.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Check if LinearParticles is looping.
    /// Return `true` if LinearParticles is in active looping state. Else `false`.
    pub fn is_looping(&self) -> bool {
        self.active && self.looping
    }

    /// Set up LinearParticles into its looping active state.
    pub fn start_loop(&mut self) -> Result<(), String> {
        self.setup(true)
    }

    /// Set up LinearParticles into its active state.
    pub fn start(&mut self) -> Result<(), String> {
        self.setup(false)
    }

    /// Tear down and deactivate LinearParticles object.
    pub fn stop(&mut self) {
        self.tear_down();
    }

    fn reset_time(&mut self) {
        self.start_time = Instant::now();
    }

    fn setup(&mut self, should_loop: bool) -> Result<(), String> {
        check_densities(&self.densities)?;
        check_locations(&self.locations)?;
        check_colors(&self.colors)?;
        check_sizes(&self.sizes)?;
        check_period(self.period)?;
        check_decay(self.decay)?;

        self.particles.clear();
        self.looping = should_loop;
        self.active = true;
        self.initialized = true;
        self.reset_time();
        Ok(())
    }

    fn tear_down(&mut self) {
        self.active = false;
        self.initialized = false;
    }

    fn should_generate(&mut self, chance: f32) -> bool {
        chance > self.rand_generator.random_range(0.0..1.0)
    }

    /// Display the next frame available from the LinearParticle
    /// system defined by users previous called settings.
    ///
    /// # Returns:
    ///
    /// - `true` if LinearParticle is still 'active' in next frame,
    /// - `false` otherwise
    pub fn next_frame(&mut self) -> Result<bool, String> {
        let current_time = self.start_time.elapsed().as_secs_f32();

        let gen_flag = map_float_value(&self.densities, current_time, self.period)?;
        if self.should_generate(gen_flag) {
            let p = Particle::new(
                map_location(
                    &self.locations,
                    self.start_location,
                    self.end_location,
                    current_time,
                    self.period,
                )?,
                map_color_value(&self.colors, current_time, self.period)?,
                map_float_value(&self.sizes, current_time, self.period)?,
                self.decay,
            );
            self.particles.push(p);
        }

        for p in self.particles.iter_mut() {
            (*p).draw();
        }
        self.particles.retain(|&p| !p.is_finished());

        if self.start_time.elapsed().as_secs_f32() > self.period {
            if self.looping {
                self.reset_time();
            } else {
                self.tear_down();
            }
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl Default for LinearParticles {
    fn default() -> Self {
        LinearParticles::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.))
    }
}

// find the linearly interpolated value from 'values' given the ratio 'elapsed' / 'total'
fn map_float_value(values: &[f32], elapsed: f32, total: f32) -> Result<f32, String> {
    let ratio = elapsed / total;
    let len = values.len() - 1;
    let vratio = len as f32 * ratio;
    let low = (vratio.floor()) as usize;
    let high = (vratio.ceil()) as usize;

    let low = if low > len { len } else { low };
    let high = if high > len { len } else { high };

    let first_value = match values.get(low) {
        Some(val) => val,
        None => {
            return Err(format!(
                "map_float_values indexing error: {} of {}",
                low, len
            ));
        }
    };

    if low == high {
        Ok(*first_value)
    } else {
        match values.get(high) {
            Some(val) => {
                let vratio_norm = high as f32 - vratio;
                Ok((first_value * vratio_norm) + (val * (1.0 - vratio_norm)))
            }
            None => Err(format!(
                "map_float_values indexing error: {} of {}",
                high, len
            )),
        }
    }
}

#[test]
fn map_float_value_test() {
    let values = vec![0.0, 1.0];
    assert_eq!(map_float_value(&values, 0.0, 1.0).unwrap_or(-1.0), 0.0);
    assert_eq!(
        map_float_value(&values, 2.0 / 3.0, 1.0).unwrap_or(-1.0),
        2.0 / 3.0
    );

    let values = vec![1.0, 0.0, 0.5, 0.0];
    assert_eq!(map_float_value(&values, 0.5, 1.0).unwrap_or(-1.0), 0.25);
}

// find the linearly interpolated color from 'colors' given the ratio 'elapsed' / 'total'
fn map_color_value(
    colors: &[Color],
    elapsed: f32,
    total: f32,
) -> Result<(f32, f32, f32, f32), String> {
    let ratio = elapsed / total;
    let len = colors.len() - 1;
    let vratio = len as f32 * ratio;
    let low = ((len as f32 * ratio).floor()) as usize;
    let high = ((len as f32 * ratio).ceil()) as usize;

    let low = if low > len { len } else { low };
    let high = if high > len { len } else { high };

    let first_value = match colors.get(low) {
        Some(val) => val,
        None => {
            return Err(format!(
                "map_color_value indexing error: {} of {}",
                low, len
            ));
        }
    };

    if low == high {
        Ok((first_value.r, first_value.g, first_value.b, first_value.a))
    } else {
        match colors.get(high) {
            Some(val) => {
                let vratio_norm = high as f32 - vratio;
                Ok((
                    (first_value.r * vratio_norm) + (val.r * (1.0 - vratio_norm)),
                    (first_value.g * vratio_norm) + (val.g * (1.0 - vratio_norm)),
                    (first_value.b * vratio_norm) + (val.b * (1.0 - vratio_norm)),
                    (first_value.a * vratio_norm) + (val.a * (1.0 - vratio_norm)),
                ))
            }
            None => Err(format!(
                "map_color_value indexing error: {} of {}",
                high, len
            )),
        }
    }
}

// Find the linearly interpolated location from 'start_location' to 'end_location'
// given the 'locations' values and the ratio 'elapsed' / 'period'
fn map_location(
    locations: &[f32],
    start_location: Vec3,
    end_location: Vec3,
    elapsed: f32,
    period: f32,
) -> Result<(f32, f32, f32), String> {
    let ratio = map_float_value(locations, elapsed, period)?;
    let vratio = Vec3::new(ratio, ratio, ratio);
    let v = (start_location * vratio) + ((Vec3::ONE - vratio) * end_location);
    Ok(v.into())
}

fn check_period(period: f32) -> Result<(), String> {
    match period {
        p if p >= 0. => Ok(()),
        p => Err(format!(
            "value error: {} period should be positive value",
            p
        )),
    }
}

fn check_decay(decay: f32) -> Result<(), String> {
    match decay {
        d if d >= 0. => Ok(()),
        d => Err(format!("value error: {} decay should be positive value", d)),
    }
}

fn check_locations(locations: &[f32]) -> Result<(), String> {
    if locations.is_empty() {
        return Err(String::from("empty vec: location Vec cannot be empty"));
    }
    for l in locations.iter() {
        if *l > 1. || *l < 0. {
            return Err(format!(
                "value error: {} location interpolation should be between 0 and 1 inclusive",
                *l
            ));
        };
    }
    Ok(())
}

fn check_densities(densities: &[f32]) -> Result<(), String> {
    if densities.is_empty() {
        return Err(String::from("empty vec: densities Vec cannot be empty"));
    }
    for d in densities.iter() {
        if *d > 1. || *d < 0. {
            return Err(format!(
                "value error: {} density value should be between 0 and 1 inclusive",
                *d
            ));
        };
    }
    Ok(())
}

fn check_colors(colors: &[Color]) -> Result<(), String> {
    if colors.is_empty() {
        return Err(String::from("empty vec: color Vec cannot be empty"));
    }
    Ok(())
}

fn check_sizes(sizes: &[f32]) -> Result<(), String> {
    if sizes.is_empty() {
        return Err(String::from("empty vec: sizes Vec cannot be empty"));
    }
    for s in sizes.iter() {
        if *s < 0. {
            return Err(format!(
                "value error: {} size value should be positive floats",
                *s
            ));
        };
    }
    Ok(())
}
