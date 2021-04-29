

use crate::bus::*;
use crate::regs::*;


//Struct for Cpu
pub struct Cpu {
	pub regs: [u64;32], //registers. RISC-V has 32 of them. 64 bitwide of course, bc 64bit arch
	pub pc: u64, //program counter
	pub bus: Bus,
}


impl Cpu {
	//Initialising stuff for a Cpu
	pub fn new(code: Vec<u8>) -> Cpu  {
		let mut cpu = Cpu {
			regs: [0;32], //set all regs to 0. Doesnt really matter except
						//that r0 is a special register - the zero register. Must always be = 0
			pc: 0,
			bus: Bus::New(code),
		};
		cpu.regs[2] = 1024*1024; //r2 is stack register. Stack grows downwards,
								//so r2 must be a non zero value. THis val was chosen at random
		return cpu;
	}
	
	//emulating fetch aspect of execution cycles
	pub fn fetch(&self) -> u32 {
		let pc = self.pc as usize; //has to be usize for indexing
		let instruction = self.bus.load(self.pc, 4) as u32;
					  
		return instruction;
	}

	pub fn decode(fetchVal: u32) -> InstructionFormat {
		//consider reformatting this
		//such that rather than manually creating the structs
		//implementing a ::New(u32) method for each struct
		//and calling that here
		let formatted_instruction = match (fetchVal & 0x7f) {
			19 | 3 => {
				//I format instructions
				let inst = RegImmInst::New(fetchVal);
				InstructionFormat::I(inst)
			}
			51 => {
				//R format instructions
				let inst = RegRegInst::New(fetchVal);  
				InstructionFormat::R(inst)
			}
			35  => {
				//S format instructions
				let inst = StoreInst::New(fetchVal);
				InstructionFormat::S(inst)
			}
			99 => {
				//B format instructions
				let inst = BranchInst::New(fetchVal);
				InstructionFormat::B(inst)
			}
			55 | 23 => {
				let inst = UpperImmInst::New(fetchVal);
				InstructionFormat::U(inst)
				//LUI instruction
			}
			103 | 111 => {
				//JAL instruction
				let inst = JumpInst::New(fetchVal); 
				InstructionFormat::J(inst)
			}
			_ => { panic!("Instruction format not yet supported"); }
		};
		formatted_instruction
	}

