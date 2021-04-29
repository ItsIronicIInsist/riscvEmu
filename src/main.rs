//The Roughest of first drafts
//Emulator for risc v architecture. 
//Probably aiming for full support for R64G - generic 64 bit instruction set
//

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

//module defining all the register structs
mod regs;
mod cpu;
mod bus;
mod dram;

use regs::Instruction;
use cpu::Cpu;

fn main() -> io::Result<()> {
	//get arguments
	//for now its just a file name
	//if interaction with terminal gets more complicated
	//should consider using something like clap
	
	let args: Vec<String>  = env::args().collect();

	if args.len() != 2 {
		panic!("Correct argument usage: riscvEmu <file>");
	}
	let mut exeFile = File::open(&args[1])?;
	
	let mut code = Vec::new();
	exeFile.read_to_end(&mut code)?;
	
	let mut cpu = Cpu::new(code);
	//emulation loop
	//could just be a while(true) i think
	
	loop {
		//fetch instruciton
		let instruction = cpu.fetch();
				
		//update pc counter
		cpu.pc += 4;
		
		//TO DO: decode, the start of execute
		let instruction = Cpu::decode(instruction);
		println!("{:?}", instruction);

		cpu.execute(instruction);
	
		for i in (0..32) {
			if (cpu.regs[i as usize] != 0) {
				println!("register {} has val {}", i, cpu.regs[i as usize]);
			}
		}

	} 
	Ok(())

}



//Notes: 
//unless otherwise specified, instructions are 32bits wide
//when an integer overflow occurs, RISC-V just wraps it around
//RISC-V supprots both little and big endian but emulator will just do little endian
//Need to build a risc-v toolchain
//
//Given that instructions are 32 bit long, any branches/jumps to a target addr
//that isnt a multiple of 4bytes should generate an exception - instruction-address-misaligned exception
//
//instructions come in four formats - R-I-S-U
//for immediate values they are almots always signed, and the 31st bits hold the sign bit
//

