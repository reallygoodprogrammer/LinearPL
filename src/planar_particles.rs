//! # PlanarParticles
//!
//! Particle system generated along a predefined plane.
//!
//! Typically, the main functionality besides defining the parameters
//! of these Particle Systems is held within the `tdpl::particle_sys::ParticleSys`
//! trait. It's recommended to look at the documentation for `ParticleSys`
//! before using this module.

use crate::particle::Particle;
use crate::particle_sys::ParticleSys;

#[derive(Debug)]
pub struct PlanarParticles {
    particles: Vec<Particle>,
    tl_location: Vec3,
    br_location: Vec3,
    pub locations: Vec<f32>,
    pub densities: Vec<f32>,
    pub colors: Vec<Color>,
    pub period: f32,
    pub decay: f32,
    initialized: bool,
    looping: bool,
    active: bool,
    start_time: Instant,
    rand_generator: ThreadRng,
}
