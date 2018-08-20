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

impl Renderer for HeadlessRenderer {
    fn start_loop<F>(&mut self, mut update: F, run: &RenderingState) where F: FnMut() -> () {
        info!("Starting main loop");
        info!("Rom: {}", self.rom_path);
        loop {
            update();
            if run.state == "stop" {
                break;
            }
        };
    }
}
