use clap::App;

pub fn read_cl_args() {
    App::new("mr-cool-nes")
        .version("0.1.0")
        .about("nes emulator")
        .author("nukeop <nukeop@gumblert.tech>")
        .get_matches(); 
}
