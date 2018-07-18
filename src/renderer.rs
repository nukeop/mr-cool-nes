use sdl2;
use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render;
use sdl2::surface::Surface;
use std::path::Path;
use emu_config::EmuConfig;

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;
const EMULATOR_FRAME_HEIGHT: u32 = 32;

pub struct Renderer {
    config: EmuConfig,
    context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    font: Surface<'static>,
    emu_frame: Surface<'static>,
    emu_screen: Surface<'static>
}

impl Renderer {
    pub fn new(config: EmuConfig) -> Renderer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem.window(
            "Mr Cool NES",
            SCREEN_WIDTH * (config.screen_size as u32),
            (EMULATOR_FRAME_HEIGHT + SCREEN_HEIGHT) * (config.screen_size as u32)
        )
            .position_centered()
            .borderless()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();

        canvas.clear();
        canvas.present();

        let font = Renderer::create_font_surface(Path::new(&config.font_path));
        let emu_frame = Surface::new(SCREEN_WIDTH, EMULATOR_FRAME_HEIGHT, PixelFormatEnum::RGB24).unwrap();
        let emu_screen = Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, PixelFormatEnum::RGB24).unwrap();
        
        Renderer {
            config: config,
            context: sdl_context,
            canvas: canvas,
            font: font,
            emu_frame: emu_frame,
            emu_screen: emu_screen
        }
    }

    pub fn create_font_surface(path: &Path) -> Surface<'static> {
        return Surface::from_file(path).unwrap();
    }

    pub fn start_loop(&mut self) {
        let mut event_pump = self.context.event_pump().unwrap();
        'running: loop {
            self.canvas.clear();

            let creator = self.canvas.texture_creator();
            let texture = creator.create_texture_from_surface(self.font.as_ref());
            self.canvas.copy(
                &texture.unwrap(),
                Rect::new(16, 32, 16, 16),
                Rect::new(0, 0, 16, 16)
            );
            
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            self.canvas.present();
        }
    }
}
