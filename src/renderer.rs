use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use emu_config::EmuConfig;

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;

pub struct Renderer {
    context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>
}

impl Renderer {
    pub fn new(config: EmuConfig) -> Renderer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();
               

        Renderer {
            context: sdl_context,
            canvas: canvas
        }
    }

    pub fn start_loop(&mut self) {
        let mut event_pump = self.context.event_pump().unwrap();
        'running: loop {
            self.canvas.clear();
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
