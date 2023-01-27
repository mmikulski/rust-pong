use ggez::{
    event,
    graphics::{self, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

struct Label {
    text: String,
}

impl Label {
    pub fn new() -> Self {
        Label {
            text: String::from("Hello"),
        }
    }
}

struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            x: 123.0,
            y: 123.0,
            w: 10.0,
            h: 10.0,
        }
    }

    // pub fn draw(&self, )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // /// We create a helper function that will allow us to easily get the inverse
    // /// of a `Direction` which we can use later to check if the player should be
    // /// able to move the snake in a certain direction.
    // pub fn inverse(&self) -> Self {
    //     match *self {
    //         Direction::Up => Direction::Down,
    //         Direction::Down => Direction::Up,
    //         Direction::Left => Direction::Right,
    //         Direction::Right => Direction::Left,
    //     }
    // }

    /// We also create a helper function that will let us convert between a
    /// `ggez` `Keycode` and the `Direction` that it represents. Of course,
    /// not every keycode represents a direction, so we return `None` if this
    /// is the case.
    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

struct GameState {
    label: Label,
    rectangle: Rectangle,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            label: Label::new(),
            rectangle: Rectangle::new(),
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // self.label.text = String::from("xyz");
        // self.rectangle.w += 1.0;


        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.rectangle.y -= 1.0;
        }        
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.rectangle.y += 1.0;
        }        
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.rectangle.x -= 1.0;
        }        
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.rectangle.x += 1.0;
        }        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // print!("{}", self.label.text);

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::RED);


        // Text is drawn from the top-left corner.
        let dest_point = ggez::glam::Vec2::new(10.0, 10.0);
        canvas.draw(
            graphics::Text::new(&self.label.text)
                .set_scale(48.),
            dest_point,
        );

        let rect = graphics::Rect::new(
            self.rectangle.x,
            self.rectangle.y,
            self.rectangle.w,
            self.rectangle.h,
        );
        let rect_to_draw = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::Color::from([0.2, 0.3, 0.4, 1.0]),
        )?;
        canvas.draw(&rect_to_draw, ggez::glam::Vec2::new(self.rectangle.x, self.rectangle.y));
        canvas.finish(ctx)?;

        Ok(())
    }

    // fn key_down_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     input: ggez::input::keyboard::KeyInput,
    //     _repeated: bool,
    // ) -> Result<(), ggez::GameError> {
    //     if let Some(key) = input.keycode.and_then(Direction::from_keycode) {
    //         // print!("key pressed: {:?}", key);
    //         match key {
    //             Direction::Up => self.rectangle.y -= 1.0,
    //             Direction::Down => self.rectangle.y += 1.0,
    //             Direction::Right => self.rectangle.x += 1.0,
    //             Direction::Left => self.rectangle.x -= 1.0
    //         }
    //     };
    //     Ok(())
    // }
}

fn main() -> GameResult {
    // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let (ctx, events_loop) = ggez::ContextBuilder::new("hello", "Maciej Mikulski")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("My game!"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()?;

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = GameState::new();
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}
