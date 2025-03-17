//! # Groups of Particles
//!
//! Defines different groups of particle systems synced
//! in different ways. All of these groups main functionality
//! is handled by the `ParticleSys` trait and it is recommended
//! to review documentation for it to learn how to interact with
//! these objects fully.

use std::slice::{Iter, IterMut};
use std::time::Instant;

use crate::particle_sys::ParticleSys;
use crate::util::check_period;

/// Group of objects implementing ParticleSys
/// that are synchronously ran together with a
/// shared period and clock.
pub struct SyncGrp<P: ParticleSys> {
    pub period: f32,
    parts: Vec<P>,
    start_time: Instant,
    active: bool,
    looping: bool,
    initialized: bool,
}

impl<P: ParticleSys + std::clone::Clone> SyncGrp<P> {
    /// Create a new SyncGrp object.
    pub fn new(period: f32, sliceparts: &[P]) -> Self {
        SyncGrp {
            period,
            parts: sliceparts.into(),
            start_time: Instant::now(),
            active: false,
            looping: false,
            initialized: false,
        }
    }

    /// Return self with ParticleSys obj's `sliceparts`.
    pub fn with_systems(mut self, sliceparts: &[P]) -> Self {
        self.parts = sliceparts.into();
        self
    }
}

impl<P> ParticleSys for SyncGrp<P>
where
    P: ParticleSys + std::clone::Clone,
{
    type T = P;

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
                check_period(self.period)?;
                p
            }
            None => self.period,
        };

        for ps in self.parts.iter_mut() {
            ps.setup(should_loop, Some(self.period))?;
        }

        self.looping = should_loop;
        self.active = true;
        self.initialized = true;
        self.reset_time();
        Ok(())
    }

    fn tear_down(&mut self) {
        for ps in self.parts.iter_mut() {
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

        for ps in self.parts.iter_mut() {
            ps.next_frame(current_time)?;
        }

        Ok(self.start_time.elapsed().as_secs_f32() <= self.period)
    }

    fn iter(&self) -> Option<Iter<'_, Self::T>> {
        Some(self.parts.iter())
    }

    fn iter_mut(&mut self) -> Option<IterMut<'_, Self::T>> {
        Some(self.parts.iter_mut())
    }

    fn with_period(mut self, p: f32) -> Self {
        self.period = p;
        self
    }
}

impl<P: ParticleSys + std::clone::Clone> Default for SyncGrp<P> {
    fn default() -> Self {
        SyncGrp::new(1.0, &[])
    }
}

/// Group of objects implementing ParticleSys that are
/// ran sequentially from the order they are defined within
/// the argument `parts`, each with the period defined
/// in the SeqGrp `period` value.
pub struct SeqGrp<P: ParticleSys> {
    pub period: f32,
    parts: Vec<P>,
    start_time: Instant,
    active: bool,
    looping: bool,
    initialized: bool,
}
