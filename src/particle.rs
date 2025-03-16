//! # Particle
//!
//! Data structure and implementation for a single point
//! particle. Particles aren't intended to be created singularly
//! but rather used within a proper object implementing ParticleSys.

use macroquad::color::Color;
use macroquad::math::Vec3;
use macroquad::prelude::draw_line_3d;
use std::time::Instant;

/// Single Particle struct. Contains the `location`, `color`, and
/// `size` of the Particle.
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
    /// length `l`.
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
            draw_line_3d(
                self.location, 
                self.end_location,
                color
                );
        } else {
            draw_line_3d(
                self.location, 
                self.end_location,
                self.color
                );
        }
        current_time > self.length
    }

    /// Reset the ellapsed time for the Particle object
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

impl Default for Particle {
    fn default() -> Self {
        Particle::new((0., 0., 0.), (0., 0., 0., 1.), 0.01, 1., false)
    }
}

fn map_color_decay(orig: Color, current: f32, total: f32) -> Color {
    Color::new(orig.r, orig.g, orig.b, orig.a * (1.0 - (current / total)))
}
