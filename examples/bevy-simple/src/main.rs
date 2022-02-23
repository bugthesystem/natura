use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use natura::*;

// A thing we want to animate.
#[derive(Default, Component)]
struct Sprite {
    x: f64,
    x_velocity: f64,
    y: f64,
    y_velocity: f64,
}

#[derive(Default)]
struct NaturaBundle {
    sprite: Sprite,
    spring: Spring,
}

// Where we want to animate it.
const TARGET_X: f64 = 40.0;
const TARGET_Y: f64 = 200.0;

fn main() {
    let sprite = Sprite::default();

    // Initialize a spring with frame-rate, angular frequency, and damping values.
    let spring = Spring::new(natura::fps(60), 6.0, 0.7);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(NaturaBundle { sprite, spring })
        .add_startup_system(template_setup)
        .add_system(template_animation)
        .run();
}

fn template_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Shall we play a game?",
            TextStyle {
                font: asset_server.load("fonts/PixelSmall.ttf"),
                font_size: 58.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}

fn template_animation(
    _: Res<Time>,
    mut natur: ResMut<NaturaBundle>,
    mut query: Query<&mut Transform, With<Text>>,
) {
    let n = natur.as_mut();

    let (sprite_x, sprite_x_velocity) = n.spring.update(n.sprite.x, n.sprite.x_velocity, TARGET_X);

    n.sprite.x = sprite_x;
    n.sprite.x_velocity = sprite_x_velocity;

    let (sprite_y, sprite_y_velocity) = n.spring.update(n.sprite.y, n.sprite.y_velocity, TARGET_Y);
    n.sprite.y = sprite_y;
    n.sprite.y_velocity = sprite_y_velocity;

    for mut transform in query.iter_mut() {
        transform.translation.x = n.sprite.x as f32;
        transform.translation.y = n.sprite.y as f32;
    }
}