	//slightly weird name bc otherwise heaps of vars would be called 'inst'
	//so instruction toExecute
	pub fn execute(&mut self, toExecute: InstructionFormat) {
		//find what instruction format it is
		match toExecute {
			InstructionFormat::R(inst) => {
				match inst.instName {
					ADD => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize].wrapping_add(self.regs[inst.rs2 as usize]);
					},
					SUB => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize].wrapping_sub(self.regs[inst.rs2 as usize]);
					},
					SLL => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] << self.regs[inst.rs2 as usize];
					},
					//set destination register to 1 is inst.rs1 as usize < rs 2. SLT is signed comp, SLTU is unsigned comp
					SLT => {
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize] as i64) < (self.regs[inst.rs2 as usize] as i64)) as u64;
					},
					SLTU => {
						self.regs[inst.rd as usize] = (self.regs[inst.rs1 as usize] < self.regs[inst.rs2 as usize]) as u64;
					},
					XOR => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] ^ self.regs[inst.rs2 as usize];
					},
					OR => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] | self.regs[inst.rs2 as usize];
					},
					AND => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] & self.regs[inst.rs2 as usize];
					},
					//when bitshifting, rust does logical shift for unsigned
					//and does arithmetic shift for signed
					SRL => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] >> self.regs[inst.rs2 as usize];
					},
					SRA => {
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize] as i64) >> self.regs[inst.rs2 as usize]) as u64;
					},
					_ => (),
				}
			},
			InstructionFormat::I(inst) => {
				match inst.instName {
					ADDI => {
						//can extend a signed int to an usngiend int and wrapping_add still handles it properly
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize]).wrapping_add(inst.imm as u64))
					},
					SLTI => {
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize] as i64) < (inst.imm as i64)) as u64;
					},
					SLTIU => { //risc specifies that imm val is first exended to i64, then u64 
						self.regs[inst.rd as usize] = (self.regs[inst.rs1 as usize] < (inst.imm as i64 as u64)) as u64;
					},
					//these are sign extended (the bit oepratiosn)
					XORI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] ^ (inst.imm as i64 as u64);
					},
					ORI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] | (inst.imm as i64 as u64); //love rust not being able to do bit operation btn u64 and i16. Expands to u64, shoud have all zeros for the other 48 bits
					},																					
					ANDI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] & (inst.imm as i64 as u64); //make all the expanded bits 1 to not mangle any data
					},
					SRLI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] >> (inst.imm as u16);
					},
					SRAI => {
						self.regs[inst.rd as usize] = unsafe {
						std::mem::transmute::<i64,u64>((self.regs[inst.rs1 as usize] as i64) >> (inst.imm as u16))
					};
					},
					SLLI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] << (inst.imm as u16);
					},
					LB => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 1) as i64 as u64;
					},
					LH => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 2) as i64 as u64;
					},
					LW => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 4) as i64 as u64;
					},
					LBU => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 1);
					},
					LHU => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 2);
					},
					_ => (),
				}
			},
			InstructionFormat::B(inst) => {
				match inst.instName {
					BEQ => {
						if self.regs[inst.rs1 as usize] == self.regs[inst.rs2 as usize] {
							self.pc = self.pc.wrapping_add((inst.imm * 4) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
																							  //to pc each loop
						}
					},
					BNE => {
						if self.regs[inst.rs1 as usize] != self.regs[inst.rs2 as usize] {
							self.pc = self.pc.wrapping_add((inst.imm * 4) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
						}
					},
					BLT => {
						if (self.regs[inst.rs1 as usize] as i64) < (self.regs[inst.rs2 as usize] as i64) {
							self.pc = self.pc.wrapping_add((inst.imm * 4) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
						}
					},
					BGE => {
						if (self.regs[inst.rs1 as usize] as i64) > ( self.regs[inst.rs2 as usize] as i64) {
							self.pc = self.pc.wrapping_add((inst.imm * 4) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
						}
					},
					BLTU => {
						if self.regs[inst.rs1 as usize] <  self.regs[inst.rs2 as usize] {
							self.pc = self.pc.wrapping_add((inst.imm * 4) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
						}
					},
					BGEU => {
						if self.regs[inst.rs1 as usize] >  self.regs[inst.rs2 as usize] {
							self.pc = self.pc.wrapping_add((inst.imm * 4) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
						}
					},
					_ => (),
				}
			},
			InstructionFormat::S(inst) => {
				match inst.instName {
					SB => {
						self.bus.store((self.regs[inst.rs1 as usize]).wrapping_add(inst.imm as u64), self.regs[inst.rs2 as usize], 1);
					},
					SH => {
						self.bus.store((self.regs[inst.rs1 as usize]).wrapping_add(inst.imm as u64), self.regs[inst.rs2 as usize], 2);
					},
					SW => {
						self.bus.store((self.regs[inst.rs1 as usize]).wrapping_add(inst.imm as u64), self.regs[inst.rs2 as usize], 4);
					},
				}
			},
			InstructionFormat::U(inst) => {
				match inst.instName {
					AUIPC => {
						self.regs[inst.rd as usize] = self.pc.wrapping_add((inst.imm << 12) as i64 as u64);
					},
					LUI => {
						//sign extend to i64, the u64 so assignment works
						self.regs[inst.rd as usize] = (inst.imm << 12) as i64 as u64;
	
					},
					_ => (),
				}
			},
			InstructionFormat::J(inst) => {
				match inst.instName {
					JAL => {
						self.regs[inst.rd as usize] = self.pc.wrapping_add(4);
						self.pc = self.pc.wrapping_add(((inst.imm << 12) * 4)as i64 as u64);
					},
					JALR => {
						self.regs[inst.rd as usize] = self.pc.wrapping_add(4);
						self.pc = self.pc.wrapping_add((inst.imm << 12) as i64 as u64);

					},
					_ => (),
				}
			},
			_ => panic!("Yet to implement that instruction format"),
		}
	}

}
