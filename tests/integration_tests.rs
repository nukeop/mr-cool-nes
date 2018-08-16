extern crate mr_cool_nes;

#[cfg(test)]
mod integration_tests {
    use mr_cool_nes::core::rom;
    use mr_cool_nes::core::mapper;
    use mr_cool_nes::core::ppu;
    use mr_cool_nes::core::cpu;
    use mr_cool_nes::core::memory;
    use mr_cool_nes::core::memory::Memory;
    use mr_cool_nes::core::nes;
    use mr_cool_nes::core::tools::split_rom;
    use mr_cool_nes::renderer::Renderer;
    use mr_cool_nes::headless_renderer;

    fn setup_emulator(rom_path: &String) -> nes::NES {
        let rom = rom::Rom::load(rom_path).unwrap();
        let mapper = mapper::select_mapper(rom);
        let ppu = ppu::PPU::new();
        let ram = memory::RAM::new();
        let cpu = cpu::CPU::new(ppu, ram, mapper);

        nes::NESBuilder::new()
            .ppu(ppu)
            .cpu(cpu)
            .finalize()
    }

    
    #[test]
    #[ignore]
    fn ram_after_reset() {
        let rom_path = "tests/roms/ram/ram_after_reset.nes".to_owned();
        let mut nes = setup_emulator(&rom_path);

        nes.cpu.reset();

        let mut renderer = Box::new(headless_renderer::HeadlessRenderer::new(&rom_path));
        renderer.start_loop(|| nes.cpu.step());
    }

    #[test]
    fn cpu_instr_implied() {
        let rom_path = "tests/roms/cpu_instructions/01-implied.nes".to_owned();
        let mut nes = setup_emulator(&rom_path);

        split_rom(nes.cpu.mem_map.mapper.get_rom(), &"tests/".to_owned());

        nes.cpu.reset();

        let mut renderer = Box::new(headless_renderer::HeadlessRenderer::new(&rom_path));
        renderer.start_loop(|| {
            nes.cpu.step();
        });
    }
}
