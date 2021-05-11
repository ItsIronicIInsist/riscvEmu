//apparently there are many forms of instruciton formats
//but to start Ill just support the main 6
//will be slightly memory inefficient (opcode has 7 bits, stored in a u8)
//but its so minimal that it doesnt matter at all

//different instruction formats for RISCV
//opcode is 7bits for each
//any register reference is 5bits
//functN is N bits wide
#[derive(Debug)]
#[derive(Copy,Clone)]
pub enum InstructionFormat { 
	R(RegRegInst), 
	I(RegImmInst),
	S(StoreInst),
	B(BranchInst),
	U(UpperImmInst),
	J(JumpInst),
}
//Consider creating enums for each instruciton format
//such that I dont have anf funct or opcode members
//and just the instruction

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Instruction {
	JAL, //Jump instructions
	JALR,
	AUIPC, //Upper Immediate instrucitons
	LUI,
	BEQ, //branch instrucitons
	BNE,
	BLT,
	BGE,
	BLTU,
	BGEU,
	SB, //store instructions
	SH,
	SW,
	SD, //64 bit store
	ADD, //R-type instructions
	SUB,
	SLL,
	SLT,
	SLTU,
	XOR,
	SRL,
	SRA,
	OR,
	AND,
	ADDW, //RV64I instructions
	SUBW,
	SLLW,
	SRLW,
	SRAW,
	ADDI, //Immediate instructions
	SLTI,
	SLTIU,
	XORI,
	ORI,
	ANDI,
	SRLI,
	SLLI,
	SRAI,
	ADDIW, //R64I instructions
	SLLIW,
	SRLIW,
	SRAIW,
	LB, //load instructions
	LH,
	LW,
	LBU,
	LHU,
	LWU, //64bit load instructions
	LD,

}



//Instruction format for instruction w/ two source registers (e.g. add r0, r1, r2 adds r1 and r2, stores in r0)
// | funct7 | rs2 | rs1 | funct3 | rd | opcode
//unsure: Do I need the opcose in these structs?
#[derive(Debug)]
#[derive(Copy,Clone)]
pub struct RegRegInst {	
	pub rs1: u8, //source register 1
	pub rs2: u8, //source register 2
	pub rd: u8,	 //destination register
	pub instName: Instruction
}

impl RegRegInst {
	pub fn New(code:u32) -> RegRegInst {
		let funct = ((code >> 12) & 0x7) | (((code >> 25) & 0x7f) << 3);
		let opcode = code & 0x7f;	
		if opcode == 41 {
			let inst = RegRegInst {
				rd: (((code >> 7) & 0x1f) as u8),
				rs1: (((code >> 15) & 0x1f) as u8),
				rs2: (((code >> 20) & 0x1f) as u8),
				instName: match funct {
					0 => Instruction::ADD,
					256 => Instruction::SUB,
					1 => Instruction::SLL,
					2 => Instruction::SLT,
					3 => Instruction::SLTU,
					4 => Instruction::XOR,
					5 => Instruction::SRL,
					261 => Instruction::SRA,
					6 => Instruction::OR,
					7 => Instruction::AND,
					_=> panic!("Invalid R-type opcode 32 bit"),
				}
			};
			inst
		}
		else {
			let inst = RegRegInst {
				rd: (((code >> 7) & 0x1f) as u8),
				rs1: (((code >> 15) & 0x1f) as u8),
				rs2: (((code >> 20) & 0x1f) as u8),
				instName: match funct {
					0 => Instruction::ADDW,
					256 => Instruction::SUBW,
					1 => Instruction::SLLW,
					261 => Instruction::SRAW,
					5 => Instruction::SRLW,
					_=> panic!("Invalid R-type 64bit funct  {} for  inst {:b}",funct, code ),
				}
			};
			inst
		}
	}
}


//Instruciton format for one source reg and an imm val. 
//imm val is 12 bits, signed/
//notable exception is the shift intruction
// | imm | rs1 | funct3 | rd | opcode |
#[derive(Debug)]
#[derive(Copy,Clone)]
pub struct RegImmInst {
	pub rs1: u8, //source register
	pub rd: u8, //dest register
	pub instName: Instruction, //val to specify what instruction
	pub imm: i16, //immediate value. Signed.
}

