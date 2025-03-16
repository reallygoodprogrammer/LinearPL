//! # TDPL Example
//!
//! Crate for creating static_part1icle based effects along with
//! the macroquad rust crate.

use macroquad::prelude::*;

use tdpl::linear_particles::{LinearGrp, LinearParticles};
use tdpl::particle::Particle;
use tdpl::particle_sys::ParticleSys;

const CAM_SPEED: f32 = 0.8;

#[macroquad::main("tdpl example")]
async fn main() -> Result<(), String> {
    let up = vec3(0., 1., 0.);

    let mut hrot: f32 = 1.18;
    let mut vrot: f32 = 0.;

    let u_pos = vec3(0., 1., 0.);

    let mut u_front =
        vec3(hrot.cos() * vrot.cos(), vrot.sin(), hrot.sin() * vrot.cos()).normalize();

    // right and up from user perspective
    let mut u_right = u_front.cross(up).normalize();
    let mut u_up = u_right.cross(u_front).normalize();

    // mouse is pressed flag
    let mut mouse_pressed = false;

    // **********************************
    // LIBRARY SETUP EXAMPLES START HERE!
    // **********************************

    // some static particles
    let mut static_part1 = Particle::new((-0.2, 1., 4.), (0., 1., 1., 1.), 0.01, 1., false);
    let mut static_part2 = Particle::new((0., 1., 4.), (0., 1., 0., 1.), 0.01, 1., false);
    let mut static_part3 = Particle::new((0.2, 1., 4.), (1., 0., 0., 1.), 0.01, 1., false);

    // some linear particle systems
    let lin_part_h = LinearParticles::new((-1., 0., 3.).into(), (1., 0., 3.).into())
        .with_decay(1.4)
        .with_locations(&[0., 0., 1., 1.])
        .with_colors(&[
            Color::new(0., 1., 1., 0.),
            Color::new(0., 1., 1., 0.),
            Color::new(0., 0.75, 1., 1.),
            Color::new(0., 0.25, 1., 1.),
            Color::new(0., 0., 1., 0.),
            Color::new(0., 0., 1., 0.),
        ]);
    let lin_part_v = LinearParticles::new((-1., 0., 3.).into(), (-1., 2., 3.).into())
        .with_decay(2.0)
        .with_locations(&[1., 0., 1.])
        .with_colors(&[PINK, PURPLE, RED, VIOLET]);
    let mut linear_grp = LinearGrp::new(
        3.,
        &[
            lin_part_h.clone(),
            lin_part_h
                .clone()
                .with_start_end(vec3(1., 0., 5.), vec3(-1., 0., 5.)),
            lin_part_h
                .clone()
                .with_start_end(vec3(-1., 2., 3.), vec3(1., 2., 3.)),
            lin_part_h
                .clone()
                .with_start_end(vec3(1., 2., 5.), vec3(-1., 2., 5.)),
            lin_part_h
                .clone()
                .with_start_end(vec3(-1., 0., 5.), vec3(-1., 0., 3.)),
            lin_part_h
                .clone()
                .with_start_end(vec3(1., 0., 3.), vec3(1., 0., 5.)),
            lin_part_h
                .clone()
                .with_start_end(vec3(-1., 2., 5.), vec3(-1., 2., 3.)),
            lin_part_h
                .clone()
                .with_start_end(vec3(1., 2., 3.), vec3(1., 2., 5.)),
            lin_part_v.clone(),
            lin_part_v
                .clone()
                .with_start_end(vec3(1., 0., 3.), vec3(1., 2., 3.)),
            lin_part_v
                .clone()
                .with_start_end(vec3(1., 0., 5.), vec3(1., 2., 5.)),
            lin_part_v
                .clone()
                .with_start_end(vec3(-1., 0., 5.), vec3(-1., 2., 5.)),
        ],
    );

    if let Err(v) = linear_grp.start_loop() {
        eprintln!("linear_grp received error at startup: {:?}", v);
    };

    // **********************************
    // END HERE
    // **********************************

    let fps_reset = 100;
    let mut fps_counter = 0;
    let mut fps_val = 0;
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

        // **********************************
        // END HERE
        // **********************************

        draw_cube_wires(vec3(-4., 1., 0.), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(4., 1., 0.), vec3(2., 2., 2.), RED);
        draw_cube_wires(vec3(0., 4., 0.), vec3(2., 2., 2.), YELLOW);
        draw_cube_wires(vec3(0., 1., -4.), vec3(2., 2., 2.), ORANGE);

        fps_counter += 1;
        if fps_counter % fps_reset == 0 {
            fps_counter = 0;
            fps_val = get_fps();
        }

        set_default_camera();
        draw_text(
            format!("TDPL example! fps:{}", fps_val).as_str(),
            screen_width() - 275.0,
            screen_height() - 30.,
            30.,
            WHITE,
        );

        next_frame().await;
    }
    Ok(())
}
