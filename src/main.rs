#![feature(exclusive_range_pattern)]
#![allow(unused)]
#![feature(const_trait_impl)]

extern crate termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::{thread, time};
use extra::rand::Randomizer;
use std::io::Read;
use std::thread::current;
use termion::async_stdin;
use std::time::Duration;

mod graphics {
    pub const PADDLE_MOVE_VALUE: i32 = 2;
    pub const HEIGHT_HIGH: i32 = 2;
    pub const HEIGHT_LOW: i32 = 32;
    pub const LENGTH_LEFT: i32 = 10;
    pub const LENGTH_RIGHT: i32 = 93;
    pub const BALL: &'static str = "o";
    pub const PADDLE: &'static str = "[]";
    pub const WALL_HORIZONTAL: &'static str = "═";
    pub const WALL_VERTICAL: &'static str = "║";
    pub const WALL_CORNER_LEFT_HIGH: &'static str = "╔";
    pub const WALL_CORNER_RIGHT_HIGH: &'static str = "╗";
    pub const WALL_CORNER_LEFT_LOW: &'static str = "╚";
    pub const WALL_CORNER_RIGHT_LOW: &'static str = "╝";
    pub const START_SCREEN: &'static [&'static str] = &[
        "╔══════════════════════════════╗",
        "║───────PONG GAME - Mamut──────║",
        "║──────────────────────────────║",
        "║ d ┆ up {{left paddle}}       ║",
        "║ s ┆ down {{left paddle}}     ║",
        "║ k ┆ up {{right paddle}}      ║",
        "║ j ┆ down {{right paddle}}    ║",
        "║ g ┆ start game               ║",
        "╚═══╧══════════════════════════╝"
    ];

}
struct Ball {
    x: i32,
    y: i32,

    x_speed: i32,
    y_speed: i32
}

struct Paddle {
    x: i32,
    y: i32,
}

static mut score_1: i32 = 0;
static mut score_2: i32 = 0;
static mut game_started: bool = false;

