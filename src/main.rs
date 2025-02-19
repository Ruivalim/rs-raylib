use chrono::{NaiveTime, Utc};
use ffi::Vector2;
use rand::prelude::*;
use raylib::{ffi::CheckCollisionPointCircle, prelude::*};

const WINDOW_WIDTH: i32 = 1200;
const WINDOW_HEIGHT: i32 = 800;

enum GameScreens {
    MainScreen,
    GameRunning,
    WinningScreen,
}

struct Target {
    pos_x: f32,
    pos_y: f32,
    dir_y: f32,
    dir_x: f32,
}

enum GameDifficult {
    Easy,
    Medium,
    Hard,
}

struct GameState {
    current_screen: GameScreens,
    targets: Vec<Target>,
    started_at: Option<NaiveTime>,
    finished_at: Option<NaiveTime>,
    clicks: i32,
    difficult: GameDifficult,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Rust + Raylib = <3")
        .build();

    rl.set_exit_key(Some(KeyboardKey::KEY_NULL));

    let mut rng = rand::rng();

    let mut game_state: GameState = GameState {
        current_screen: GameScreens::MainScreen,
        targets: vec![],
        started_at: None,
        finished_at: None,
        clicks: 0,
        difficult: GameDifficult::Easy,
    };

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        match game_state.current_screen {
            GameScreens::MainScreen => update_main_screen(&mut rl, &mut game_state, &mut rng),
            GameScreens::GameRunning => update_game_screen(&mut rl, &mut game_state),
            GameScreens::WinningScreen => update_wining_screen(&mut rl, &mut game_state),
        }

        let mut canvas = rl.begin_drawing(&thread);

        match game_state.current_screen {
            GameScreens::MainScreen => draw_main_screen(&mut canvas, &game_state),
            GameScreens::GameRunning => draw_game_screen(&mut canvas, &game_state),
            GameScreens::WinningScreen => draw_wining_screen(&mut canvas, &game_state),
        }
    }
}

fn update_main_screen(rl: &mut RaylibHandle, game_state: &mut GameState, rng: &mut ThreadRng) {
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        game_state.started_at = Some(Utc::now().time());
        game_state.targets = vec![];
        game_state.clicks = 0;
        for _ in 0..10 {
            let pos_y = rng.random_range(30.0..(WINDOW_HEIGHT - 30) as f32);
            let pos_x = rng.random_range(30.0..(WINDOW_WIDTH - 30) as f32);
            let mut dir_y = match game_state.difficult {
                GameDifficult::Easy => 1.0,
                GameDifficult::Hard => 5.0,
                GameDifficult::Medium => 3.0,
            };
            let mut dir_x = match game_state.difficult {
                GameDifficult::Easy => 1.0,
                GameDifficult::Hard => 5.0,
                GameDifficult::Medium => 3.0,
            };

            if rng.random() {
                dir_x *= -1.0;
            }

            if rng.random() {
                dir_y *= -1.0;
            }
            game_state.targets.push(Target {
                pos_x,
                pos_y,
                dir_y,
                dir_x,
            });
        }

        game_state.current_screen = GameScreens::GameRunning
    }

    if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
        game_state.difficult = GameDifficult::Easy
    }
    if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
        game_state.difficult = GameDifficult::Medium
    }
    if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
        game_state.difficult = GameDifficult::Hard
    }
}

fn draw_main_screen(canvas: &mut RaylibDrawHandle, game_state: &GameState) {
    canvas.clear_background(Color::new(135, 206, 235, 255));
    let text = "Clicker Game";
    let text_fsize = 40;
    let text_size = canvas.measure_text(&text, text_fsize);
    canvas.draw_text(
        &text,
        (WINDOW_WIDTH / 2) - (text_size / 2),
        (WINDOW_HEIGHT / 2) - 100,
        text_fsize,
        Color::BLACK,
    );

    let current_difficult = match game_state.difficult {
        GameDifficult::Easy => "Easy",
        GameDifficult::Hard => "Hard",
        GameDifficult::Medium => "Medium",
    };

    let text = format!("Current difficult: {}", current_difficult);
    let text_fsize = 20;
    let text_size = canvas.measure_text(&text, text_fsize);
    canvas.draw_text(
        &text,
        (WINDOW_WIDTH / 2) - (text_size / 2),
        (WINDOW_HEIGHT / 2) - 25,
        text_fsize,
        Color::BLACK,
    );

    let text = "Press 1 to easy; 2 to medium and 3 to hard";
    let text_fsize = 20;
    let text_size = canvas.measure_text(&text, text_fsize);
    canvas.draw_text(
        &text,
        (WINDOW_WIDTH / 2) - (text_size / 2),
        (WINDOW_HEIGHT / 2) + 25,
        text_fsize,
        Color::BLACK,
    );

    let text = "Press Space to start";
    let text_fsize = 20;
    let text_size = canvas.measure_text(&text, text_fsize);
    canvas.draw_text(
        &text,
        (WINDOW_WIDTH / 2) - (text_size / 2),
        (WINDOW_HEIGHT / 2) + 100,
        text_fsize,
        Color::BLACK,
    );
}

