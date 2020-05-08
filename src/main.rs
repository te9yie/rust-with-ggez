extern crate ggez;

use ggez::*;

struct MainState {
    image: graphics::Image,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "/ascii.png")?;
        let state = MainState { image: image };
        Ok(state)
    }

    fn draw_text(
        &self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        text: &str,
        color: graphics::Color,
    ) -> GameResult {
        let mut offset = 0.;
        for c in text.chars() {
            let i = c as i32;
            let u = (i & 0x0f) as f32 * 8.;
            let v = ((i >> 4) & 0x0f) as f32 * 16.;
            graphics::draw(
                ctx,
                &self.image,
                graphics::DrawParam::new()
                    .src(graphics::Rect::new(
                        u / 128.,
                        v / 128.,
                        8. / 128.,
                        16. / 128.,
                    ))
                    .dest(mint::Point2 {
                        x: x + offset,
                        y: y,
                    })
                    .color(color),
            )?;
            offset += 8.;
        }
        Ok(())
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        use graphics as g;
        g::clear(ctx, [0.17, 0.17, 0.17, 1.0].into());
        self.draw_text(
            ctx,
            10.,
            10.,
            "the quick brown fox jumps over the lazy dog",
            g::WHITE,
        )?;
        self.draw_text(
            ctx,
            10.,
            30.,
            "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG",
            g::Color::new(1., 0., 0., 1.),
        )?;
        g::present(ctx)
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let (ctx, event_loop) = &mut ContextBuilder::new("with-ggez", "te9yie")
        .window_setup(conf::WindowSetup {
            title: "with ggez".to_string(),
            ..Default::default()
        })
        .window_mode(conf::WindowMode {
            width: 16. * 40.,
            height: 9. * 40.,
            ..Default::default()
        })
        .add_resource_path(resource_dir)
        .build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
