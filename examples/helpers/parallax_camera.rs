use bevy::{core::Time, input::Input, math::Vec3, prelude::*, render::camera::Camera};

#[derive(Component,Debug)]
pub struct ParallaxSpeed {
    pub x: f32,
    pub y: f32,
}

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut set: ParamSet< (
        Query</*&mut Transform, */&mut OrthographicProjection, With<Camera>>,
        Query<(&ParallaxSpeed, &mut Transform)>,
    )>,
) {
    let mut direction = Vec3::ZERO;
    for mut ortho in set.p0().iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }
    }

    for (parallax_speed, mut transform) in set.p1().iter_mut() {
        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * Vec3::new(parallax_speed.x, parallax_speed.y, 0.0) * 100.;
        transform.translation.z = z;
        println!("Parallax speed: {:?}", parallax_speed);
    }
}
