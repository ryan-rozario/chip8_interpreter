struct chip8{
    opcode:u16,
    memory:[u8,4096],
    v:[u8,16],
    i:u16,
    pc:u16,
    gfx:[u8,64*32],
    delay_timer:u8,
    sound_timer:u8,
    stack:[u16,16],
    sp:u16,
    key:[u8,16]
}

impl chip8{
    fn getInstance(&self){
        //clean the memory and set registers to zero
        self.opcode =0;
        self.i=0;
        self.sp=0;

        //program counter starts at 0x200
        self.pc = 0x200;

        // Clear display
        for elem in self.gfx.iter_mut() { *elem = 0; }	
        // Clear stack
        for elem in self.stack.iter_mut() { *elem = 0; }	
        // Clear registers V0-VF
        for elem in self.v.iter_mut() { *elem = 0; }	
        // Clear memory
        for elem in self.memory.iter_mut() { *elem = 0; }	

        for font_loc in 0..80 {
            memory[font_loc]=chip8_fontset[i];
        }

        //reset timer
        self.delay_timer=0;
        self.sound_timer=0;

        //load program
        
    }
    fn emulate_cycle(){
        
    }
}
