//! # LinearParticles
//!
//! Particle effect system generated along a single line.

use macroquad::color::Color;
use macroquad::math::Vec3;
use std::time::Instant;

use crate::particle::Particle;

/// # LinearParticles
///
/// LinearParticle system. User should be in charge of setting
/// appropriate `locations`, `densities`, `colors`, `sizes`
/// such that their values are interpolated over the defined `period`
/// in seconds.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LinearParticles {
    particles: Vec<Particle>,
    start_location: Vec3,
    end_location: Vec3,
    locations: Vec<f32>,
    densities: Vec<f32>,
    colors: Vec<Color>,
    sizes: Vec<f32>,
    period: f32,
    initialized: bool,
    looping: bool,
    active: bool,
    start_time: Instant,
}

impl LinearParticles {
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
            initialized: false,
            looping: false,
            active: false,
            start_time: Instant::now(),
        }
    }

    /// # Set period
    ///
    /// ## Arguments:
    ///
    /// * p: `f32` total length of the LinearParticles particle generation.
    pub fn set_period(&mut self, p: f32) -> Result<(), &str> {
        match p {
            p if p > 0. => {
                self.period = p;
                Ok(())
            }
            _ => Err("value error: period should be positive value\n"),
        }
    }

    /// # Set locations
    ///
    /// ## Arguments:
    ///
    /// locs: vec of 0 to 1 `f32` values representing locations between
    /// `start_location` and `end_location`, interpolated between when
    /// LinearParticles is being actively drawn.
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

    /// # Set densities
    ///
    /// ## Arguments:
    ///
    /// dens: vec of 0-1 `f32` values representing chance of particle
    /// being drawn along the line for the given frame, interpolated between when
    /// LinearParticles is being actively drawn.
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

    /// # Set Colors
    ///
    /// ## Arguments:
    ///
    /// cols: vec of `macroquad::color::Color` structs representing the color individual
    /// Particle objects should be drawn with, interpolated between when LinearParticles is
    /// being actively drawn.
    pub fn set_colors(&mut self, cols: Vec<Color>) -> Result<(), &str> {
        if cols.is_empty() {
            return Err("empty vec: color Vec cannot be empty");
        }
        self.colors = cols.clone();
        Ok(())
    }

    /// # Set Sizes
    ///
    /// ## Arguments:
    ///
    /// sizs: vec of positive `f32` values representing the size of the individual
    /// Particle objects when drawn, interpolated between when LinearParticles is being
    /// actively drawn.
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

    /// # Is Active
    ///
    /// Return `true` if LinearParticles is in active state. Else `false`.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// # Is Looping
    ///
    /// Return `true` if LinearParticles is in active looping state. Else `false`.
    pub fn is_looping(&self) -> bool {
        self.active && self.looping
    }

    /// # Loop
    ///
    /// Set up LinearParticles into its looping active state.
    pub fn r#loop(&mut self) {
        self.setup(true);
    }

    /// # Start
    ///
    /// Set up LinearParticles into its active state.
    pub fn start(&mut self) {
        self.setup(false);
    }

    /// # Stop
    ///
    /// Tear down and deactivate LinearParticles object.
    pub fn stop(&mut self) {
        self.tear_down();
    }

    fn setup(&mut self, should_loop: bool) {
        self.particles.clear();
        self.looping = should_loop;
        self.active = true;
        self.start_time = Instant::now();
        self.initialized = true;
    }

    fn tear_down(&mut self) {
        self.active = false;
        self.initialized = false;
    }
}

impl Default for LinearParticles {
    fn default() -> Self {
        LinearParticles::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.))
    }
}
