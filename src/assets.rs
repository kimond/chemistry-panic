use ggez::{Context, GameResult};
use ggez::graphics;

pub struct Assets {
    pub font_small: graphics::Font,
    pub title: graphics::Text,
    pub intro_text: graphics::Text,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font_title = graphics::Font::new(ctx, "/CHEMISTR.TTF", 18)?;
        let font_small = graphics::Font::new(ctx, "/chemistry.ttf", 12)?;
        let title = graphics::Text::new(ctx, "Chemistry Panic", &font_title)?;
        let intro_text = graphics::Text::new(ctx, "Press SPACE to start!", &font_small)?;

        let a = Assets {
            font_small,
            title,
            intro_text
        };

        Ok(a)
    }
}