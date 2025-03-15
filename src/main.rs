//! # TDPL Example
//!
//! Crate for creating static_part1icle based effects along with
//! the macroquad rust crate.

use macroquad::prelude::*;
use tdpl::linear_particles::LinearParticles;
use tdpl::particle::Particle;

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

    let mut lin_part_one = LinearParticles::new((-1., 0., 3.).into(), (1., 0., 3.).into())
        .with_period(2.5)
        .with_decay(0.07)
        .with_locations(&[0., 0., 1., 1.])
        .with_colors(&[
            Color::new(0., 1., 1., 0.),
            Color::new(0., 1., 1., 0.),
            Color::new(0., 0.75, 1., 1.),
            Color::new(0., 0.25, 1., 1.),
            Color::new(0., 0., 1., 0.),
            Color::new(0., 0., 1., 0.),
        ]);

    let mut lin_part_two = lin_part_one
        .clone()
        .with_start(vec3(-1., 0., 5.))
        .with_end(vec3(1., 0., 5.));
    let mut lin_part_three = lin_part_one
        .clone()
        .with_start(vec3(-1., 2., 3.))
        .with_end(vec3(1., 2., 3.));
    let mut lin_part_four = lin_part_one
        .clone()
        .with_start(vec3(-1., 2., 5.))
        .with_end(vec3(1., 2., 5.));

    if let Err(v) = lin_part_one.start_loop() {
        eprintln!("lin_part_one received error at startup: {:?}", v);
    };
    if let Err(v) = lin_part_two.start_loop() {
        eprintln!("lin_part_one received error at startup: {:?}", v);
    };
    if let Err(v) = lin_part_three.start_loop() {
        eprintln!("lin_part_one received error at startup: {:?}", v);
    };
    if let Err(v) = lin_part_four.start_loop() {
        eprintln!("lin_part_one received error at startup: {:?}", v);
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

        // draw static particles, reset their clock
        static_part1.draw();
        static_part2.draw();
        static_part3.draw();
        static_part1.reset();
        static_part2.reset();
        static_part3.reset();

        lin_part_one.display()?;
        lin_part_two.display()?;
        lin_part_three.display()?;
        lin_part_four.display()?;

        // **********************************
        // END HERE
        // **********************************

        draw_cube_wires(vec3(-4., 1., 0.), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(4., 1., 0.), vec3(2., 2., 2.), RED);
        draw_cube_wires(vec3(0., 4., 0.), vec3(2., 2., 2.), YELLOW);
        draw_cube_wires(vec3(0., 1., -4.), vec3(2., 2., 2.), ORANGE);

        set_default_camera();
        draw_text(
            "Particles example!",
            screen_width() / 2.,
            screen_height() / 2.,
            30.,
            WHITE,
        );

        next_frame().await;
    }
    Ok(())
}