fn update_game_screen(rl: &mut RaylibHandle, game_state: &mut GameState) {
    if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
        game_state.current_screen = GameScreens::MainScreen
    }

    let mut targets: Vec<Target> = vec![];

    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        game_state.clicks += 1;
    }

    for target in game_state.targets.iter_mut() {
        let mut keep_alive = true;
        target.pos_x += target.dir_x;
        target.pos_y += target.dir_y;

        if target.pos_x >= (WINDOW_WIDTH - 15) as f32 || target.pos_x <= 15. {
            target.dir_x = target.dir_x * -1.0;
        }

        if target.pos_y >= (WINDOW_HEIGHT - 15) as f32 || target.pos_y <= 15. {
            target.dir_y = target.dir_y * -1.0;
        }
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position().into();

            unsafe {
                if CheckCollisionPointCircle(
                    mouse_pos,
                    Vector2 {
                        x: target.pos_x,
                        y: target.pos_y,
                    },
                    15.0,
                ) {
                    keep_alive = false
                }
            }
        }

        if keep_alive {
            targets.push(Target {
                pos_x: target.pos_x,
                pos_y: target.pos_y,
                dir_y: target.dir_y,
                dir_x: target.dir_x,
            })
        }
    }

    if targets.len() == 0 {
        game_state.finished_at = Some(Utc::now().time());
        game_state.current_screen = GameScreens::WinningScreen
    }
    game_state.targets = targets;
}

fn draw_game_screen(canvas: &mut RaylibDrawHandle, game_state: &GameState) {
    canvas.clear_background(Color::new(135, 206, 235, 255));

    for target in game_state.targets.iter() {
        canvas.draw_circle(target.pos_x as i32, target.pos_y as i32, 15.0, Color::RED);
    }
    canvas.draw_fps(WINDOW_WIDTH - 100, 0);

    let now = Utc::now().time();
    let diff = now - game_state.started_at.unwrap();
    let timer_text = format!("{}.{}", diff.num_seconds(), diff.num_milliseconds());
    let timer_size = canvas.measure_text(&timer_text, 20);

    canvas.draw_text(
        &timer_text,
        (WINDOW_WIDTH / 2) - (timer_size / 2),
        10,
        20,
        Color::RED,
    );

    canvas.draw_text(
        &format!("Clicks: {}", game_state.clicks),
        10,
        10,
        20,
        Color::BLACK,
    );
}

fn update_wining_screen(rl: &mut RaylibHandle, game_state: &mut GameState) {
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        game_state.current_screen = GameScreens::MainScreen;
    }
}

fn draw_wining_screen(canvas: &mut RaylibDrawHandle, game_state: &GameState) {
    canvas.clear_background(Color::new(135, 206, 235, 255));
    let text = "Congrats!";
    let text_size = canvas.measure_text(text, 50);
    canvas.draw_text(
        text,
        (WINDOW_WIDTH / 2) - (text_size / 2),
        (WINDOW_HEIGHT / 2) - 100,
        50,
        Color::GREEN,
    );

    let diff = game_state.finished_at.unwrap() - game_state.started_at.unwrap();
    let timer_text = format!(
        "You finished in {}.{} seconds, with {} total clicks!",
        diff.num_seconds(),
        diff.num_milliseconds(),
        game_state.clicks
    );
    let timer_size = canvas.measure_text(&timer_text, 40);

    canvas.draw_text(
        &timer_text,
        (WINDOW_WIDTH / 2) - (timer_size / 2),
        WINDOW_HEIGHT / 2,
        40,
        Color::GOLD,
    );

    let restart_text = "Press Enter to restart";
    let restart_fsize = 20;
    let restart_size = canvas.measure_text(&restart_text, restart_fsize);

    canvas.draw_text(
        &restart_text,
        (WINDOW_WIDTH / 2) - (restart_size / 2),
        (WINDOW_HEIGHT / 2) + 100,
        restart_fsize,
        Color::BLACK,
    )
}
