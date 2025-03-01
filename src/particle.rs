//! # Particle
//!
//! Data structure and implementation for a single point
//! particle. Particles aren't intended to be created singularly
//! but rather within a LinearParticles, PlanarParticles, or
//! SpatialParticles object.

use macroquad::color::Color;
use macroquad::math::Vec3;
use macroquad::prelude::draw_cube;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// # Particle
///

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub location: Vec3,
    pub color: Color,
    pub size: f32,
}

impl Particle {
    /// Instantiate a new Particle at location = (0, 0, 0)
    /// with a black color.
    pub fn new((x, y, z): (f32, f32, f32), (r, g, b, a): (f32, f32, f32, f32), s: f32) -> Self {
        Particle {
            location: Vec3::new(x, y, z),
            color: Color::new(r, g, b, a),
            size: s,
        }
    }

    #[inline]
    pub fn add_location(mut self, x: f32, y: f32, z: f32) -> Self {
        self.location.x += x;
        self.location.y += y;
        self.location.z += z;
        self
    }

    #[inline]
    pub fn sub_location(mut self, x: f32, y: f32, z: f32) -> Self {
        self.location.x -= x;
        self.location.y -= y;
        self.location.z -= z;
        self
    }

    #[inline]
    pub fn set_location(&mut self, x: f32, y: f32, z: f32) {
        self.location = Vec3::new(x, y, z);
    }

    #[inline]
    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = Color::new(r, g, b, a);
    }

    #[inline]
    pub fn set_size(&mut self, s: f32) {
        self.size = s;
    }

    #[inline]
    pub fn draw(&self) {
        draw_cube(self.location, Vec3::splat(self.size), None, self.color);
    }
}

impl Add<Vec3> for Particle {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        self.add_location(rhs.x, rhs.y, rhs.z)
    }
}

impl AddAssign<Vec3> for Particle {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        *self = self.add_location(rhs.x, rhs.y, rhs.z);
    }
}

impl Add<[f32; 3]> for Particle {
    type Output = Self;

    #[inline]
    fn add(self, rhs: [f32; 3]) -> Self::Output {
        self.add_location(rhs[0], rhs[1], rhs[2])
    }
}

impl AddAssign<[f32; 3]> for Particle {
    #[inline]
    fn add_assign(&mut self, rhs: [f32; 3]) {
        *self = self.add_location(rhs[0], rhs[1], rhs[2]);
    }
}

impl Default for Particle {
    fn default() -> Self {
        Particle::new((0., 0., 0.), (0., 0., 0., 1.), 0.01)
    }
}

impl Sub<Vec3> for Particle {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        self.sub_location(rhs.x, rhs.y, rhs.z)
    }
}

impl SubAssign<Vec3> for Particle {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = self.sub_location(rhs.x, rhs.y, rhs.z);
    }
}

impl Sub<[f32; 3]> for Particle {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: [f32; 3]) -> Self::Output {
        self.sub_location(rhs[0], rhs[1], rhs[2])
    }
}

impl SubAssign<[f32; 3]> for Particle {
    #[inline]
    fn sub_assign(&mut self, rhs: [f32; 3]) {
        *self = self.sub_location(rhs[0], rhs[1], rhs[2]);
    }
}
