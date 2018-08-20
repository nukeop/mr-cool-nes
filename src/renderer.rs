pub struct RenderingState<'a> {
    pub state: &'a str
}

pub trait Renderer {
    fn start_loop<F>(&mut self, mut update: F, state: &RenderingState) where F: FnMut() -> ();
}
