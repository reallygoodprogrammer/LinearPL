//! # LinearPL
//!
//! Fairly straight-forward *Linear Particle Library* for use with
//! [macroquad](https://github.com/not-fl3/macroquad) in rust.
//!
//! ---
//!
//! # Usage
//!
//! Here is how LinearPL is currently intended to be used:
//!
//! ### ParticleSys
//!
//! The core api for this library resides in the `linearpl::particle_sys::ParticleSys`
//! trait's start and stop methods:
//!
//! * `start()` and `start_loop()` setup and prepare the particle system to be drawn
//! * `run()` displays particles with respect to the amount of elapsed time from "starting"
//! * `stop()` stops the particle system before termination in `run()` or while looping
//!
//! Along with these methods, all implementations of `ParticleSys` in the library implement
//! particle systems that span a set `period` held by the object, which is the number of seconds
//! the particle system should run.
//!
//! ### LinearParticles
//!
//! For the linear particle system `linearpl::linear_particles::LinearParticles`, the particles
//! fall along a linear path defined by the `start_location` and `end_location` Vec3 members of
//! the object. The user then has control over some other settings which are linearly interpolated
//! over throughout the entire `period` of the objects particle generation:
//!
//! * `densities` : chance that a particle will be drawn in the given frame (0 to 1)
//! * `locations` : location to generate particle on line from 0 (`start_location`) to 1 (`end_location`)
//! * `colors` : color of particle generated in the given frame (using `macroquad::color::Color`)
//!
//! Other than that, there is a `decay` control which sets the amount of time it a particle
//! is drawn for (i.e. defines the `period` of each individual particle).
//!
//! ### SyncGrp and SeqGrp
//!
//! These two objects are used to created synchronized groups of objects implementing `ParticleSys`
//! using a single clock, making it easier for the user to create more complex and interesting
//! graphics from the particle system implementation in the library. These objects hold any
//! type of `ParticleSys` implementation, including other SyncGrp and SeqGrp objects.
//!
//! ---
//!
//! ### Future
//!
//! My original intention for this project was to create more types of particle systems within the
//! library which I haven't done **yet**. With the time I had available, I instead decided to
//! implement a linear particles system along with objects for grouping and syncing particle systems
//! I create in the future. I have enjoyed working on this project and may come back and hopefully
//! make it better.

mod util;

pub mod groups;
pub mod linear_particles;
pub mod particle;
pub mod particle_sys;
