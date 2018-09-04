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
    use mr_cool_nes::renderer::{Renderer, RenderingState};
    use mr_cool_nes::headless_renderer;

    static mut RENDERING_STATE: RenderingState = RenderingState{state: "test"};

    fn setup_ppu() -> ppu::PPU {
        ppu::PPU::new()
    }
    
    fn setup_emulator<'a>(rom_path: &String, ppu: &'a mut ppu::PPU) -> nes::NES<'a> {
        let rom = rom::Rom::load(rom_path).unwrap();
        let mapper = Box::new(mapper::TestMapper::new(rom));
        let ram = memory::RAM::new();
        let cpu = cpu::CPU::new(ppu, ram, mapper);

        nes::NESBuilder::new()
            .cpu(cpu)
            .finalize()
    }

    fn run_integration_test(rom_name: &str, rom_path: &str, error_lower: u8, error_upper: u8) {
        println!("\nRunning test: {}", rom_name);
        let my_rom_path = &rom_path.to_owned();
        let mut ppu = setup_ppu();
        let mut nes = setup_emulator(my_rom_path, &mut ppu);

        nes.cpu.reset();
        
        let mut renderer = headless_renderer::HeadlessRenderer::new(&rom_path.to_owned());
        let mut test_status = 0xFF;
        
        unsafe {
            RENDERING_STATE.state = "test";
            
            renderer.start_loop(|r: &mut headless_renderer::HeadlessRenderer| {

                nes.cpu.step();
                
                let status = nes.cpu.load_byte(0x6000);
                if (test_status != status) {
                    println!("Test status changed to {:X}.", status);

                    if (status == 0x0) {
                        println!("Test initializing.\n");
                    }
                    
                    if (status == 0x80) {
                        println!("Test started running.\n");
                    }

                    if (test_status == 0x80) {
                        println!("Test finished running. Result code: {:X}", status);

                        // Error codes: https://github.com/christopherpow/nes-test-roms/blob/master/other/nestest.txt#L167
                        assert!(status < error_lower || status > error_upper);
                        RENDERING_STATE.state = "stop";
                    }
                    
                    test_status = status;
                }
            }, &RENDERING_STATE);
        }
    }

    
    #[test]
    #[ignore]
    fn ram_after_reset() {
        let rom_path = &"tests/roms/ram/ram_after_reset.nes".to_owned();
        let mut ppu = setup_ppu();
        let mut nes = setup_emulator(&rom_path, &mut ppu);

        nes.cpu.reset();

        let mut renderer = Box::new(headless_renderer::HeadlessRenderer::new(&rom_path));
        unsafe { renderer.start_loop(|r: &mut headless_renderer::HeadlessRenderer| nes.cpu.step(), &RENDERING_STATE); }
    }

    #[test]    
    fn cpu_instr_implied() {
        run_integration_test(
            "01-implied.nes",
            "tests/roms/cpu_instructions/01-implied.nes",
            0x3E,
            0x45
        );
    }

    #[test]
    #[ignore]
    fn cpu_instr_immediate() {
        run_integration_test(
            "02-immediate.nes",
            "tests/roms/cpu_instructions/02-immediate.nes",
            0x18,
            0x75
        );
    }

    #[test]
    fn cpu_instr_zeropage() {
        run_integration_test(
            "03-zero_page.nes",
            "tests/roms/cpu_instructions/02-immediate.nes",
            0x18,
            0x75
        );
    }

}
