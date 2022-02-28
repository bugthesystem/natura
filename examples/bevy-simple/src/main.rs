use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_natura::{NaturaAnimationBundle, NaturaAnimationPlugin};
use natura::{AngularFrequency, DampingRatio, DeltaTime};

// Where we want to animate it.
const TARGET_X: f64 = 40.0;
const TARGET_Y: f64 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NaturaAnimationPlugin::new(
            DeltaTime(60.0),
            AngularFrequency(6.0),
            DampingRatio(0.7),
        ))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(template_setup)
        .add_system(template_animation)
        .run();
}

fn template_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Natura: Shall we play a game Bevy?",
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
    mut natura_bundle: ResMut<NaturaAnimationBundle>,
    mut query: Query<&mut Transform, With<Text>>,
) {
    let _natura = natura_bundle.as_mut();

    let (sprite_x, sprite_x_velocity) = _natura.spring.update(_natura.sprite.x, _natura.sprite.x_velocity, TARGET_X);

    _natura.sprite.x = sprite_x;
    _natura.sprite.x_velocity = sprite_x_velocity;

    let (sprite_y, sprite_y_velocity) = _natura.spring.update(_natura.sprite.y, _natura.sprite.y_velocity, TARGET_Y);
    _natura.sprite.y = sprite_y;
    _natura.sprite.y_velocity = sprite_y_velocity;

    for mut transform in query.iter_mut() {
        transform.translation.x = _natura.sprite.x as f32;
        transform.translation.y = _natura.sprite.y as f32;
    }
}
