//! # Single Particle
//!
//! Data structure and implementation for a single point
//! particle. Particles aren't intended to be created singularly
//! but rather used within a proper object implementing ParticleSys.
//!
//! If you are wanting to implement your own struct with the
//! ParticleSys trait such that it generates visible particles
//! itself, you should use the `Particles` struct defined in this module.

use macroquad::color::Color;
use macroquad::math::Vec3;
use macroquad::prelude::draw_line_3d;
use std::slice::{Iter, IterMut};
use std::time::Instant;

use crate::particle_sys::ParticleSys;
use crate::util::map_color_decay;

/// Single Particle struct. Contains the `location` and `color`.
/// Because `macroquad` does not support 3 dimensional points
/// or single pixels, a `Particle` is implemented as a small
/// line. This is why it contains the `end_location` member, as
/// it is used as the ending point of the line.
///
/// This design choice can be embraced fully when creating a particle
/// system that operates on a continuous line by setting the `end_location`
/// somewhere near the the next point of the particle system to imitate
/// continuity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    location: Vec3,
    end_location: Vec3,
    color: Color,
    length: f32,
    sloped: bool,
    start_time: Instant,
}

impl Particle {
    /// Instantiate a new Particle at `(x, y, z)` location
    /// with `(r, g, b, a)` color and `s` size, length `l`.
    /// `sloped` determines if the particle's opacity fades
    /// out or not.
    pub fn new(
        (x, y, z): (f32, f32, f32),
        (r, g, b, a): (f32, f32, f32, f32),
        size: f32,
        length: f32,
        sloped: bool,
    ) -> Self {
        let l = Vec3::new(x, y, z);
        let el = l + Vec3::splat(size);
        Particle {
            location: l,
            end_location: el,
            color: Color::new(r, g, b, a),
            length,
            sloped,
            start_time: Instant::now(),
        }
    }

    /// Instantiate a new Particle at `(x, y, z)` location
    /// and ending location `(x, y, z)` with `(r, g, b, a)` color,
    /// length `l`. `sloped` determines if the particle's opacity fades
    /// out or not.
    pub fn new_line(
        (x, y, z): (f32, f32, f32),
        (xe, ye, ze): (f32, f32, f32),
        (r, g, b, a): (f32, f32, f32, f32),
        length: f32,
        sloped: bool,
    ) -> Self {
        Particle {
            location: Vec3::new(x, y, z),
            end_location: Vec3::new(xe, ye, ze),
            color: Color::new(r, g, b, a),
            length,
            sloped,
            start_time: Instant::now(),
        }
    }

    /// Add the `x`, `y`, `z` argument values to the location of Particle.
    #[inline]
    pub fn add_location(mut self, x: f32, y: f32, z: f32) -> Self {
        self.location.x += x;
        self.location.y += y;
        self.location.z += z;
        self
    }

    /// Subtract the `x`, `y`, `z` argument values to the location of Particle.
    #[inline]
    pub fn sub_location(mut self, x: f32, y: f32, z: f32) -> Self {
        self.location.x -= x;
        self.location.y -= y;
        self.location.z -= z;
        self
    }

    /// Set the location of the particle to `x`, `y`, `z` argument.
    #[inline]
    pub fn set_location(&mut self, x: f32, y: f32, z: f32) {
        self.location = Vec3::new(x, y, z);
    }

    /// Set the color of the particle to `r`, `g`, `b`, `a` argument.
    #[inline]
    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = Color::new(r, g, b, a);
    }

    /// Draw the Particle within the macroquad world coords. Returns
    /// `true` if Particle has surpassed its length, else `false`.
    #[inline]
    pub fn draw(&mut self) -> bool {
        let current_time = self.start_time.elapsed().as_secs_f32();
        if self.sloped {
            let color = map_color_decay(self.color, current_time, self.length);
            draw_line_3d(self.location, self.end_location, color);
        } else {
            draw_line_3d(self.location, self.end_location, self.color);
        }
        current_time > self.length
    }

    /// Reset the ellapsed time for the Particle object
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

impl ParticleSys for Particle {
    type T = Particle;

    fn is_active(&self) -> bool {
        true
    }

    fn is_looping(&self) -> bool {
        false
    }

    fn is_initialized(&mut self) -> bool {
        true
    }

    fn reset_time(&mut self) {
        self.reset()
    }

    fn elapsed_time(&mut self) -> Option<f32> {
        Some(self.start_time.elapsed().as_secs_f32())
    }

    fn setup(&mut self, _should_loop: bool, _p: Option<f32>) -> Result<(), String> {
        self.reset();
        Ok(())
    }

    fn tear_down(&mut self) {}

    fn next_frame(&mut self, _time: Option<f32>) -> Result<bool, String> {
        Ok(self.draw())
    }

    fn iter(&self) -> Option<Iter<'_, Self::T>> {
        None
    }

    fn iter_mut(&mut self) -> Option<IterMut<'_, Self::T>> {
        None
    }

    fn with_period(mut self, p: f32) -> Self {
        self.length = p;
        self
    }
}

impl Default for Particle {
    fn default() -> Self {
        Particle::new((0., 0., 0.), (0., 0., 0., 1.), 0.01, 1., false)
    }
}
