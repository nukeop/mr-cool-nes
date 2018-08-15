extern crate mr_cool_nes;

#[cfg(test)]
mod integration_tests {
    use mr_cool_nes::core::rom;
    use mr_cool_nes::core::mapper;
    use mr_cool_nes::core::ppu;
    use mr_cool_nes::core::cpu;
    use mr_cool_nes::core::memory;
    use mr_cool_nes::core::nes;
    use mr_cool_nes::renderer::Renderer;
    use mr_cool_nes::headless_renderer;

    
    #[test]
    #[ignore]
    fn ram_after_reset() {
        let rom_path = "tests/roms/ram_after_reset.nes".to_owned();
        let rom = rom::Rom::load(&rom_path).unwrap();
        let mapper = mapper::select_mapper(rom);
        let ppu = ppu::PPU::new();
        let ram = memory::RAM::new();
        let cpu = cpu::CPU::new(ppu, ram, mapper);

        let mut nes = nes::NESBuilder::new()
            .ppu(ppu)
            .cpu(cpu)
            .finalize();

        nes.cpu.reset();

        let mut renderer = Box::new(headless_renderer::HeadlessRenderer::new(&rom_path));
        renderer.start_loop(|| nes.cpu.step());
    }
}
