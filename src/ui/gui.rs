//! Simple, SDL2-based GUI

/// Initialize the GUI context
///
/// TODO: Return a Result with a proper type
pub struct Gui {
    canvas: sdl2::render::WindowCanvas,
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

        Some(Gui { canvas })
    }
}
