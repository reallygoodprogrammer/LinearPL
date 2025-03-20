//! # LinearParticles and LinearGrp
//!
//! Particle system generated along a single straight line.
//!
//! Typically, the main functionality besides defining the parameters
//! of these Particle Systems is held within the `linearpl::particle_sys::ParticleSys`
//! trait. It's recommended to look at the documentation for `ParticleSys`
//! before using this module.

use macroquad::color::Color;
use macroquad::math::Vec3;
use macroquad::prelude::get_fps;
use rand::rngs::ThreadRng;
use rand::{rng, Rng};
use std::slice::{Iter, IterMut};
use std::time::Instant;

use crate::particle::Particle;
use crate::particle_sys::ParticleSys;
use crate::util::{
    check_colors, check_decay, check_densities, check_locations, check_period, map_color_value,
    map_float_value, map_location,
};

// ***************************************
// LinearParticles
// ***************************************

/// LinearParticle system. User should be in charge of setting
/// appropriate `locations`, `densities`, and `colors`
/// such that their values are interpolated over the defined `period`
/// in seconds. `decay` refers to the amount of time the particles
/// generated stay visible.
#[derive(Debug, Clone)]
pub struct LinearParticles {
    particles: Vec<Particle>,
    start_location: Vec3,
    end_location: Vec3,
    locations: Vec<f32>,
    densities: Vec<f32>,
    colors: Vec<Color>,
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
            locations: vec![0., 1.],
            densities: vec![1.],
            colors: vec![Color::new(1., 1., 1., 1.)],
            period: 1.,
            decay: 0.09,
            initialized: false,
            looping: false,
            active: false,
            start_time: Instant::now(),
            rand_generator: rng(),
        }
    }

    // used in density calculations
    fn should_generate(&mut self, chance: f32) -> bool {
        chance > self.rand_generator.random_range(0.0..1.0)
    }

    /// Return self (consuming it) with decay `d`.
    pub fn with_decay(mut self, d: f32) -> Result<Self, String> {
        check_decay(d)?;
        self.decay = d;
        Ok(self)
    }

    /// Return self (consuming it) with locations `l`.
    pub fn with_locations(mut self, l: &[f32]) -> Result<Self, String> {
        check_locations(l)?;
        self.locations = l.into();
        Ok(self)
    }

    /// Return self (consuming it) with densities `d`.
    pub fn with_densities(mut self, d: &[f32]) -> Result<Self, String> {
        check_densities(d)?;
        self.densities = d.into();
        Ok(self)
    }

    /// Return self (consuming it) with colors `c`.
    pub fn with_colors(mut self, c: &[Color]) -> Result<Self, String> {
        check_colors(c)?;
        self.colors = c.into();
        Ok(self)
    }

    /// Return self (consuming it) with start-location `sl`, ending location `el.
    pub fn with_start_end(mut self, sl: Vec3, el: Vec3) -> Result<Self, String> {
        self.start_location = sl;
        self.end_location = el;
        Ok(self)
    }

    /// Return clone of self with decay `d`.
    pub fn clone_with_decay(&self, d: f32) -> Result<Self, String> {
        self.clone().with_decay(d)
    }

    /// Return clone self with locations `l`.
    pub fn clone_with_locations(&self, l: &[f32]) -> Result<Self, String> {
        self.clone().with_locations(l)
    }

    /// Return clone self with densities `d`.
    pub fn clone_with_densities(&self, d: &[f32]) -> Result<Self, String> {
        self.clone().with_densities(d)
    }

    /// Return clone self with colors `c`.
    pub fn clone_with_colors(&self, c: &[Color]) -> Result<Self, String> {
        self.clone().with_colors(c)
    }

    /// Return clone self with start-location `sl`, ending location `el`.
    pub fn clone_with_start_end(&self, sl: Vec3, el: Vec3) -> Result<Self, String> {
        self.clone().with_start_end(sl, el)
    }

    /// Reverse the LinearParticles `locations`, `sizes`, `densities`, `colors`,
    /// `start_location`, `end_location`, such that the presets defined for each
    /// would create a reverse of the original graphic generated. This function
    /// does not reset the elapsed time of the object.
    pub fn reversed(mut self) {
        std::mem::swap(&mut self.start_location, &mut self.end_location);
        self.locations.reverse();
        self.densities.reverse();
        self.colors.reverse();
    }
}

// ***************************************
// Impl's for LinearParticles

impl ParticleSys for LinearParticles {
    type T = Particle;

    fn is_active(&self) -> bool {
        self.active
    }

    fn is_looping(&self) -> bool {
        self.active && self.looping
    }

    fn is_initialized(&mut self) -> bool {
        self.initialized
    }

    fn reset_time(&mut self) {
        self.start_time = Instant::now();
    }

