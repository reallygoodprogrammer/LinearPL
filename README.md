# linearpl

<a href="https://github.com/reallygoodprogrammer/linearpl/blob/main/LICENSE.txt"><img alt="Crates.io License" src="https://img.shields.io/crates/l/linearpl"></a>
<a href="https://crates.io/crates/linearpl"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/linearpl"></a>

Linear particle library for [macroquad](https://github.com/not-fl3/macroquad).

---

# Usage

An example of how to use this library is given in [main.rs](src/main.rs), I'd recommend
looking at that after for a more detailed example that puts everything together.

## Examples

A linear particle instance starting at (0, 0, 0) and ending at (1, 1, 1):

```rust
use macroquad::prelude::*;
use linearpl::linear_particles::LinearParticles;

...

let start = Vec3::new(0.0, 0.0, 0.0);
let end = Vec3::new(1.0, 1.0, 1.0);
let mut linear_instance = LinearParticles::new(start, end)
    .with_decay(1.4)?
    .with_locations(&[0.0, 1.0])?
    .with_colors(&[RED, BLUE])?;

if let Err(v) == linear_instance.start_loop() {
    eprintln!("received error: {:?}", v);
}

loop {
    ...
    linear_instance.run()?;
    ...
}
```

---

# Parts

### ParticleSys

The core api for this library resides in the `linearpl::particle_sys::ParticleSys`
trait's start and stop methods:

* `start()` and `start_loop()` setup and prepare the particle system to be drawn
* `run()` displays particles with respect to the amount of elapsed time from "starting"
* `stop()` stops the particle system before termination in `run()` or while looping

Along with these methods, all implementations of `ParticleSys` in the library implement
particle systems that span a set `period` held by the object, which is the number of seconds
the particle system should run.

### LinearParticles

For the linear particle system `linearpl::linear_particles::LinearParticles`, the particles 
fall along a linear path defined by the `start_location` and `end_location` Vec3 members of
the object. The user then has control over some other settings which are linearly interpolated
over throughout the entire `period` of the objects particle generation:

* `densities` : chance that a particle will be drawn in the given frame (0 to 1)
* `locations` : location to generate particle on line from 0 (`start_location`) to 1 (`end_location`)
* `colors` : color of particle generated in the given frame (using `macroquad::color::Color`)

Other than that, there is a `decay` control which sets the amount of time it a particle
is drawn for (i.e. defines the `period` of each individual particle).

### SyncGrp and SeqGrp

These two objects are used to created synchronized groups of objects implementing `ParticleSys`
using a single clock, making it easier for the user to create more complex and interesting
graphics from the particle system implementation in the library. These objects hold any
type of `ParticleSys` implementation, including other SyncGrp and SeqGrp objects.


# Licensing

MIT License included [here](LICENSE.txt).
