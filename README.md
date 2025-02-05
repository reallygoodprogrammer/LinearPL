# TDPL
 Three Dimensional Particle Library (for Macroquad in Rust)

---

My intention for this library is to easily generalize some particle system style 
graphics so they can easily be placed inside a Macroquad 3d environment without 
needing to create complex data structures to maintain the individual particles 
within the system, limiting the hassle of having to interact with each 
particle when designing. Some ideas for how I would structure this library are 
based around different types of particle system geometries. For example, a linea
particle system versus a planar particle system versus a spatial particle system.

For the Linear Particle System, the particles fall along a linear path defined 
by the caller. The caller has control over:
* Densities on line at specified interval or periods
* Overall length of graphical output (or continuously generate the output)
* Color of particles at a given period and location
* Randomization amount for different settings

Some other more global controls I would like to provide are how to interpolate 
between different settings when generating the system (ex: should the change from
red to green be linearly interpolated, sudden, exponential?). The basic idea for
the linear particle system would also apply to the planar system and the spatial
system except with some small changes to make up for the changes in geometry.

