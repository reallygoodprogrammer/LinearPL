//! # LinearParticles
//!

use crate::particle::Particle;
use macroquad::color::Color;

#[derive(Debug)]
#[allow(dead_code)]
pub struct LinearParticles {
    particles: Vec<Particle>,
    densities: Vec<f32>,
    colors: Vec<Color>,
    sizes: Vec<f32>,
    period: f32,
    initialized: bool,
    looping: bool,
    active: bool,
    count_period: f32,
}

impl LinearParticles {
    pub fn new(p: f32, dens: Vec<f32>, cols: Vec<Color>, sizs: Vec<f32>) -> Self {
        LinearParticles {
            particles: Vec::new(),
            densities: dens.clone(),
            colors: cols.clone(),
            sizes: sizs.clone(),
            period: p,
            initialized: true,
            looping: false,
            active: false,
            count_period: 0.,
        }
    }

    pub fn set_period(&mut self, p: f32) {
        self.period = p;
    }

    pub fn set_densities(&mut self, dens: Vec<f32>) {
        self.densities = dens.clone();
    }

    pub fn set_colors(&mut self, cols: Vec<Color>) {
        self.colors = cols.clone();
    }

    pub fn set_sizes(&mut self, sizs: Vec<f32>) {
        self.sizes = sizs.clone();
    }
}

impl Default for LinearParticles {
    fn default() -> Self {
        LinearParticles::new(1., vec![0.], vec![Color::new(0., 0., 0., 1.)], vec![0.01])
    }
}
