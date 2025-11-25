use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_natura::{NaturaAnimationPlugin, NaturaSpringBundle, NaturaTarget};
use natura::{AngularFrequency, DampingRatio};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NaturaAnimationPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2d::default());

    // Spawn first animated text - moves to top-right
    // Uses default damping for smooth motion
    commands.spawn((
        Text2d::new("Natura: First sprite!"),
        TextFont {
            font: asset_server.load("fonts/PixelSmall.ttf"),
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        NaturaSpringBundle::new(
            AngularFrequency(6.0),
            DampingRatio(0.7),
        ),
        NaturaTarget::new_2d(200.0, 150.0),
    ));

    // Spawn second animated text - moves to bottom-left with bouncy spring
    // Low damping ratio (0.3) creates oscillating/bouncy motion
    commands.spawn((
        Text2d::new("Second sprite - bouncy!"),
        TextFont {
            font: asset_server.load("fonts/PixelSmall.ttf"),
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 1.0, 0.5)),
        NaturaSpringBundle::new(
            AngularFrequency(8.0),
            DampingRatio(0.3), // Under-damped: bouncy oscillation
        ),
        NaturaTarget::new_2d(-200.0, -100.0),
    ));

    // Spawn third animated text - moves slowly to center-bottom
    // Critically damped (1.0) reaches target as fast as possible without oscillation
    commands.spawn((
        Text2d::new("Third - smooth"),
        TextFont {
            font: asset_server.load("fonts/PixelSmall.ttf"),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.5, 0.5)),
        NaturaSpringBundle::new(
            AngularFrequency(3.0),
            DampingRatio(1.0), // Critically damped: no bounce, fastest to target
        ),
        NaturaTarget::new_2d(0.0, -200.0),
    ));
}
