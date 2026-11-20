
use macroquad::prelude::*;

// Main menu layout
const LEFT_MARGIN: f32 = 60.0;
const TITLE_Y: f32 = 140.0;
const INSTRUCTIONS_Y: f32 = 220.0;
const LINE_SPACING: f32 = 35.0;

const TITLE_FONT_SIZE: f32 = 42.0;
const BODY_FONT_SIZE: f32 = 24.0;

// Start button settings
const BUTTON_WIDTH: f32 = 220.0;
const BUTTON_HEIGHT: f32 = 60.0;
const BUTTON_Y_RATIO: f32 = 0.7;
const BUTTON_FONT_SIZE: f32 = 28.0;


const GAME_OVER_TITLE_SIZE: f32 = 48.0;
const GAME_OVER_SCORE_SIZE: f32 = 30.0;
const GAME_OVER_HINT_SIZE: f32 = 20.0;
const GAME_OVER_BUTTON_GAP: f32 = 20.0;


const GAME_OVER_AGAIN_Y_RATIO: f32 = 0.54;

pub fn draw_main_menu() {
    clear_background(BLACK);

    // Game title
    draw_text(
        "RUST DEFENDER 2026",
        LEFT_MARGIN,
        TITLE_Y,
        TITLE_FONT_SIZE,
        WHITE,
    );

    // Player instructions
    draw_text(
        "Arrow Keys: Move",
        LEFT_MARGIN,
        INSTRUCTIONS_Y,
        BODY_FONT_SIZE,
        LIGHTGRAY,
    );

    draw_text(
        "Space: Shoot",
        LEFT_MARGIN,
        INSTRUCTIONS_Y + LINE_SPACING,
        BODY_FONT_SIZE,
        LIGHTGRAY,
    );

    
    // Calculate button position
    let button_x = (screen_width() - BUTTON_WIDTH) / 2.0;
    let button_y = screen_height() * BUTTON_Y_RATIO;

    // Draw button background
    draw_rectangle(
        button_x,
        button_y,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
        GREEN,
    );

    // Calculate text position
    let label = "START GAME";
    let text_size = measure_text(label, None, BUTTON_FONT_SIZE as u16, 1.0);

    let text_x = button_x + (BUTTON_WIDTH - text_size.width) / 2.0;
    let text_y = button_y + (BUTTON_HEIGHT + text_size.height) / 2.0;

    // Draw button label
    draw_text(label, text_x, text_y, BUTTON_FONT_SIZE, WHITE);

}

pub fn start_button_clicked() -> bool {
    let (mouse_x, mouse_y) = mouse_position();

    let button_x = (screen_width() - BUTTON_WIDTH) / 2.0;
    let button_y = screen_height() * BUTTON_Y_RATIO;

    let inside_button =
        mouse_x >= button_x
        && mouse_x <= button_x + BUTTON_WIDTH
        && mouse_y >= button_y
        && mouse_y <= button_y + BUTTON_HEIGHT;

    inside_button && is_mouse_button_pressed(MouseButton::Left)
}


fn draw_centered_text(text: &str, y: f32, size: f32, color: Color) {
    let dimensions = measure_text(text, None, size as u16, 1.0);
    let x = (screen_width() - dimensions.width) / 2.0;

    draw_text(text, x, y, size, color);
}

fn draw_menu_button(label: &str, y: f32, color: Color) {
    let x = (screen_width() - BUTTON_WIDTH) / 2.0;

    draw_rectangle(x, y, BUTTON_WIDTH, BUTTON_HEIGHT, color);

    let dimensions = measure_text(label, None, BUTTON_FONT_SIZE as u16, 1.0);

    let text_x = x + (BUTTON_WIDTH - dimensions.width) / 2.0;
    let text_y = y + (BUTTON_HEIGHT + dimensions.height) / 2.0;

    draw_text(label, text_x, text_y, BUTTON_FONT_SIZE, WHITE);
}

pub fn draw_game_over(score: u32) {
    clear_background(BLACK);

    draw_centered_text(
        "GAME OVER",
        screen_height() * 0.27,
        GAME_OVER_TITLE_SIZE,
        RED,
    );

    draw_centered_text(
        &format!("FINAL SCORE: {}", score),
        screen_height() * 0.41,
        GAME_OVER_SCORE_SIZE,
        WHITE,
    );

    let again_y = screen_height() * GAME_OVER_AGAIN_Y_RATIO;
    let exit_y = again_y + BUTTON_HEIGHT + GAME_OVER_BUTTON_GAP;

    draw_menu_button("PLAY AGAIN", again_y, GREEN);
    draw_menu_button("EXIT GAME", exit_y, DARKGRAY);

    draw_centered_text(
        "Press R to restart",
        exit_y + BUTTON_HEIGHT + 40.0,
        GAME_OVER_HINT_SIZE,
        LIGHTGRAY,
    );
}


pub fn play_again_clicked() -> bool {
    let (mouse_x, mouse_y) = mouse_position();

    let button_x = (screen_width() - BUTTON_WIDTH) / 2.0;
    let button_y = screen_height() * GAME_OVER_AGAIN_Y_RATIO;

    let inside_button =
        mouse_x >= button_x
        && mouse_x <= button_x + BUTTON_WIDTH
        && mouse_y >= button_y
        && mouse_y <= button_y + BUTTON_HEIGHT;

    inside_button && is_mouse_button_pressed(MouseButton::Left)
}


pub fn exit_button_clicked() -> bool {
    let (mouse_x, mouse_y) = mouse_position();

    // Calculate EXIT GAME button position
    let button_x = (screen_width() - BUTTON_WIDTH) / 2.0;
    let button_y = screen_height() * GAME_OVER_AGAIN_Y_RATIO
        + BUTTON_HEIGHT
        + GAME_OVER_BUTTON_GAP;

    // Check whether mouse is inside the button
    let inside_button =
        mouse_x >= button_x
        && mouse_x <= button_x + BUTTON_WIDTH
        && mouse_y >= button_y
        && mouse_y <= button_y + BUTTON_HEIGHT;

    inside_button && is_mouse_button_pressed(MouseButton::Left)
}
