//! # LinearPL Example
//!
//! Crate for creating particle systems alongside
//! the macroquad crate.

use macroquad::prelude::*;

use linearpl::groups::{SeqGrp, SyncGrp};
use linearpl::linear_particles::LinearParticles;
use linearpl::particle::Particle;
use linearpl::particle_sys::ParticleSys;

const CAM_SPEED: f32 = 0.8;

#[macroquad::main("LinearPL Example")]
async fn main() -> Result<(), String> {
    let up = vec3(0., 1., 0.);

    let mut hrot: f32 = 1.57;
    let mut vrot: f32 = 0.2;

    let u_pos = vec3(0., 1., 0.);

    let mut u_front =
        vec3(hrot.cos() * vrot.cos(), vrot.sin(), hrot.sin() * vrot.cos()).normalize();
    let mut u_right = u_front.cross(up).normalize();
    let mut u_up = u_right.cross(u_front).normalize();

    let mut mouse_pressed = false;

    // **********************************
    // LIBRARY SETUP EXAMPLES START HERE!
    // **********************************
    
    let offset = 1.5;

    // some static particles
    let mut static_part1 = Particle::new((-1.5, 1.2 + offset, 4.), (0., 1., 1., 1.), 0.005, 1., false)?;
    let mut static_part2 = Particle::new((-1.5, 1. + offset, 4.), (0., 1., 0., 1.), 0.005, 1., false)?;
    let mut static_part3 = Particle::new((-1.5, 0.8 + offset, 4.), (1., 0., 0., 1.), 0.005, 1., false)?;

    // some linear particle systems
    let lin_part_h: LinearParticles =
        LinearParticles::new((-1., offset, 3.).into(), (1., offset, 3.).into())
            .with_decay(1.4)?
            .with_locations(&[0., 0., 1., 1.])?
            .with_colors(&[
                Color::new(0., 1., 1., 0.),
                Color::new(0., 1., 1., 0.),
                Color::new(0., 0.75, 1., 1.),
                Color::new(0., 0.25, 1., 1.),
                Color::new(0., 0., 1., 0.),
                Color::new(0., 0., 1., 0.),
            ])?;
    let lin_part_v: LinearParticles =
        LinearParticles::new((-1., offset, 3.).into(), (-1., 2. + offset, 3.).into())
            .with_decay(2.0)?
            .with_locations(&[1., 0., 1.])?
            .with_colors(&[PINK, PURPLE, RED, VIOLET])?;
    let mut linear_grp = SyncGrp::new(
        3.,
        &[
            lin_part_h.clone_with_start_end(vec3(1., offset, 5.), vec3(-1., offset, 5.))?,
            lin_part_h.clone_with_start_end(vec3(-1., 2. + offset, 3.), vec3(1., 2. + offset, 3.))?,
            lin_part_h.clone_with_start_end(vec3(1., 2. + offset, 5.), vec3(-1., 2. + offset, 5.))?,
            lin_part_h.clone_with_start_end(vec3(-1., offset, 5.), vec3(-1., offset, 3.))?,
            lin_part_h.clone_with_start_end(vec3(1., offset, 3.), vec3(1., offset, 5.))?,
            lin_part_h.clone_with_start_end(vec3(-1., 2. + offset, 5.), vec3(-1., 2. + offset, 3.))?,
            lin_part_h.clone_with_start_end(vec3(1., 2. + offset, 3.), vec3(1., 2. + offset, 5.))?,
            lin_part_h,
            lin_part_v.clone_with_start_end(vec3(1., offset, 3.), vec3(1., 2. + offset, 3.))?,
            lin_part_v.clone_with_start_end(vec3(1., offset, 5.), vec3(1., 2. + offset, 5.))?,
            lin_part_v.clone_with_start_end(vec3(-1., offset, 5.), vec3(-1., 2. + offset, 5.))?,
            lin_part_v,
        ],
    );

    let lil_lin_part = LinearParticles::new((-0.75, 0.25 + offset, 3.25).into(), (-0.75, 1.75 + offset, 3.25).into())
        .with_decay(0.2)?
        .with_densities(&[0.25])?
        .with_locations(&[1., 1., 0.5, 0., 0.])?
        .with_colors(&[SKYBLUE, GREEN])?;

    let mut linear_seq = SeqGrp::new(
        4.,
        &[
            lil_lin_part
                .clone_with_colors(&[GREEN, SKYBLUE])?
                .with_start_end((-0.75, 1.75 + offset, 3.25).into(), (0.75, 0.25 + offset, 4.75).into())?,
            lil_lin_part
                .clone_with_start_end((0.75, 0.25 + offset, 4.75).into(), (0.75, 1.75 + offset, 4.75).into())?,
            lil_lin_part
                .clone_with_colors(&[GREEN, SKYBLUE])?
                .with_start_end((0.75, 1.75 + offset, 4.75).into(), (0.75, 0.25 + offset, 3.25).into())?,
            lil_lin_part
                .clone_with_start_end((0.75, 0.25 + offset, 3.25).into(), (0.75, 1.75 + offset, 3.25).into())?,
            lil_lin_part
                .clone_with_colors(&[GREEN, SKYBLUE])?
                .with_start_end((0.75, 1.75 + offset, 3.25).into(), (-0.75, 0.25 + offset, 4.75).into())?,
            lil_lin_part
                .clone_with_start_end((-0.75, 0.25 + offset, 4.75).into(), (-0.75, 1.75 + offset, 4.75).into())?,
            lil_lin_part
                .clone_with_colors(&[GREEN, SKYBLUE])?
                .with_start_end((-0.75, 1.75 + offset, 4.75).into(), (-0.75, 0.25 + offset, 3.25).into())?,
            lil_lin_part,
        ],
    );

    let base_grid_line = LinearParticles::default()
        .with_decay(0.8)?
        .with_locations(&[0., 1., 0.])?
        .with_colors(&[RED, VIOLET, BLUE, SKYBLUE])?;

    let mut grid_lines_z: Vec<LinearParticles> = Vec::new();
    let mut grid_lines_x: Vec<LinearParticles> = Vec::new();
    let size = 15.;
    let res = 0.5;

    for i in (-(size as i32) + 1)..(size as i32) {
        grid_lines_z.push(base_grid_line.clone_with_start_end(
            (i as f32 * res, 0., size).into(),
            (i as f32 * res, 0., -size).into(),
        )?);
        grid_lines_x.push(base_grid_line.clone_with_start_end(
            (size, 0., i as f32 * res).into(),
            (-size, 0., i as f32 * res).into(),
        )?);
    }
    let mut grid = SyncGrp::new(
        8.,
        &[
            SyncGrp::new(8., &grid_lines_z),
            SyncGrp::new(8., &grid_lines_x),
        ],
    );

    if let Err(v) = linear_grp.start_loop() {
        eprintln!("linear_grp received error at startup: {:?}", v);
    };

    if let Err(v) = linear_seq.start_loop() {
        eprintln!("linear_seq received error at startup: {:?}", v);
    };

    if let Err(v) = grid.start_loop() {
        eprintln!("grid received error at startup: {:?}", v);
    };

    // **********************************
    // END HERE
    // **********************************

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            mouse_pressed = true;
        }
        if is_mouse_button_released(MouseButton::Left) {
            mouse_pressed = false;
        }
        if mouse_pressed {
            let d_mouse = mouse_delta_position();

            hrot += d_mouse.x * CAM_SPEED;
            vrot += d_mouse.y * -CAM_SPEED;
            u_front =
                vec3(hrot.cos() * vrot.cos(), vrot.sin(), hrot.sin() * vrot.cos()).normalize();
            u_right = u_front.cross(up).normalize();
            u_up = u_right.cross(u_front).normalize();
        }

        // non-event calls go here:
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: u_pos,
            up: u_up,
            target: u_pos + u_front,
            ..Default::default()
        });

        // **********************************
        // LIBRARY DRAW EXAMPLES START HERE!
        // **********************************

        // draw static particles manually, reset their clocks
        static_part1.draw();
        static_part2.draw();
        static_part3.draw();
        static_part1.reset();
        static_part2.reset();
        static_part3.reset();

        // draw the group of linear particle systems
        linear_grp.run()?;
        linear_seq.run()?;

        draw_line_3d(
            vec3(-size, 0., -size),
            vec3(size, 0., -size),
            WHITE,
        );
        draw_line_3d(
            vec3(size, 0., -size),
            vec3(size, 0., size),
            WHITE,
        );
        draw_line_3d(
            vec3(size, 0., size),
            vec3(-size, 0., size),
            WHITE,
        );
        draw_line_3d(
            vec3(-size, 0., size),
            vec3(-size, 0., -size),
            WHITE,
        );
        grid.run()?;

        // **********************************
        // END HERE
        // **********************************

        set_default_camera();
        draw_text(
            "\\(^O^)/ LinearPL",
            screen_width() - 200.0,
            screen_height() - 30.,
            20.,
            WHITE,
        );
        draw_text(
            "drag mouse to look around",
            screen_width() - 230.0,
            screen_height() - 10.,
            20.,
            WHITE,
        );

        next_frame().await;
    }
    Ok(())
}
