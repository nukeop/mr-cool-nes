pub trait Mapper {
    fn load_pgr_rom(&self);
    fn load_chr_rom(&self);
    fn load_battery_ram(&self);

    fn load_rom(&self) {
        self.load_pgr_rom();
        self.load_chr_rom();
        self.load_battery_ram();
    }
}
