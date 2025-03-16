//! # Particle Sys
//!
//! Contains the trait definition for ParticleSys, a system
//! of particles.

use std::slice::{Iter, IterMut};

/// Defines how to interact with a system of particles within
/// the `tdpl` library.
pub trait ParticleSys {
    type T;

    /// Check if ParticleSys is active.
    /// Returns `true` if ParticleSys is in active state. Else `false`.
    fn is_active(&self) -> bool;

    /// Check if ParticleSys is looping.
    /// Return `true` if ParticleSys is in active looping state. Else `false`.
    fn is_looping(&self) -> bool;

    /// Return `true` if LinearParticles is initialized and ready to use.
    fn is_initialized(&mut self) -> bool;

    /// Reset the elapsed time counter for the ParticleSys.
    ///
    /// The implementor can implement this how they desire for specific
    /// timing effects, although it is recommended to have the
    /// counter measuring time for the ParticleSys to be set back
    /// at `0.0` so that Particle Groups function as intended.
    fn reset_time(&mut self);

    /// Return the `Some(elapsed)` where elapsed is total elapsed seconds
    /// counted by the ParticleSys as `f32`, or None if that's desirable.
    fn elapsed_time(&mut self) -> Option<f32>;

    /// Set up the ParticleSys such that it is ready to be
    /// displayed.
    ///
    /// The implementor is in charge of making sure that this
    /// operation will result in `is_active()` and `is_initialized()`
    /// calls returning true.
    fn setup(&mut self, should_loop: bool) -> Result<(), String>;

    /// Tear down the ParticleSys such that `is_active()` and `is_initialized()`
    /// return false and any other resetting of variables necessary for
    /// the ParticleSys to be able to call `setup()`.
    ///
    /// This is equivalent to calling the `stop()` method for this trait.
    fn tear_down(&mut self);

    /// Display the next frame of the ParticleSys Particles with
    /// elapsed time `time` if `Some(time)`, else the ParticleSys own
    /// counting mechanism.
    ///
    /// # Returns
    ///
    /// - `Ok(true)` if LinearParticle is still 'generating' in next frame,
    /// - `Ok(false)` otherwise
    ///
    /// Note: looping mechanisms are handled by the traits `display`
    /// implementation and should not be implemented in this method.
    fn next_frame(&mut self, time: Option<f32>) -> Result<bool, String>;

    /// Return an Iterator over the Particle Pieces managed by the
    /// ParticleSys.
    fn iter(&self) -> Iter<'_, Self::T>;

    /// Return a Mutable Iterator over the Particle Pieces managed by
    /// the ParticleSys.
    fn iter_mut(&mut self) -> IterMut<'_, Self::T>;

    /// Set up ParticleSys into its looping active state.
    fn start_loop(&mut self) -> Result<(), String> {
        self.setup(true)
    }

    /// Set up ParticleSys into its active state.
    fn start(&mut self) -> Result<(), String> {
        self.setup(false)
    }

    /// Tear down and deactivate ParticleSys object.
    fn stop(&mut self) {
        self.tear_down();
    }

    /// Display the next frame available from the LinearParticle.
    ///
    ///
    /// # Returns:
    ///
    /// - `Ok(true)` if LinearParticle is still 'active' in next frame,
    /// - `Ok(false)` otherwise
    fn run(&mut self) -> Result<bool, String> {
        if !(self.is_active() && self.is_initialized()) {
            return Err("object has not been setup for display call".into());
        }
        let elapsed = self.elapsed_time();
        if !self.next_frame(elapsed)? {
            if self.is_looping() {
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
