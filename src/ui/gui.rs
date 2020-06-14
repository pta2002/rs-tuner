//! Simple, SDL2-based GUI

/// Initialize the GUI context
///
/// TODO: Return a Result with a proper type
use sdl2::event::Event;
use sdl2::pixels::Color;

pub struct Gui {
    canvas: sdl2::render::WindowCanvas,
    context: sdl2::Sdl,
}

impl Gui {
    pub fn new() -> Option<Gui> {
        let sdl_context = sdl2::init().ok()?;
        let video_subsystem = sdl_context.video().ok()?;

        // TODO have this configurable via init file
        let mut window = video_subsystem
            .window("tuner", 300, 50)
            .position_centered()
            .build()
            .ok()?;

        window.set_opacity(0.5);

        let mut canvas = window.into_canvas().build().ok()?;

        canvas.window_mut().show();

        Some(Gui {
            canvas,
            context: sdl_context,
        })
    }

    pub fn draw(&mut self) {
        let mut event_pump = self.context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                _ => {}
            }
        }
        self.canvas.set_draw_color(Color::RGBA(20, 20, 20, 10));
        self.canvas.clear();
        self.canvas.present();
    }
}
