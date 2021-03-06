use sdl2;
use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render;
use sdl2::surface::{Surface, SurfaceRef};
use std::path::Path;
use std::ffi::OsString;

use core::ppu::PPU;
use font_map::get_letter;
use emu_config::EmuConfig;
use renderer::{Renderer, RenderingState};

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;
const EMULATOR_FRAME_HEIGHT: u32 = 32;


pub struct SDLRenderer {
    rom_path: String,
    context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    font: Surface<'static>,
    emu_frame: Surface<'static>,
    emu_screen: Surface<'static>,
    screen_size: u8
}

impl SDLRenderer {
    pub fn new(config: &EmuConfig, rom_path: &String) -> SDLRenderer {
        info!("Creating an SDL renderer...");
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem.window(
            "Mr. Cool NES",
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

        let font = SDLRenderer::create_font_surface(Path::new(&config.font_path));
        let emu_frame = Surface::new(SCREEN_WIDTH, EMULATOR_FRAME_HEIGHT, PixelFormatEnum::BGR24).unwrap();
        let emu_screen = Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, PixelFormatEnum::BGR24).unwrap();
        
        SDLRenderer {
            rom_path: rom_path.to_owned(),
            context: sdl_context,
            canvas,
            font,
            emu_frame,
            emu_screen,
            screen_size: config.screen_size
        }
    }

    pub fn create_font_surface(path: &Path) -> Surface<'static> {
        return Surface::from_file(path).unwrap();
    }

    pub fn draw_text(&mut self, text: &String, x: u32, y: u32) {
        let creator = self.canvas.texture_creator();
        let texture = creator.create_texture_from_surface(self.font.as_ref()).unwrap();

        for (i, letter) in text.chars().enumerate() {
            let rect = get_letter(&letter);
            self.canvas.copy(
                &texture,
                rect,
                Rect::new((x + (i as u32)*16) as i32, y as i32, 16, 16)
            ).unwrap();
        }
    }

    pub fn draw_title(&mut self) {
        self.draw_text(&("Mr. Cool NES".to_owned()), 0, 0);
    } 
}

impl Renderer<SDLRenderer> for SDLRenderer {
    fn start_loop<F>(&mut self, mut update: F, run: &RenderingState) where F: FnMut(&mut SDLRenderer) {
        info!("Starting render loop");
        let rom_name = self.rom_path.to_owned();
        let rom_name_path = Path::new(&rom_name);
        let rom_filename = rom_name_path
            .file_name()
            .expect("Couldn't extract filename from rom path")
            .to_str()
            .expect("Couldn't convert filename to string")
            .to_owned();
        
        let mut event_pump = self.context.event_pump().unwrap();
        'running: loop {
            // Draw
            self.canvas.clear();
            self.draw_title();
            self.draw_text(&("ROM: ".to_owned() + &rom_filename), 0, 16);

            //Update
            update(self);
            
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        info!("Quit event received, shutting down");
                        break 'running
                    },
                    _ => {}
                }
            }
            self.canvas.present();
        }
    }

    fn render_screen(&mut self, ppu: &mut PPU) {
        let creator = self.canvas.texture_creator();
        let mut texture = creator.create_texture_streaming(PixelFormatEnum::BGR24, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
        let screen = ppu.get_screen();
        
        texture.update(None, &screen, SCREEN_WIDTH as usize * 3).unwrap();
        
        self.canvas.copy(
            &texture,
            None,
            Rect::new(
                0,
                EMULATOR_FRAME_HEIGHT as i32,
                SCREEN_WIDTH * self.screen_size as u32,
                SCREEN_HEIGHT * self.screen_size as u32
            )
        ).unwrap();
    }
}
