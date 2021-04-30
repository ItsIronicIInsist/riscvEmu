#![allow(non_snake_case)]

use crate::bus::*;
use crate::regs::*;
use crate::regs::Instruction;

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
			103 | 19 | 3 | 27 => {
				//I format instructions
				let inst = RegImmInst::New(fetchVal);
				InstructionFormat::I(inst)
			}
			51 | 59 => {
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
				//LUI / AUIPC instruction
			}
			111 => {
				//JAL instruction
				let inst = JumpInst::New(fetchVal); 
				InstructionFormat::J(inst)
			}
			_ => { panic!("Instruction format {:b}  not yet supported for code {:b}", (fetchVal & 0x7f), fetchVal); }
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
					Instruction::ADD => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize].wrapping_add(self.regs[inst.rs2 as usize]);
					},
					Instruction::ADDW => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize].wrapping_add(self.regs[inst.rs2 as usize]) as i32 as i64 as u64;
					},
					Instruction::SUB => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize].wrapping_sub(self.regs[inst.rs2 as usize]);
					},
					Instruction::SUB => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize].wrapping_sub(self.regs[inst.rs2 as usize]) as i32 as i64 as u64;
					},
					Instruction::SLL => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] << self.regs[inst.rs2 as usize];
					},
					Instruction::SLLW => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] << self.regs[inst.rs2 as usize] as i32 as i64 as u64;
					},

					//set destination register to 1 is inst.rs1 as usize < rs 2. SLT is signed comp, SLTU is unsigned comp
					Instruction::SLT => {
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize] as i64) < (self.regs[inst.rs2 as usize] as i64)) as u64;
					},
					Instruction::SLTU => {
						self.regs[inst.rd as usize] = (self.regs[inst.rs1 as usize] < self.regs[inst.rs2 as usize]) as u64;
					},
					Instruction::XOR => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] ^ self.regs[inst.rs2 as usize];
					},
					Instruction::OR => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] | self.regs[inst.rs2 as usize];
					},
					Instruction::AND => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] & self.regs[inst.rs2 as usize];
					},
					//when bitshifting, rust does logical shift for unsigned
					//and does arithmetic shift for signed
					Instruction::SRL => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] >> self.regs[inst.rs2 as usize];
					},
					Instruction::SRLW => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] >> self.regs[inst.rs2 as usize] as i32 as i64 as u64;
					},
					Instruction::SRA => {
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize] as i64) >> self.regs[inst.rs2 as usize]) as u64;
					},
					Instruction::SRAW => {
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize] as i64) >> self.regs[inst.rs2 as usize]) as i32 as i64 as u64;
					},
					_ => (),
				}
			},
			InstructionFormat::I(inst) => {
				match inst.instName {
					Instruction::ADDI => {
						//can extend a signed int to an usngiend int and wrapping_add still handles it properly
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize]).wrapping_add(inst.imm as u64))
					},
					Instruction::ADDIW => {
						//can extend a signed int to an usngiend int and wrapping_add still handles it properly
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize]).wrapping_add(inst.imm as u64)) as i32 as i64 as u64;
					},
					Instruction::SLTI => {
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize] as i64) < (inst.imm as i64)) as u64;
					},
					Instruction::SLTIU => { //risc specifies that imm val is first exended to i64, then u64 
						self.regs[inst.rd as usize] = (self.regs[inst.rs1 as usize] < (inst.imm as i64 as u64)) as u64;
					},
					//these are sign extended (the bit oepratiosn)
					Instruction::XORI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] ^ (inst.imm as i64 as u64);
					},
					Instruction::ORI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] | (inst.imm as i64 as u64); //love rust not being able to do bit operation btn u64 and i16. Expands to u64, shoud have all zeros for the other 48 bits
					},																					
					Instruction::ANDI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] & (inst.imm as i64 as u64); //make all the expanded bits 1 to not mangle any data
					},
					Instruction::SRLI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] >> (inst.imm as u16);
					},
					Instruction::SRLI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] >> (inst.imm as u16) as i32 as i64 as u64;
					},
					Instruction::SRAI => {
						self.regs[inst.rd as usize] = unsafe {
						std::mem::transmute::<i64,u64>((self.regs[inst.rs1 as usize] as i64) >> (inst.imm as u16))
					};
					},
					Instruction::SRAIW => {
						self.regs[inst.rd as usize] = ((self.regs[inst.rs1 as usize] as i64) >> (inst.imm as u16)) as i32 as i64 as u64;
					},
					Instruction::SLLI => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] << (inst.imm as u16);
					},
					Instruction::SLLIW => {
						self.regs[inst.rd as usize] = self.regs[inst.rs1 as usize] << (inst.imm as u16) as i32 as i64 as u64;
					},
					Instruction::LB => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 1) as i64 as u64;
					},
					Instruction::LH => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 2) as i64 as u64;
					},
					Instruction::LW => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 4) as i64 as u64;
					},
					Instruction::LBU => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 1);
					},
					Instruction::LHU => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 2);
					},
					Instruction::LWU => {

						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 4);
					},
					Instruction::LD => {
						self.regs[inst.rd as usize] = self.bus.load(self.regs[inst.rs1  as usize].wrapping_add(inst.imm as u64), 8) as i64 as u64;
					},
					Instruction::JALR => {
						self.pc = self.regs[inst.rs1 as usize].wrapping_add(inst.imm as u64).wrapping_sub(4);
					},
					_ => (),
				}
			},
			InstructionFormat::B(inst) => {
				match inst.instName {
					Instruction::BEQ => {
						if self.regs[inst.rs1 as usize] == self.regs[inst.rs2 as usize] {
							println!("pc before {}", self.pc);
							self.pc = self.pc.wrapping_add((inst.imm << 1) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
																							  //to pc each loop
							println!("pc after {}", self.pc + 4);
						}
					},
					Instruction::BNE => {
						if self.regs[inst.rs1 as usize] != self.regs[inst.rs2 as usize] {
							println!("pc before {}", self.pc);
							self.pc = self.pc.wrapping_add((inst.imm << 1) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
							println!("pc after {}", self.pc + 4);
						}
					},
					Instruction::BLT => {
						if (self.regs[inst.rs1 as usize] as i64) < (self.regs[inst.rs2 as usize] as i64) {
							self.pc = self.pc.wrapping_add((inst.imm << 1) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
						}
					},
					Instruction::BGE => {
						if (self.regs[inst.rs1 as usize] as i64) > ( self.regs[inst.rs2 as usize] as i64) {
							self.pc = self.pc.wrapping_add((inst.imm << 1) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
						}
					},
					Instruction::BLTU => {
						if self.regs[inst.rs1 as usize] <  self.regs[inst.rs2 as usize] {
							self.pc = self.pc.wrapping_add((inst.imm << 1) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
						}
					},
					Instruction::BGEU => {
						if self.regs[inst.rs1 as usize] >  self.regs[inst.rs2 as usize] {
							self.pc = self.pc.wrapping_add((inst.imm << 1) as u64).wrapping_sub(4); //the syub is because we ahve an unconditional ass
						}
					},
					_ => (),
				}
			},
			InstructionFormat::S(inst) => {
				match inst.instName {
					Instruction::SB => {
						println!("Calling store with size 1");
						self.bus.store((self.regs[inst.rs1 as usize]).wrapping_add(inst.imm as u64), self.regs[inst.rs2 as usize], 1);
					},
					Instruction::SH => {
						println!("Calling store with size 2");
						self.bus.store((self.regs[inst.rs1 as usize]).wrapping_add(inst.imm as u64), self.regs[inst.rs2 as usize], 2);
					},
					Instruction::SW => {
						println!("Calling store with size 4");
						self.bus.store((self.regs[inst.rs1 as usize]).wrapping_add(inst.imm as u64), self.regs[inst.rs2 as usize], 4);
					},
					Instruction::SD => {
						println!("Calling store with size 8");
						self.bus.store(self.regs[inst.rs1 as usize].wrapping_add(inst.imm as u64), self.regs[inst.rs2 as usize], 8);
					},
					_ => {
						panic!("agony");
					}
				}
			},
			InstructionFormat::U(inst) => {
				match inst.instName {
					Instruction::AUIPC => {
						self.regs[inst.rd as usize] = self.pc.wrapping_add((inst.imm << 12) as i64 as u64);
					},
					Instruction::LUI => {
						//sign extend to i64, the u64 so assignment works
						self.regs[inst.rd as usize] = (inst.imm << 12) as i64 as u64;
	
					},
					_ => (),
				}
			},
			InstructionFormat::J(inst) => {
				match inst.instName {
					Instruction::JAL => {
						self.regs[inst.rd as usize] = self.pc.wrapping_add(4);
						println!("pc before {}", self.pc);
						self.pc = self.pc.wrapping_add((inst.imm << 1) as u64).wrapping_sub(4) as i64 as u64;
						println!("pc after {}", self.pc + 4);
					},
					_ => (),
				}
			},
			_ => panic!("Yet to implement that instruction format"),
		}
	}
}
