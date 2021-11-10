//! Keys used to style the application

use druid::{theme::*, Color, Env, FontDescriptor, FontFamily, FontStyle, FontWeight, Key};

/// Font used for large and bold text like central titles
pub const HEADER_FONT: Key<FontDescriptor> = Key::new("recipier.header-font");
/// Larger and bold font for sub headers
pub const LABEL_FONT: Key<FontDescriptor> = Key::new("recipier.label-font");
/// The System UI font to use with general text
pub const SYSTEM_FONT: Key<FontDescriptor> = Key::new("recipier.system-ui-font");
/// Small font to use for short instructions / tooltips
pub const SMALL_FONT: Key<FontDescriptor> = Key::new("recipier.small-font");

pub const COLOR_1: Key<Color> = Key::new("recipier.color1");
pub const COLOR_2: Key<Color> = Key::new("recipier.color2");
pub const COLOR_3: Key<Color> = Key::new("recipier.color3");
pub const COLOR_4: Key<Color> = Key::new("recipier.color4");

pub const SPACING: f64 = 5.;

/// Set environment keys to their appropriate values
pub fn set(env: &mut Env) {
    env.set(
        HEADER_FONT,
        FontDescriptor::new(FontFamily::SYSTEM_UI)
            .with_size(24.)
            .with_weight(FontWeight::HEAVY),
    );
    env.set(
        LABEL_FONT,
        FontDescriptor::new(FontFamily::SYSTEM_UI)
            .with_size(16.)
            .with_weight(FontWeight::BOLD),
    );
    env.set(
        SYSTEM_FONT,
        FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.),
    );
    env.set(
        SMALL_FONT,
        FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(11.),
    );

    env.set(COLOR_1, Color::from_hex_str("#F0F5F9").unwrap());
    env.set(COLOR_2, Color::from_hex_str("#C9D6DF").unwrap());
    env.set(COLOR_3, Color::from_hex_str("#52616B").unwrap());
    env.set(COLOR_4, Color::from_hex_str("#1E2022").unwrap());

    //env.set(COLOR_1, Color::from_hex_str("#F5E8C7").unwrap());
    //env.set(COLOR_2, Color::from_hex_str("#DEBA9D").unwrap());
    //env.set(COLOR_3, Color::from_hex_str("#9E7777").unwrap());
    //env.set(COLOR_4, Color::from_hex_str("#6F4C5B").unwrap());

    env.set(BORDER_DARK, env.get(COLOR_3));
    env.set(BORDER_LIGHT, env.get(COLOR_2));
    env.set(SCROLLBAR_BORDER_COLOR, env.get(COLOR_2));

    env.set(BUTTON_DARK, env.get(COLOR_2));
    env.set(BUTTON_LIGHT, env.get(COLOR_2));

    env.set(UI_FONT, env.get(SYSTEM_FONT));
    env.set(UI_FONT_BOLD, env.get(LABEL_FONT));
    env.set(
        UI_FONT_ITALIC,
        env.get(SYSTEM_FONT).with_style(FontStyle::Italic),
    );

    env.set(LABEL_COLOR, env.get(COLOR_3));
    env.set(PRIMARY_LIGHT, env.get(COLOR_2));
    env.set(PRIMARY_DARK, env.get(COLOR_3));

    env.set(SELECTION_COLOR, env.get(COLOR_3));
    env.set(SELECTION_TEXT_COLOR, env.get(COLOR_1));

    env.set(BACKGROUND_DARK, env.get(COLOR_1));
    env.set(BACKGROUND_LIGHT, env.get(COLOR_1));
    env.set(WINDOW_BACKGROUND_COLOR, env.get(COLOR_1));

    env.set(CURSOR_COLOR, env.get(COLOR_4));
}
