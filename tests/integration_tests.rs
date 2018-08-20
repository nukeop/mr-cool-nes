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

    fn setup_emulator(rom_path: &String) -> nes::NES {
        let rom = rom::Rom::load(rom_path).unwrap();
        let mapper = Box::new(mapper::TestMapper::new(rom));
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
        unsafe { renderer.start_loop(|| nes.cpu.step(), &RENDERING_STATE); }
    }

    #[test]
    fn cpu_instr_implied() {
        println!("Running test: 01-implied.nes");
        let rom_path = "tests/roms/cpu_instructions/01-implied.nes".to_owned();
        let mut nes = setup_emulator(&rom_path);

        nes.cpu.reset();
        
        let mut renderer = headless_renderer::HeadlessRenderer::new(&rom_path);
        let mut test_status = 0xFF;
        
        unsafe {
            RENDERING_STATE.state = "test";
            
            renderer.start_loop(|| {

                nes.cpu.step();
                //println!("Run in closure: {}", run);
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
                        assert!(status < 0x3E || status > 0x45);
                        RENDERING_STATE.state = "stop";
                    }
                    
                    test_status = status;
                }
                
            }, &RENDERING_STATE);
        }
    }
}
