use ggez::{
    event,
    graphics::{self, Canvas, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult, glam::Vec2,
};
use rand::Rng;
        
const SCREEN_SIZE: (f32, f32) = (800., 600.);
const PADDLE_SIZE: (f32, f32) = (25., 150.);
const HELLO_LABEL_POSITION: Vec2 = ggez::glam::Vec2::new(SCREEN_SIZE.1/2. * 2., 10.0);
const LEFT_SCORE_LABEL_POSITION: Vec2 = ggez::glam::Vec2::new(50., 10.0);
const RIGHT_SCORE_LABEL_POSITION: Vec2 = ggez::glam::Vec2::new(SCREEN_SIZE.0*2. - 50., 10.0);
        

struct Label {
    text: String,
}

impl Label {
    pub fn new() -> Self {
        Label {
            text: String::from("Hello Pong!"),
        }
    }
}

struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    direction: Direction,
    deviation: f32,
    speed: f32,
    autorun: bool,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32, autorun: bool, deviation: f32) -> Self {
        Rectangle {
            x,
            y,
            w,
            h,
            speed,
            autorun,
            deviation,
            direction: Direction::Right,
        }
    }

    pub fn update(&mut self) -> () {
        self.collide();

        if self.autorun {
            match self.direction {
                Direction::Down => self.move_down(),
                Direction::Up => self.move_up(),
                Direction::Right => self.move_right(),
                Direction::Left => self.move_left(),
            }
            // println!("ball: {} {}", self.y, self.deviation);
            self.y += self.deviation;
        }
    }

    fn collide(&mut self) -> () {
        match self.direction {
            Direction::Down => {
                if self.y >= SCREEN_SIZE.1 + self.h {
                    println!("{}", "changing direction");
                    self.direction = Direction::inverse(&self.direction);
                }
            }
            Direction::Up => {
                if self.y <= 0. {
                    println!("{}", "changing direction");
                    self.direction = Direction::inverse(&self.direction);
                }
            }
            Direction::Right => {
                if self.x >= SCREEN_SIZE.0 {
                    println!("{}", "changing direction");
                    self.direction = Direction::inverse(&self.direction);
                }
            }
            Direction::Left => {
                if self.x <= 0. {
                    println!("{}", "changing direction");
                    self.direction = Direction::inverse(&self.direction);
                }
            }
        }
        if self.y >= SCREEN_SIZE.1 - self.h {
            println!("changing deviation{}", self.deviation);
            self.deviation = -self.deviation;
            println!("changing deviation{}", self.deviation);
        }
    
        if self.y <= 0. {
            println!("changing deviation{}", self.deviation);
            self.deviation = -self.deviation;
            println!("changing deviation{}", self.deviation);
        }
    }

    pub fn move_right(&mut self) -> () {
        // println!("right {} towards {}", self.x, SCREEN_SIZE.0);
        if self.x < SCREEN_SIZE.0 {
            self.x += self.speed;
        }
    }
    pub fn move_left(&mut self) -> () {
        // println!("left {} from {}", self.x, SCREEN_SIZE.0);
        if self.x > 0. {
            self.x -= self.speed;
        }
    }
    pub fn move_up(&mut self) -> () {
        // println!("up {} from {}", self.y, SCREEN_SIZE.1);
        if self.y > 0. {
            self.y -= self.speed;
        }
    }
    pub fn move_down(&mut self) -> () {
        // println!("down {} towards {}", self.y, SCREEN_SIZE.1);
        if self.y < SCREEN_SIZE.1 - self.h {
            self.y += self.speed;
        }
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        let rect = graphics::Rect::new(self.x, self.y, self.w * 2., self.h* 2.);
        let rect_to_draw = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color::from([0.2, 0.3, 0.4, 1.0]),
        )?;
        canvas.draw(&rect_to_draw, ggez::graphics::DrawParam::new().dest(ggez::glam::Vec2::new(self.x, self.y)));
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

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
    ball: Rectangle,
    paddle_left: Rectangle,
    paddle_right: Rectangle,
    score_left: i32,
    score_right: i32,
}

