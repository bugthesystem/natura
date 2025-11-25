use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_natura::{NaturaAnimationPlugin, NaturaSpringBundle, NaturaTarget};
use natura::{AngularFrequency, DampingRatio, DeltaTime};

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
    commands.spawn((
        Text2d::new("Natura: First sprite!"),
        TextFont {
            font: asset_server.load("fonts/PixelSmall.ttf"),
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        NaturaSpringBundle::new(
            DeltaTime(60.0),
            AngularFrequency(6.0),
            DampingRatio(0.7),
        ),
        NaturaTarget { x: 200.0, y: 150.0 },
    ));

    // Spawn second animated text - moves to bottom-left with different spring settings
    commands.spawn((
        Text2d::new("Second sprite - bouncy!"),
        TextFont {
            font: asset_server.load("fonts/PixelSmall.ttf"),
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 1.0, 0.5)),
        NaturaSpringBundle::new(
            DeltaTime(60.0),
            AngularFrequency(8.0),
            DampingRatio(0.3), // More bouncy
        ),
        NaturaTarget { x: -200.0, y: -100.0 },
    ));

    // Spawn third animated text - moves slowly to center-bottom
    commands.spawn((
        Text2d::new("Third - smooth"),
        TextFont {
            font: asset_server.load("fonts/PixelSmall.ttf"),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.5, 0.5)),
        NaturaSpringBundle::new(
            DeltaTime(60.0),
            AngularFrequency(3.0),
            DampingRatio(1.0), // Critically damped - no bounce
        ),
        NaturaTarget { x: 0.0, y: -200.0 },
    ));
}