impl RegImmInst {
	pub fn New(code:u32) -> RegImmInst {
		let opcode = (code & 0x7f);
		//immedate instructions
		//havent implemented shift instructions yet - SHAMT will equal the imm val
		match opcode {
			19 => {
				let mut inst = RegImmInst {
					rd: (((code >> 7) & 0x1f) as u8),
					instName: match ((code >> 12) & 0x7) {
						0 => Instruction::ADDI,
						1 => Instruction::SLLI,
						2 => Instruction::SLTI,
						3 => Instruction::SLTIU,
						4 => Instruction::XORI,
						//stores both SRLI and SRAI
						5 => {
							match ((code >> 30)) {
								1 =>  Instruction::SRAI,
								0 =>  Instruction::SRLI,
								_ => panic!("Invalid sshift instruction"),
							}
						},
						6 => Instruction::ORI,
						7 => Instruction::ANDI,
						_ => panic!("Invalid immediate funct val"),
					},
					rs1: (((code >> 15) & 0x1f) as u8),
					imm: (((code as i32) >> 20) as i16),
				};
				if inst.instName == Instruction::SRAI {
					inst.imm = inst.imm & 0x1f;
				}
				inst
			},
			27 => {
				let mut inst = RegImmInst {
					rd: (((code >> 7) & 0x1f) as u8),
					instName: match ((code >> 12) & 0x7) {
						0 => Instruction::ADDIW,
						1 => Instruction::SLLIW,
						//stores both SRLI and SRAI
						5 => {
							match ((code >> 30)) {
								1 =>  Instruction::SRAIW,
								0 =>  Instruction::SRLIW,
								_ => panic!("Invalid sshift instruction"),
							}
						},
						_ => panic!("Invalid immediate funct val {} for {:b}",(code >> 12) & 0x7 ,code),
					},
					rs1: (((code >> 15) & 0x1f) as u8),
					imm: (((code as i32) >> 20) as i16),
				};
				if inst.instName == Instruction::SRAIW {
					inst.imm = inst.imm & 0x1f;
				}
				inst
			},
			3=> {
		//Load instructions
				let inst = RegImmInst {
					rd: (((code >> 7) & 0x1f) as u8),
					instName: match ((code >> 12) & 0x7) {
						0 => Instruction::LB,
						1 => Instruction::LH,
						2 => Instruction::LW,
						3 => Instruction::LD,
						4 => Instruction::LBU,
						5 => Instruction::LHU,
						6 => Instruction::LWU,
						_ => panic!("Invalid immeidate funct val"),
					},
					rs1: (((code >> 15) & 0x1f) as u8),
					imm: (((code as i32) >> 20) as i16),
				};
				inst
			},
			103 => {
				let inst = RegImmInst {
					rd: (((code >> 7) & 0x1f) as u8),
					instName: Instruction::JALR,
					rs1: (((code >> 15) & 0x1f) as u8),
					imm: (((code as i32) >> 20) as i16),
				};
				inst
			}
			_ => panic!("invalid opcode form immediate instruction {:b}", opcode),
		}

	}
}


//Instruction format for store instructions.
// | imm [11:5] | rs2 | rs1 | funct3 | imm[4:0] | opcode
//No destination register. Stores into memory.
// Memory at [rs1] + imm = [rs2] (brackets are dereferencing
//Loads are n the I format
#[derive(Debug)]
#[derive(Copy,Clone)]
pub struct StoreInst {
	pub rs1: u8, // base (base mem of where stuff is being stored. Imm is added to it)
	pub rs2: u8, // src (val being stored)
	pub instName: Instruction, //vall to specify instruction
	pub imm: i16, //immediate value. In the format the imm value is split into different chunks
			  //but they are combined together so just the one member
}

impl StoreInst {
	pub fn New(code: u32) -> StoreInst {
		let inst = StoreInst {	
			instName: match ((code >> 12) & 0x7) {
				0 => Instruction::SB,
				1 => Instruction::SH,
				2 => Instruction::SW,
				3 => Instruction::SD,
				_ => panic!("Invalid funct3 for store instName"), 
			},
			rs1: (((code >> 15) & 0x1f) as u8),
			rs2: (((code >> 20) & 0x1f) as u8),
			imm: {
				(((code >> 7) & 0x1f) as i16) | ((((code as i32) >> 25) << 5) as i16)
			}
		};
		//println!("imm[0:4] = {:b}", ((code >> 7) & 0x1f) as i16);

		//println!("imm[5:11] = {:b}", ((code as i32) >> 20) as i16);
		inst
	}
}

