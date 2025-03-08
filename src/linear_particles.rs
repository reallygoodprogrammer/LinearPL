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
    locations: Vec<f32>,
    densities: Vec<f32>,
    colors: Vec<Color>,
    sizes: Vec<f32>,
    period: f32,
    decay: f32,
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

    /// Set the period (length) of the LinearParticle's graphic.
    ///
    /// # Arguments:
    ///
    /// - p: `f32` total length of the LinearParticles particle generation.
    pub fn set_period(&mut self, p: f32) -> Result<(), &str> {
        match p {
            p if p > 0. => {
                self.period = p;
                Ok(())
            }
            _ => Err("value error: period should be positive value\n"),
        }
    }

    /// Set the decay (Particle length) of the LinearParticle's graphic.
    ///
    /// # Arguments:
    ///
    /// - d: `f32` total length of the particles generation.
    pub fn set_decay(&mut self, d: f32) -> Result<(), &str> {
        match d {
            d if d > 0. => {
                self.decay = d;
                Ok(())
            }
            _ => Err("value error: decay should be positive value\n"),
        }
    }

    /// Set locations for particles to be drawn in relation to start and
    /// end location values.
    ///
    /// # Arguments:
    ///
    /// - locs: vec of 0 to 1 `f32` values representing locations between
    ///   `start_location` and `end_location`, interpolated between when
    ///   LinearParticles is being actively drawn.
    pub fn set_locations(&mut self, locs: Vec<f32>) -> Result<(), &str> {
        if locs.is_empty() {
            return Err("empty vec: location Vec cannot be empty");
        }
        for l in locs.iter() {
            if *l > 1. || *l < 0. {
                return Err("value error: location values should be between 0 and 1 inclusive");
            };
        }
        self.locations = locs.clone();
        Ok(())
    }

    /// Set densities of particles to be generated.
    ///
    /// # Arguments:
    ///
    /// - dens: vec of 0-1 `f32` values representing chance of particle
    ///   being drawn along the line for the given frame, interpolated between when
    ///   LinearParticles is being actively drawn.
    pub fn set_densities(&mut self, dens: Vec<f32>) -> Result<(), &str> {
        if dens.is_empty() {
            return Err("empty vec: densities Vec cannot be empty");
        }
        for d in dens.iter() {
            if *d > 1. || *d < 0. {
                return Err("value error: density values should be between 0 and 1 inclusive");
            };
        }
        self.densities = dens.clone();
        Ok(())
    }

    /// Set colors for particles to be generated as.
    ///
    /// # Arguments:
    ///
    /// - cols: vec of `macroquad::color::Color` structs representing the color individual
    ///   Particle objects should be drawn with, interpolated between when LinearParticles is
    ///   being actively drawn.
    pub fn set_colors(&mut self, cols: Vec<Color>) -> Result<(), &str> {
        if cols.is_empty() {
            return Err("empty vec: color Vec cannot be empty");
        }
        self.colors = cols.clone();
        Ok(())
    }

    /// Set size for particles to be generated as.
    ///
    /// # Arguments:
    ///
    /// - sizs: vec of positive `f32` values representing the size of the individual
    ///   Particle objects when drawn, interpolated between when LinearParticles is being
    ///   actively drawn.
    pub fn set_sizes(&mut self, sizs: Vec<f32>) -> Result<(), &str> {
        if sizs.is_empty() {
            return Err("empty vec: sizes Vec cannot be empty");
        }
        for s in sizs.iter() {
            if *s > 1. || *s < 0. {
                return Err("value error: size values should be positive floats\n");
            };
        }
        self.sizes = sizs.clone();
        Ok(())
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
    pub fn r#loop(&mut self) {
        self.setup(true);
    }

    /// Set up LinearParticles into its active state.
    pub fn start(&mut self) {
        self.setup(false);
    }

    /// Tear down and deactivate LinearParticles object.
    pub fn stop(&mut self) {
        self.tear_down();
    }

    fn reset_time(&mut self) {
        self.start_time = Instant::now();
    }

    fn setup(&mut self, should_loop: bool) {
        self.particles.clear();
        self.looping = should_loop;
        self.active = true;
        self.initialized = true;
        self.reset_time();
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
    pub fn next_frame(&mut self) -> Result<bool, &'static str> {
        let current_time = self.start_time.elapsed().as_secs_f32();

        let gen_flag = map_float_value(&self.densities, current_time)?;
        if self.should_generate(gen_flag) {
            let p = Particle::new(
                map_location(
                    &self.locations,
                    self.start_location,
                    self.end_location,
                    current_time,
                )?,
                map_color_value(&self.colors, current_time)?,
                map_float_value(&self.sizes, current_time)?,
                self.decay,
            );
            self.particles.push(p);
        }

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

// these are some helper functions for scaling different types of values
//
fn map_float_value(values: &[f32], elapsed: f32) -> Result<f32, &'static str> {
    let ratio = values.len() as f32 / elapsed;
    let low = (elapsed * ratio).floor() as usize;
    let high = (elapsed * ratio).ceil() as usize;

    let first_value = match values.get(low) {
        Some(val) => val,
        None => {
            return Err("unexpected error in map_float_value indexing!");
        }
    };

    if low == high {
        Ok(*first_value)
    } else {
        let val_ratio = elapsed - (low as f32);
        match values.get(high) {
            Some(val) => Ok((first_value * val_ratio) + (val * (1.0 - val_ratio))),
            None => Err("unexpected error in map_float_value indexing!"),
        }
    }
}

fn map_color_value(colors: &[Color], elapsed: f32) -> Result<(f32, f32, f32, f32), &'static str> {
    let ratio = colors.len() as f32 / elapsed;
    let low = (elapsed * ratio).floor() as usize;
    let high = (elapsed * ratio).ceil() as usize;

    let first_value = match colors.get(low) {
        Some(val) => val,
        None => {
            return Err("unexpected error in map_Color_value indexing!");
        }
    };

    if low == high {
        Ok((first_value.r, first_value.g, first_value.b, first_value.a))
    } else {
        let val_ratio = elapsed - (low as f32);
        match colors.get(high) {
            Some(val) => Ok((
                (first_value.r * val_ratio) + (val.r * (1.0 - val_ratio)),
                (first_value.g * val_ratio) + (val.g * (1.0 - val_ratio)),
                (first_value.b * val_ratio) + (val.b * (1.0 - val_ratio)),
                (first_value.a * val_ratio) + (val.a * (1.0 - val_ratio)),
            )),
            None => Err("unexpected error in map_Color_value indexing!"),
        }
    }
}

fn map_location(
    locations: &[f32],
    start_location: Vec3,
    end_location: Vec3,
    elapsed: f32,
) -> Result<(f32, f32, f32), &'static str> {
    let ratio = map_float_value(locations, elapsed)?;
    let vratio = Vec3::new(ratio, ratio, ratio);
    Ok(((start_location * vratio) + ((Vec3::ONE - vratio) * end_location)).into())
}
