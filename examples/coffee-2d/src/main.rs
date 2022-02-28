use coffee::graphics::{Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Timer};

use natura::*;

fn main() -> coffee::Result<()> {
    NaturaExample::run(WindowSettings {
        title: String::from("Λ.R.Ξ.N.Λ 2D — Natura: example with Coffee"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

// where we want to animate it.
const TARGET_X: f64 = 300.0;
const TARGET_Y: f64 = 500.0;

// A thing we want to animate.
#[derive(Default)]
struct RectSprite {
    x: f64,
    x_velocity: f64,
    y: f64,
    y_velocity: f64,
}

struct NaturaExample {
    sprite: RectSprite,
    spring: Spring,
}

impl Game for NaturaExample {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<NaturaExample> {
        // Initialize a spring with frame-rate, angular frequency, and damping values.
        Task::succeed(|| NaturaExample {
            sprite: RectSprite::default(),
            spring: Spring::new(DeltaTime(natura::fps(60)), AngularFrequency(6.0), DampingRatio(0.5)),
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);
        let mut mesh = Mesh::new();
        let (sprite_x, sprite_x_velocity) =
            self.spring
                .update(self.sprite.x, self.sprite.x_velocity, TARGET_X);
        self.sprite.x = sprite_x;
        self.sprite.x_velocity = sprite_x_velocity;

        let (sprite_y, sprite_y_velocity) =
            self.spring
                .update(self.sprite.y, self.sprite.y_velocity, TARGET_Y);
        self.sprite.y = sprite_y;
        self.sprite.y_velocity = sprite_y_velocity;

        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.sprite.x as f32,
                y: self.sprite.y as f32,
                width: 200.0,
                height: 100.0,
            }),
            Color::WHITE,
        );
        mesh.draw(&mut frame.as_target());
    }
}