    fn elapsed_time(&mut self) -> Option<f32> {
        Some(self.start_time.elapsed().as_secs_f32())
    }

    fn setup(&mut self, should_loop: bool, p: Option<f32>) -> Result<(), String> {
        self.period = match p {
            Some(p) => {
                check_period(p)?;
                p
            }
            None => self.period,
        };

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

    fn next_frame(&mut self, time: Option<f32>) -> Result<bool, String> {
        let current_time = match time {
            Some(v) => v,
            None => self.start_time.elapsed().as_secs_f32(),
        };

        if current_time <= self.period {
            let gen_flag = map_float_value(&self.densities, current_time, self.period)?;
            if self.should_generate(gen_flag) {
                let nft = 4.0 / get_fps() as f32;
                let p = Particle::new_line(
                    map_location(
                        &self.locations,
                        self.start_location,
                        self.end_location,
                        current_time,
                        self.period,
                    )?,
                    map_location(
                        &self.locations,
                        self.start_location,
                        self.end_location,
                        current_time + nft,
                        self.period,
                    )?,
                    map_color_value(&self.colors, current_time, self.period)?,
                    self.decay,
                    true,
                )?;
                self.particles.push(p);
            }
        }

        self.particles.retain_mut(|p| !(*p).draw());
        Ok(self.start_time.elapsed().as_secs_f32() <= self.period)
    }

    fn iter(&self) -> Option<Iter<'_, Self::T>> {
        Some(self.particles.iter())
    }

    fn iter_mut(&mut self) -> Option<IterMut<'_, Self::T>> {
        Some(self.particles.iter_mut())
    }

    fn with_period(mut self, p: f32) -> Result<Self, String> {
        check_period(p)?;
        self.period = p;
        Ok(self)
    }
}

impl Default for LinearParticles {
    fn default() -> Self {
        LinearParticles::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.))
    }
}

// ***************************************
// LinearGrp
// ***************************************

/// Group of LinearParticles objects with a synced period and
/// start time. This is similar to `linearpl::groups::SyncGrp` but
/// only allowed to contain LinearParticles objects.
#[derive(Debug, Clone)]
pub struct LinearGrp {
    pub period: f32,
    linear_particles: Vec<LinearParticles>,
    active: bool,
    looping: bool,
    initialized: bool,
    start_time: Instant,
}

impl LinearGrp {
    /// Create a new group of LinearParticles objects.
    pub fn new(period: f32, linparts: &[LinearParticles]) -> Self {
        LinearGrp {
            period,
            linear_particles: linparts.into(),
            start_time: Instant::now(),
            active: false,
            looping: false,
            initialized: false,
        }
    }

    /// Returns self with contained LinearParticles `linparts`.
    pub fn with_systems(mut self, linparts: &[LinearParticles]) -> Self {
        self.linear_particles = linparts.into();
        self
    }
}

impl ParticleSys for LinearGrp {
    type T = LinearParticles;

    fn is_active(&self) -> bool {
        self.active
    }

    fn is_looping(&self) -> bool {
        self.active && self.looping
    }

    fn is_initialized(&mut self) -> bool {
        self.initialized
    }

    fn reset_time(&mut self) {
        self.start_time = Instant::now();
    }

    fn elapsed_time(&mut self) -> Option<f32> {
        Some(self.start_time.elapsed().as_secs_f32())
    }

    fn setup(&mut self, should_loop: bool, p: Option<f32>) -> Result<(), String> {
        self.period = match p {
            Some(p) => {
                check_period(p)?;
                p
            }
            None => self.period,
        };

        for ps in self.linear_particles.iter_mut() {
            ps.setup(should_loop, Some(self.period))?;
        }

        self.looping = should_loop;
        self.active = true;
        self.initialized = true;
        self.reset_time();
        Ok(())
    }

    fn tear_down(&mut self) {
        for ps in self.linear_particles.iter_mut() {
            ps.tear_down();
        }

        self.active = false;
        self.initialized = false;
    }

    fn next_frame(&mut self, time: Option<f32>) -> Result<bool, String> {
        let current_time = match time {
            None => Some(self.start_time.elapsed().as_secs_f32()),
            v => v,
        };

        for ps in self.linear_particles.iter_mut() {
            ps.next_frame(current_time)?;
        }

        Ok(current_time <= Some(self.period))
    }

    fn iter(&self) -> Option<Iter<'_, Self::T>> {
        Some(self.linear_particles.iter())
    }

    fn iter_mut(&mut self) -> Option<IterMut<'_, Self::T>> {
        Some(self.linear_particles.iter_mut())
    }

    fn with_period(mut self, p: f32) -> Result<Self, String> {
        check_period(p)?;
        self.period = p;
        Ok(self)
    }
}

impl Default for LinearGrp {
    fn default() -> Self {
        LinearGrp::new(1.0, &[])
    }
}
