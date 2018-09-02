use core::ppu::PPU;
use renderer::{Renderer, RenderingState};

pub struct HeadlessRenderer {
    rom_path: String
}

impl HeadlessRenderer {
    pub fn new(rom_path: &String) -> HeadlessRenderer {
        info!("Creating a headless renderer...");
        HeadlessRenderer {
            rom_path: rom_path.to_owned()
        }
    }
}

impl Renderer<HeadlessRenderer> for HeadlessRenderer {
    fn start_loop<F>(&mut self, mut update: F, run: &RenderingState) where F: FnMut(&mut HeadlessRenderer) {
        info!("Starting main loop");
        info!("Rom: {}", self.rom_path);
        loop {
            update(self);
            if run.state == "stop" {
                break;
            }
        };
    }

    fn render_screen(&mut self, ppu: &mut PPU) {}
}
