/// # Particle Sets
///
/// Contains some implementations of synced particle system sets
/// that use the other particle systems implemented in the library.
/// These sets act as a way to have a group of particle systems
/// that share a start time, and period for graphics generation.


/// Group of synced LinearParticles objects with a synced period and 
/// start time.
pub struct LinearSet {
    linear_particles : Vec<LinearParticles>,
    period : f32,
    start_time: Instant,
}
