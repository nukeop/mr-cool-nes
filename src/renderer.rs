use core::ppu::PPU;

pub struct RenderingState<'a> {
    pub state: &'a str
}

pub trait Renderer<R> {
    fn start_loop<F>(&mut self, mut update: F, state: &RenderingState) where F: FnMut(&mut R) {}

    fn render_screen(&mut self, ppu: &mut PPU) {}
}
