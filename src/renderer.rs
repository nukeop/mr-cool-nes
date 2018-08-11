pub trait Renderer {
    fn start_loop<F>(&mut self, mut update: F) where F: FnMut() -> () {
        loop {

        };
    }
}
