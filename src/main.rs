use ffi::Vector2;
use raylib::{ffi::CheckCollisionPointCircle, ffi::GetRandomValue, prelude::*};

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

struct GameState {
    current_screen: GameScreens,
    points: i32,
    targets: Vec<Target>,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Rust + Raylib = <3")
        .build();

    rl.set_exit_key(Some(KeyboardKey::KEY_NULL));

    let mut game_state: GameState = GameState {
        current_screen: GameScreens::MainScreen,
        points: 0,
        targets: vec![],
    };

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        match game_state.current_screen {
            GameScreens::MainScreen => {
                update_main_screen(&mut rl, &mut game_state);
            }
            GameScreens::GameRunning => {
                update_game_screen(&mut rl, &mut game_state);
            }
            GameScreens::WinningScreen => update_wining_screen(&mut rl, &mut game_state),
        }

        let mut canvas = rl.begin_drawing(&thread);

        match game_state.current_screen {
            GameScreens::MainScreen => {
                draw_main_screen(&mut canvas);
            }
            GameScreens::GameRunning => {
                draw_game_screen(&mut canvas, &game_state);
            }
            GameScreens::WinningScreen => draw_wining_screen(&mut canvas, &game_state),
        }
    }
}

fn update_main_screen(rl: &mut RaylibHandle, game_state: &mut GameState) {
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        game_state.targets = vec![];
        for _ in 0..10 {
            unsafe {
                let pos_y = GetRandomValue(30, WINDOW_HEIGHT - 30) as f32;
                let pos_x = GetRandomValue(30, WINDOW_WIDTH - 30) as f32;
                let dir_y = GetRandomValue(-2, 2) as f32;
                let dir_x = GetRandomValue(-2, 2) as f32;

                game_state.targets.push(Target {
                    pos_x,
                    pos_y,
                    dir_y,
                    dir_x,
                });
            }
        }

        game_state.current_screen = GameScreens::GameRunning
    }
}

fn draw_main_screen(canvas: &mut RaylibDrawHandle) {
    canvas.clear_background(Color::new(135, 206, 235, 255));
    let text_size = canvas.measure_text("Hello Raylib", 20);
    canvas.draw_text(
        "Hello Raylib",
        (WINDOW_WIDTH / 2) - (text_size / 2),
        (WINDOW_HEIGHT / 2) - 10,
        20,
        Color::BLACK,
    );
}

fn update_game_screen(rl: &mut RaylibHandle, game_state: &mut GameState) {
    if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
        game_state.current_screen = GameScreens::MainScreen
    }

    let mut targets: Vec<Target> = vec![];

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
                    game_state.points += 1;
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

    canvas.draw_text(
        &format!("Points: {}", game_state.points),
        20,
        20,
        20,
        Color::BLUEVIOLET,
    );
}

fn update_wining_screen(rl: &mut RaylibHandle, game_state: &mut GameState) {
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        game_state.current_screen = GameScreens::MainScreen;
    }
}

fn draw_wining_screen(canvas: &mut RaylibDrawHandle, _game_state: &GameState) {
    canvas.clear_background(Color::new(135, 206, 235, 255));
    let text = "Congrats, you won!";
    let text_size = canvas.measure_text(text, 20);
    canvas.draw_text(
        text,
        (WINDOW_WIDTH / 2) - (text_size / 2),
        (WINDOW_HEIGHT / 2) - 10,
        20,
        Color::GREEN,
    );
}