//Instruciton format for branch instructions
// | imm[12 | 10: 5] | rs2 | rs1 | funct3 | imm[4:1 | 11] | opcode
//branches are defined as relative to program counter (loops and such are genrally small so imm val offers an offset of +- 2^13 bytes), often large enough
// the reason its 2^13, not 2^11 (12 bits, 1 for signedness) is because PC is always 4byte aligned
//so jump is pc + 4*imm
//needs two source registers for comparing. Like jeq
#[derive(Debug)]
#[derive(Copy,Clone)]
pub struct BranchInst {
	pub rs1: u8, //source register 1
	pub rs2: u8, //source register 2
	pub imm: i16, //branch offset
	pub instName: Instruction,
}


//implementing new method for each struct
impl BranchInst {
	pub fn New(code: u32) -> BranchInst {
		let inst = BranchInst {
			instName: match ((code >> 12) & 0x7) {
				0 => Instruction::BEQ,
				1 => Instruction::BNE,
				4 => Instruction::BLT,
				5 => Instruction::BGE,
				6 => Instruction::BLTU,
				7 => Instruction::BGEU,
				_ => panic!("Invalid funct3 for branch instruction"),
			},
			rs1: (((code >> 15) & 0x1f) as u8),
			rs2: (((code >> 20) & 0x1f) as u8),
			imm: {
				let bit11 = ((code >> 7) & 0x1) as i16;
				let bit1to4 = ((code >> 8) & 0xf) as i16;
				let bit5to10 = ((code >> 25) & 0x3f) as i16;	
				let bit12 = ((code >> 31) & 0x1) as i16; //signedness
				let mut total : i16 = bit1to4;
				total = total | (bit5to10 << 4);
				total = total | (bit11 << 10);
				total = total | (bit12 << 15); //this is shifted 15 because its the signedness bit
				if (bit12 == 1) {
					total = total | (0b1111 << 11); //have to sign extend it
				}
				total								//from 12 to 16 bits
												//which means setting these all to 1
			}
		};
		inst
	}
}


//we want 32bit immediate values, but so far we only have 12 bits
//this is 'Upper Immediate' instructions - has a 20bit immediate value
// | imm | rd | opcode |
//Together, you can have full 32bit range
//LUI x0, <20bitval>, ADDI x0, <12bitval>, x0
//Note: LUI clears the last 10bits
//Theres no funct3, just two instructions LUI (load upper immediate) and AUIPC (add upper immediat to pc)
// AUIPC does not change pc - it just stores the result in the destination register. used for relative addressing
#[derive(Debug)]
#[derive(Copy,Clone)]
pub struct UpperImmInst {
	pub rd: u8, //destination register
	pub imm: i32, //the immediate vaue. 20bits
	pub instName: Instruction, //opcode is needed bc it acts as funct value - diffrentiating btn instructions
}

impl UpperImmInst {
	pub fn New(code: u32) -> UpperImmInst {
		let inst = UpperImmInst {
			instName: match code & 0x7f {
				55 => Instruction::LUI,
				23 => Instruction::AUIPC,
				_ => panic!("Invalid Jump instruction opcode"),
			},
			rd: (((code >> 7) & 0x1f) as u8),
			imm: ((code as i32) >> 12),
		};
		inst
	}
}



//U format
// | imm[20|10:1|11|19:12] | rd | opcode | 
// absolute jump instruction. Still only 20 bits, 19 if you disregard sign
//dont do any pc + 4*imm stuff (for some reason???)
//imm value is highly mangled. Look nto something called multiplexer
#[derive(Debug)]
#[derive(Copy,Clone)]
pub struct JumpInst {
	pub rd: u8, // PC+4 is stored in here
	pub imm: i32, //the location to jump to. Dont to the pc + 4*imm, just pc + imm
	pub instName: Instruction, //opcode is needed bc it acts as funct value - diffrentiating btn instructions
}


impl JumpInst {
	pub fn New(code: u32) -> JumpInst {
		let inst = JumpInst {
			instName: Instruction::JAL, 
			rd: (((code >> 7) & 0x1f) as u8),
			imm: {
			let bit12to19 = ((code >> 12) & 0xff) as i32;
				let bit11 = ((code >> 20) & 0x1) as i32;
				let bit1to10 = ((code >>21)& 0x3ff) as i32;
				let bit20 = ((code >> 31) & 0x1) as i32; //signedness bit
				let mut total : i32 = bit1to10;
				total = total | (bit11 << 10);
				total = total | (bit12to19 << 11);
				total = total | (bit20 << 31); //shifted by 31 because its signedness bit
				if (bit20 == 1) {
					total = total | (0b111111111111 << 19);
				}
				total
			},
			};
		inst
	}
}