fn draw(x: u16, y: u16, content: String) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}{}",
           termion::cursor::Goto(x, y),
           content,
           termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();
}
fn start_screen() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut array_counter = 20;

    stdout.flush().unwrap();

    write!(stdout,
           "{}",
           termion::clear::All)
        .unwrap();
    stdout.flush().unwrap();

    for x in graphics::START_SCREEN {
        write!(stdout, "{}{}{}",
               termion::cursor::Goto(35, array_counter),
               x,
               termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();
        array_counter += 1

    }
}
fn display_walls() {
    //horizontal
    for i in graphics::LENGTH_LEFT..graphics::LENGTH_RIGHT {
        draw(i as u16, graphics::HEIGHT_LOW as u16, graphics::WALL_HORIZONTAL.to_string());
        draw(i as u16, graphics::HEIGHT_HIGH as u16, graphics::WALL_HORIZONTAL.to_string());
    }

    for u in graphics::HEIGHT_HIGH..graphics::HEIGHT_LOW {

        draw(graphics::LENGTH_LEFT as u16, u as u16, graphics::WALL_VERTICAL.to_string());
        draw(graphics::LENGTH_RIGHT as u16, u as u16, graphics::WALL_VERTICAL.to_string())
    }

    draw(graphics::LENGTH_LEFT as u16, graphics::HEIGHT_HIGH as u16, graphics::WALL_CORNER_LEFT_HIGH.to_string());
    draw(graphics::LENGTH_LEFT as u16, graphics::HEIGHT_LOW as u16, graphics::WALL_CORNER_LEFT_LOW.to_string());

    draw(graphics::LENGTH_RIGHT as u16, graphics::HEIGHT_HIGH as u16, graphics::WALL_CORNER_RIGHT_HIGH.to_string());
    draw(graphics::LENGTH_RIGHT as u16, graphics::HEIGHT_LOW as u16, graphics::WALL_CORNER_RIGHT_LOW.to_string())
}
fn default_setup() {
    start_screen();

}
fn display_paddles(self1: &mut Paddle, self2: &mut Paddle) {

    for g in 0..3 {
        draw(self1.x as u16, self1.y as u16 - g, graphics::PADDLE.to_string());
        draw(self1.x as u16, self1.y as u16 + g, graphics::PADDLE.to_string());
        draw(self2.x as u16, self2.y as u16 - g, graphics::PADDLE.to_string());
        draw(self2.x as u16, self2.y as u16 + g, graphics::PADDLE.to_string());
    }

}
fn display_ball(self_b: &mut Ball) {
    draw(self_b.x as u16, self_b.y as u16, graphics::BALL.to_string())
}
fn game_setup(self_b: &mut Ball, self1: &mut Paddle, self2: &mut Paddle) {

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
        .unwrap();

    display_walls();
    display_paddles(self1, self2);
    display_ball(self_b);

    unsafe{game_started = true}
}
fn ball_update(self_b: &mut Ball, self1: &mut Paddle, self2: &mut Paddle) {

    draw(self_b.x as u16, self_b.y as u16, " ".to_string());


    ball_fondler(self_b, self1, self2);

    self_b.x += self_b.x_speed;
    self_b.y += self_b.y_speed;

    display_ball(self_b);

    thread::sleep(Duration::from_millis(50))
}
fn move_paddle(paddle_type: bool, direction: bool, self1: &mut Paddle, self2: &mut Paddle) {

    for g in 0..3 {
        draw(self1.x as u16, self1.y as u16 - g, "  ".to_string());
        draw(self1.x as u16, self1.y as u16 + g, "  ".to_string());
        draw(self2.x as u16, self2.y as u16 - g, "  ".to_string());
        draw(self2.x as u16, self2.y as u16 + g, "  ".to_string())
    }

    paddle_fondler(paddle_type, direction, self1, self2);

    if paddle_type == true {
        if direction == true {
            self1.y += graphics::PADDLE_MOVE_VALUE
        } else if direction == false {
            self1.y -= graphics::PADDLE_MOVE_VALUE
        }
    } else if paddle_type == false {
        if direction == true {
            self2.y += graphics::PADDLE_MOVE_VALUE
        } else if direction == false {
            self2.y -= graphics::PADDLE_MOVE_VALUE
        }
    }

    display_paddles(self1, self2);
}
fn paddle_fondler(paddle_type: bool, direction: bool, self1: &mut Paddle, self2: &mut Paddle) {

    if paddle_type == true {
        // down
        if direction == true {
            if self1.y >= graphics::HEIGHT_LOW - 3 {
                self1.y -= graphics::PADDLE_MOVE_VALUE
            }
        // up
        } else if direction == false {
           if self1.y <= graphics::HEIGHT_HIGH + 3 {
               self1.y += graphics::PADDLE_MOVE_VALUE
           }
        }
    } else if paddle_type == false {
        // down
        if direction == true {
            if self2.y >= graphics::HEIGHT_LOW - 3 {
                self2.y -= graphics::PADDLE_MOVE_VALUE
            }
            // up
        } else if direction == false {
            if self2.y <= graphics::HEIGHT_HIGH + 3 {
                self2.y += graphics::PADDLE_MOVE_VALUE
            }
        }
    }

}
fn ball_fondler(self_b: &mut Ball, self1: &mut Paddle, self2: &mut Paddle) {

    if self_b.x >= graphics::LENGTH_RIGHT as i32 - 2 || self_b.x <= graphics::LENGTH_LEFT as i32 + 2 {
        if self_b.x >= graphics::LENGTH_RIGHT as i32 - 1 {
            unsafe{score_1 += 1}
        } else if self_b.x <= graphics::LENGTH_LEFT as i32 + 1 {
            unsafe{score_2 += 1}
        }
        self_b.x_speed *= -1;
        self_b.x = ((graphics::LENGTH_RIGHT as i32 - graphics::LENGTH_LEFT as i32) + 1) / 2 + graphics::LENGTH_LEFT;
        self_b.y = graphics::HEIGHT_LOW as i32 / 2;

    } else if self_b.y >= graphics::HEIGHT_LOW as i32 - 1 || self_b.y <= graphics::HEIGHT_HIGH as i32 + 1 {
        self_b.y_speed *= -1;

    }

    if self_b.x <= self1.x + 2 && self_b.y <= self1.y + 2 && self_b.y >= self1.y - 2 {
        self_b.x_speed *= -1
    } else if self_b.x >= self2.x - 2 && self_b.y <= self2.y + 2 && self_b.y >= self2.y - 2 {
        self_b.x_speed *= -1
    }
}
fn init(self_b: &mut Ball, self1: &mut Paddle, self2: &mut Paddle) {

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide)
        .unwrap();


    default_setup();

    loop {

        // Keybindings

        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        } else if let Some(Ok(b'g')) = b {
            game_setup(self_b, self1, self2);
        } else if let Some(Ok(b'm')) = b {
            move_paddle(false, true, self1, self2);
        } else if let Some(Ok(b'k')) = b {
            move_paddle(false, false, self1, self2)
        } else if let Some(Ok(b'z')) = b {
            move_paddle(true, true, self1, self2)
        } else if let Some(Ok(b'a')) = b {
            move_paddle(true, false, self1, self2)
        }
        stdout.flush().unwrap();

        if unsafe{game_started == true} {
            ball_update(self_b, self1, self2);

            display_score()


        }


    }
}
fn display_score() {
    draw((((graphics::LENGTH_RIGHT as i32 - graphics::LENGTH_LEFT as i32) + 1) / 2 + graphics::LENGTH_LEFT) as u16, (graphics::HEIGHT_HIGH as i32 + 1) as u16, "|".to_string());
    draw((((graphics::LENGTH_RIGHT as i32 - graphics::LENGTH_LEFT as i32) + 1) / 2 + graphics::LENGTH_LEFT - 2) as u16, (graphics::HEIGHT_HIGH as i32 + 1) as u16, unsafe{score_1}.to_string());
    draw((((graphics::LENGTH_RIGHT as i32 - graphics::LENGTH_LEFT as i32) + 1) / 2 + graphics::LENGTH_LEFT + 2) as u16, (graphics::HEIGHT_HIGH as i32 + 1) as u16, unsafe{score_2}.to_string())
}

fn main() {

    let mut ball: Ball = Ball {
        //x: 52,
        x: ((graphics::LENGTH_RIGHT as i32 - graphics::LENGTH_LEFT as i32) + 1) / 2 + graphics::LENGTH_LEFT,
        y: graphics::HEIGHT_LOW as i32 / 2,

        x_speed: 1,
        y_speed: 1
    };

    let mut paddle1: Paddle = Paddle {
        x: graphics::LENGTH_LEFT as i32 + 2,
        y: graphics::HEIGHT_LOW as i32 / 2 + 1
    };

    let mut paddle2: Paddle = Paddle {
        x: graphics::LENGTH_RIGHT as i32 - 3,
        y: graphics::HEIGHT_LOW as i32 / 2 + 1
    };



    init(&mut ball, &mut paddle1, &mut paddle2);
}



