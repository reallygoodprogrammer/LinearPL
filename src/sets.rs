/// # Particle Sets
///
/// Contains some implementations of synced particle system sets
/// that use the other particle systems implemented in the library.
/// These sets act as a way to have a group of particle systems
/// that share a start time, and period for graphics generation.

/// Group of LinearParticles objects with a synced period and 
/// start time.
pub struct LinearGrp {
    period : f32,
    linear_particles : Vec<LinearParticles>,
    start_time: Instant,
}

impl LinearGrp {
    /// Create a new group of LinearParticles objects.
    pub fn new(period: f32, linparts: &[LinearParticles]) -> Self {
        LinearGrp {
            period,
            linear_particles: linparts.into(),
        }
    }

    /// Check if LinearGrp is active.
    /// Returns `true` if LinearParticles is in active state. Else `false`.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Check if LinearGrp is looping.
    /// Return `true` if LinearParticles is in active looping state. Else `false`.
    pub fn is_looping(&self) -> bool {
        self.active && self.looping
    }

    /// Set up LinearGrp into its looping active state.
    pub fn start_loop(&mut self) -> Result<(), String> {
        self.setup(true)
    }

    /// Set up LinearGrp into its active state.
    pub fn start(&mut self) -> Result<(), String> {
        self.setup(false)
    }

    /// Tear down and deactivate LinearGrp object.
    pub fn stop(&mut self) {
        self.tear_down();
    }

    // reset the elapsed time counter
    fn reset_time(&mut self) {
        self.start_time = Instant::now();
    }
}
