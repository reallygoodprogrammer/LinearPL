//! # LinearParticles
//!
//! Particle system generated along a single straight line.

use macroquad::color::Color;
use macroquad::math::Vec3;
use rand::rngs::ThreadRng;
use rand::{rng, Rng};
use std::slice::{Iter, IterMut};
use std::time::Instant;

use crate::particle::Particle;
use crate::particle_sys::ParticleSys;

// ***************************************
// LinearParticles
// ***************************************

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
            locations: vec![0., 1.],
            densities: vec![1.],
            colors: vec![Color::new(1., 1., 1., 1.)],
            sizes: vec![0.02],
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

    /// Return self with period `p`.
    pub fn with_period(mut self, p: f32) -> Self {
        self.period = p;
        self
    }

    /// Return self with decay `d`.
    pub fn with_decay(mut self, d: f32) -> Self {
        self.decay = d;
        self
    }

    /// Return self with locations `l`.
    pub fn with_locations(mut self, l: &[f32]) -> Self {
        self.locations = l.into();
        self
    }

    /// Return self with densities `d`.
    pub fn with_densities(mut self, d: &[f32]) -> Self {
        self.densities = d.into();
        self
    }

    /// Return self with colors `c`.
    pub fn with_colors(mut self, c: &[Color]) -> Self {
        self.colors = c.into();
        self
    }

    /// Return self with sizes `s`.
    pub fn with_sizes(mut self, s: &[f32]) -> Self {
        self.sizes = s.into();
        self
    }

    /// Return self with start-location `sl`.
    pub fn with_start_end(mut self, sl: Vec3, el: Vec3) -> Self {
        self.start_location = sl;
        self.end_location = el;
        self
    }

    /// Reverse the LinearParticles `locations`, `sizes`, `densities`, `colors`,
    /// `start_location`, `end_location`, such that the presets defined for each
    /// would create a reverse of the original graphic generated. This function
    /// does not reset the elapsed time of the object.
    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.start_location, &mut self.end_location);
        self.locations.reverse();
        self.sizes.reverse();
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

    fn start_loop(&mut self) -> Result<(), String> {
        self.setup(true)
    }

    fn start(&mut self) -> Result<(), String> {
        self.setup(false)
    }

    fn stop(&mut self) {
        self.tear_down();
    }

    fn reset_time(&mut self) {
        self.start_time = Instant::now();
    }

    fn elapsed_time(&mut self) -> Option<f32> {
        Some(self.start_time.elapsed().as_secs_f32())
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

    fn next_frame(&mut self, time: Option<f32>) -> Result<bool, String> {
        let current_time = match time {
            Some(v) => v,
            None => self.start_time.elapsed().as_secs_f32(),
        };

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
                true,
            );
            self.particles.push(p);
        }

        for p in self.particles.iter_mut() {
            (*p).draw();
        }
        self.particles.retain(|&p| !p.is_finished());

        Ok(self.start_time.elapsed().as_secs_f32() <= self.period)
    }

    fn iter(&self) -> Iter<'_, Particle> {
        self.particles.iter()
    }

    fn iter_mut(&mut self) -> IterMut<'_, Particle> {
        self.particles.iter_mut()
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
/// start time.
pub struct LinearGrp {
    pub period: f32,
    linear_particles: Vec<LinearParticles>,
    start_time: Instant,
}

impl LinearGrp {
    /// Create a new group of LinearParticles objects.
    pub fn new(period: f32, linparts: &[LinearParticles]) -> Self {
        LinearGrp {
            period,
            linear_particles: linparts.into(),
            start_time: Instant::now(),
        }
    }

    /// Return cloned self with period `p`. Because of clone,
    /// this could be an expensive operation depending on the number of
    /// LinearParticles in the group. It is typically better to simply
    /// set `self.period` to a positive value yourself.
    pub fn with_period(mut self, p: f32) -> Self {
        self.period = p;
        self
    }
}

// ***************************************
// Other functions
// ***************************************

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

// check that the period of LinearParticles is valid
fn check_period(period: f32) -> Result<(), String> {
    match period {
        p if p >= 0. => Ok(()),
        p => Err(format!(
            "value error: {} period should be positive value",
            p
        )),
    }
}

// check that the decay of LinearParticles is valid
fn check_decay(decay: f32) -> Result<(), String> {
    match decay {
        d if d >= 0. => Ok(()),
        d => Err(format!("value error: {} decay should be positive value", d)),
    }
}

// check that the locations interpolation values are valid
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

// check that the density chance values are valid
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

// check that the color interpolations are valid
fn check_colors(colors: &[Color]) -> Result<(), String> {
    if colors.is_empty() {
        return Err(String::from("empty vec: color Vec cannot be empty"));
    }
    Ok(())
}

// check that the size interpolations are valid
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
