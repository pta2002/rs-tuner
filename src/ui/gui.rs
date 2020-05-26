//! Simple, SDL2-based GUI

/// Initialize the GUI context
///
/// TODO: Return a Result with a proper type
pub struct Gui {
    canvas: sdl2::render::Canvas,
    window: sdl2::video::Window,
}

pub fn init() -> Option<()> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // TODO have this configurable via init file
    let mut window = video_subsystem
        .window("tuner", 300, 50)
        .position_centered()
        .build()?;

    window.set_opacity(0.5);

    let mut canvas = window.into_canvas().build()?;

    Gui { canvas, window }
}
