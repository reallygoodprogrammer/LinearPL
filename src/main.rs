use macroquad::prelude::*;

const CAM_SPEED: f32 = 0.8;

#[macroquad::main("tdpl example")]
async fn main() {
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

        draw_cube_wires(vec3(-4., 1., 0.), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(0., 1., 4.), vec3(2., 2., 2.), BLUE);
        draw_cube_wires(vec3(4., 1., 0.), vec3(2., 2., 2.), RED);
        draw_cube_wires(vec3(0., 4., 0.), vec3(2., 2., 2.), YELLOW);
        draw_cube_wires(vec3(0., 1., -4.), vec3(2., 2., 2.), ORANGE);

        set_default_camera();
        draw_text(
            "Hello, world!",
            screen_width() / 2.,
            screen_height() / 2.,
            30.,
            WHITE,
        );

        next_frame().await;
    }
}