impl GameState {
    pub fn new() -> Self {
        
        let mut rng = rand::thread_rng();
        let initial_ball_deviation =rng.gen_range(-3.0..3.0);
        
        GameState {
            label: Label::new(),
            ball: Rectangle::new(SCREEN_SIZE.0 / 2., SCREEN_SIZE.1 / 2., 20., 20., 6., true, initial_ball_deviation),
            paddle_left: Rectangle::new(
                0. + PADDLE_SIZE.0 / 2.,
                SCREEN_SIZE.1 / 2.,
                PADDLE_SIZE.0,
                PADDLE_SIZE.1,
                7.,
                false,
                0.,
            ),
            paddle_right: Rectangle::new(
                SCREEN_SIZE.0 - PADDLE_SIZE.0 * 1.5,
                SCREEN_SIZE.1 / 2.,
                PADDLE_SIZE.0,
                PADDLE_SIZE.1,
                7.,
                false,
                0.,
            ),
            score_left: 0,
            score_right: 0,
        }
    }

    pub fn score_left_up(&mut self) -> () {
        self.score_left += 1;
    }

    pub fn score_right_up(&mut self) -> () {
        self.score_right += 1;
    }

    pub fn restart_ball(&mut self) -> () {
        let mut rng = rand::thread_rng();
        let initial_ball_deviation =rng.gen_range(-3.0..3.0);

        self.ball.x = SCREEN_SIZE.0/2.;
        self.ball.y = SCREEN_SIZE.1/2.;
        self.ball.direction = self.ball.direction.inverse();
        self.ball.deviation = initial_ball_deviation;
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let pressed_keys = ctx.keyboard.pressed_keys();
        for key in pressed_keys {
            match key {
                KeyCode::W => self.paddle_left.move_up(),
                KeyCode::S => self.paddle_left.move_down(),
                KeyCode::Up => self.paddle_right.move_up(),
                KeyCode::Down => self.paddle_right.move_down(),
                _ => (),
            }
        }

        if self.ball.x >= SCREEN_SIZE.0 {
            self.score_left_up();
            self.restart_ball();
        }
        if self.ball.x <= 0. {
            self.score_right_up();
            self.restart_ball();
        }


        let ball_rect = graphics::Rect::new(self.ball.x, self.ball.y, self.ball.w, self.ball.h);
        let paddle_left_rect = graphics::Rect::new(
            self.paddle_left.x,
            self.paddle_left.y,
            self.paddle_left.w,
            self.paddle_left.h,
        );
        let paddle_right_rect = graphics::Rect::new(
            self.paddle_right.x,
            self.paddle_right.y,
            self.paddle_right.w,
            self.paddle_right.h,
        );

        if ball_rect.overlaps(&paddle_left_rect) || ball_rect.overlaps(&paddle_right_rect) {
            println!("ball ({}, {}), left ({}, {}), right({},{})", self.ball.x, self.ball.y, self.paddle_left.x, self.paddle_left.y, self.paddle_right.x, self.paddle_right.y);
            self.ball.direction = self.ball.direction.inverse();
        }

        self.ball.update();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::RED);

        let mut hello_label = graphics::Text::new(&self.label.text);
        canvas.draw(hello_label.set_scale(48.), HELLO_LABEL_POSITION);
        
        let mut left_score_label = graphics::Text::new(format!("{}", &self.score_left));
        canvas.draw(left_score_label.set_scale(48.), LEFT_SCORE_LABEL_POSITION);
        
        let mut right_score_label = graphics::Text::new(format!("{}", &self.score_right));
        canvas.draw(right_score_label.set_scale(48.), RIGHT_SCORE_LABEL_POSITION);

        self.ball.draw(ctx, &mut canvas)?;
        self.paddle_left.draw(ctx, &mut canvas)?;
        self.paddle_right.draw(ctx, &mut canvas)?;
        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        println!("key: {:?}", input.keycode);
        // if let Some(key) = input.keycode.and_then(Direction::from_keycode) {
        //     self.rectangle.direction = key;
        // };
        Ok(())
    }
}

fn main() -> GameResult {
    // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let (ctx, events_loop) = ggez::ContextBuilder::new("hello", "Maciej Mikulski")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("My game!"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0 * 2., SCREEN_SIZE.1 * 2.),
        )
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()?;

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = GameState::new();
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}
