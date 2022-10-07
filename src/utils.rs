use macroquad::prelude::{
    draw_text, get_time, measure_text, screen_height, screen_width, Vec2, DARKGRAY,
};

pub fn wrap_around(v: &Vec2) -> Vec2 {
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > screen_width() {
        vr.x = 0.;
    }
    if vr.x < 0. {
        vr.x = screen_width()
    }
    if vr.y > screen_height() {
        vr.y = 0.;
    }
    if vr.y < 0. {
        vr.y = screen_height()
    }
    vr
}

pub fn get_screen_center() -> Vec2 {
    Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
}

pub fn get_elapsed_time() -> f64 {
    get_time()
}

pub fn draw_message(msg: &str, font_size: f64) {
    let text_size = measure_text(msg, None, font_size as _, 1.0);
    draw_text(
        msg,
        screen_width() / 2.0 - text_size.width / 2.0,
        screen_height() / 2.0 - text_size.height / 2.0,
        font_size as _,
        DARKGRAY,
    );
}
