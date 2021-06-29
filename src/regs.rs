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
	R4(R3Inst),
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
	FENCE,
	FENCEI,
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
	MUL, //RV64-M instructions
	MULH,
	MULHSU,
	MULHU,
	DIV,
	DIVU,
	REM,
	REMU,
	MULW,
	DIVW,
	DIVUW,
	REMW,
	REMUW,
	FLW, //RV64F (hell)
	FSW,
	FADDS,
	FSUBS,
	FMULS,
	FDIVS,
	FSQRTS,
	FMINS,
	FMAXS,
	FEQS,
	FLTS,
	FLES,
	FNMADDS,
	FNMSUBS,
	FMSUBS,
	FMADDS,
	FCVTSLU,
	FCVTSL,
	FCVTLUS,
	FCVTLS,
	FCVTSWU,
	FCVTSW,
	FCVTWUS,
	FCVTWS,
	FSGNJNS,
	FSGNJS,
	FSGNJXS,
	FMVXW,
	FMVWX,
	FCLASSS,
	FMADDD, //RV64D (Not all isnt yet)
	FMSUBD,
	FNMSUBD,
	FNMADDD,
	CSRRW, //RV64CSR
	CSRRS,
	CSRRC,
	CSRRWI,
	CSRRSI,
	CSRRCI,
	AMOADDW, //RV64A
	AMOADDD,
	AMOANDW,
	AMOANDD,
	AMOORW,
	AMOORD,
	AMOXORW,
	AMOXORD,
	AMOMAXW,
	AMOMAXD,
	AMOMAXUW,
	AMOMAXUD,
	AMOMINW,
	AMOMIND,
	AMOMINUW,
	AMOMINUD,
	AMOSWAPW,
	AMOSWAPD,
	LRW,
	SCW,
	LRD,
	SCD,
}


#[derive(Debug)]
#[derive(Copy,Clone)]
pub struct R3Inst {
	pub rs1: u8,
	pub rs2: u8,
	pub rs3: u8,
	pub rd: u8,
	pub instName: Instruction,
}

impl R3Inst {
	pub fn New(code: u32) -> R3Inst {
		let opcode = code & 0x7f;
		let funct2 = (code >> 25) & 0x3;
		let inst = R3Inst {
			rs1: ((code >> 15) & 0x1f) as u8,
			rs2: ((code >> 20) & 0x1f) as u8,
			rs3: ((code>> 27) & 0x1f) as u8,
			rd: ((code >> 7) & 0x1f) as u8,
			instName: match (funct2, opcode) {
				(0, 67) => Instruction::FMADDS,
				(1, 67) => Instruction::FMADDD,
				(0, 71) => Instruction::FMSUBS,
				(1, 71) => Instruction::FMSUBD,
				(0, 75) => Instruction::FNMSUBS,
				(1, 75) => Instruction::FNMSUBD,
				(0, 79) => Instruction::FNMADDS,
				(1, 79) => Instruction::FNMADDD,
				(_,_) => panic!("Invalid R4 type instruction opcode"),
			}
		};
		inst
	}
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
		let funct3 = (code >> 12) & 0x7;
		let funct7 = (code >> 25) & 0x7f;
		let opcode = code & 0x7f;

		match opcode  {
			51 => {
				if funct7 == 1 {
				//RV32M inst
					let inst = RegRegInst {
						rd: (((code >> 7) & 0x1f) as u8),
						rs1: (((code >> 15) & 0x1f) as u8),
						rs2: (((code >> 20) & 0x1f) as u8),
						instName: match funct3 {
							0 => Instruction::MUL,
							1 => Instruction::MULH,
							2 => Instruction::MULHSU,
							3 => Instruction::MULHU,
							4 => Instruction::DIV,
							5 => Instruction::DIVU,
							6 => Instruction::REM,
							7 => Instruction::REMU,
							_=> panic!("Invalid R-type opcode 32 bit RV32M"),
						}
					};
					inst
				}
				else {
				//RV32I inst
					let inst = RegRegInst {
						rd: (((code >> 7) & 0x1f) as u8),
						rs1: (((code >> 15) & 0x1f) as u8),
						rs2: (((code >> 20) & 0x1f) as u8),
						instName: match (funct7, funct3) {
							(0,0) => Instruction::ADD,
							(32,0) => Instruction::SUB,
							(0,1) => Instruction::SLL,
							(0,2) => Instruction::SLT,
							(0,3) => Instruction::SLTU,
							(0,4) => Instruction::XOR,
							(0,5) => Instruction::SRL,
							(32,5) => Instruction::SRA,
							(0,6) => Instruction::OR,
							(0,7) => Instruction::AND,
							_=> panic!("Invalid R-type opcode 32 bit RV32I"),
						}
					};
					inst
				}
			},
			59 => {
				if funct7 == 1 {
				//RV64M instructions
					let inst = RegRegInst {
						rd: (((code >> 7) & 0x1f) as u8),
						rs1: (((code >> 15) & 0x1f) as u8),
						rs2: (((code >> 20) & 0x1f) as u8),
						instName: match funct3 {
							0 => Instruction::MULW,
							4 => Instruction::DIVW,
							5 => Instruction::DIVUW,
							6 => Instruction::REMW,
							7 => Instruction::REMUW,
							_=> panic!("Invalid R-type 64bit funct RV64I" ),
						}
					};
					inst
				}
				else {
				//RV64I instructions
					let inst = RegRegInst {
						rd: (((code >> 7) & 0x1f) as u8),
						rs1: (((code >> 15) & 0x1f) as u8),
						rs2: (((code >> 20) & 0x1f) as u8),
						instName: match (funct7, funct3) {
							(0,0) => Instruction::ADDW,
							(32,0) => Instruction::SUBW,
							(0,1) => Instruction::SLLW,
							(32,5) => Instruction::SRAW,
							(0,5) => Instruction::SRLW,
							_=> panic!("Invalid R-type 64bit funct RV64I" ),
						}
					};
					inst
				}
			},
			47 => {
				//adjusting for aq/rl bits, so its really more a funct5
				let funct7 = funct7 >> 2;
				//RV32A Instructions
				if funct3 == 2 {
					let inst = RegRegInst {
						rd: (((code >> 7) & 0x1f) as u8),
						rs1: (((code >> 15) & 0x1f) as u8),
						rs2: (((code >> 20) & 0x1f) as u8),
						instName: match (funct7) {
							(0) => Instruction::AMOADDW,
							(1) => Instruction::AMOSWAPW,
							(2) => Instruction::LRW,
							(3) => Instruction::SCW,
							(4) => Instruction::AMOXORW,
							(8) => Instruction::AMOORW,
							(12) => Instruction::AMOANDW,
							(16) => Instruction::AMOMINW,
							(20) => Instruction::AMOMAXW,
							(24) => Instruction::AMOMINUW,
							(28) => Instruction::AMOMAXUW,
							_=> panic!("Invalid R-type 64bit funct RV32a" ),
						}
					};
					inst
				}
				//RV64A Instructions
				else {
					let inst = RegRegInst {
						rd: (((code >> 7) & 0x1f) as u8),
						rs1: (((code >> 15) & 0x1f) as u8),
						rs2: (((code >> 20) & 0x1f) as u8),
						instName: match (funct7) {
							(0) => Instruction::AMOADDD,
							(1) => Instruction::AMOSWAPD,
							(2) => Instruction::LRD,
							(3) => Instruction::SCD,
							(4) => Instruction::AMOXORD,
							(8) => Instruction::AMOORD,
							(12) => Instruction::AMOANDD,
							(16) => Instruction::AMOMIND,
							(20) => Instruction::AMOMAXD,
							(24) => Instruction::AMOMINUD,
							(28) => Instruction::AMOMAXUD,
							_=> panic!("Invalid R-type 64bit funct RV64A" ),
						}
					};
					inst
				}
			},
			_ => {panic!("Invalid opcode for R type instruction");},
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
			15 => { //technically need to differentiate btn FENCE and FENCEI. But. I dont implement either, as emulator is singe threaded
				let inst = RegImmInst {
					rd: (((code >> 7) & 0x1f) as u8),
					instName: Instruction::FENCE,
					rs1: (((code >> 15) & 0x1f) as u8),
					imm: (((code as i32) >> 20) as i16),
				};
				inst
			}
			115 => { //CSR Instructions
				let inst = RegImmInst {
					rd: (((code >> 7) & 0x1f) as u8),
					instName: match ((code >> 12) & 0x7) {
						1 => Instruction::CSRRW,
						2 => Instruction::CSRRS,
						3 => Instruction::CSRRC,
						5 => Instruction::CSRRWI,
						6 => Instruction::CSRRSI,
						7 => Instruction::CSRRCI,
						_ => panic!("Invalid immeidate funct val"),
					},
					rs1: (((code >> 15) & 0x1f) as u8),
					imm: (((code as u32) >> 20) as i16), //this is CSR offset
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

//user level csrs
pub const USTATUS: usize = 0x0;
pub const UIE: usize = 0x4;
pub const UTVEC: usize = 0x5; 
pub const USCRATCH: usize = 0x40;
pub const UEPC: usize = 0x41;
pub const UCAUSE: usize = 0x42;
pub const UBADADDR: usize = 0x43; 
pub const UIP: usize = 0x44;
pub const CYCLE: usize = 0xc00;
pub const TIME: usize = 0xc01;
pub const INSTRET: usize = 0xc02;

// Machine-level CSRs.
// Hardware thread ID.
pub const MHARTID: usize = 0xf14;
// Machine status register.
pub const MSTATUS: usize = 0x300;
//ISA and extensions
pub const MIFA: usize = 0x301;
// Machine exception delefation register.
pub const MEDELEG: usize = 0x302;
// Machine interrupt delefation register.
pub const MIDELEG: usize = 0x303;
// Machine interrupt-enable register.
pub const MIE: usize = 0x304;
// Machine trap-handler base address.
pub const MTVEC: usize = 0x305;
// Machine counter enable.
pub const MCOUNTEREN: usize = 0x306;
// Scratch register for machine trap handlers.
pub const MSCRATCH: usize = 0x340;
// Machine exception program counter.
pub const MEPC: usize = 0x341;
// Machine trap cause.
pub const MCAUSE: usize = 0x342;
// Machine bad address or instruction.
pub const MTBADADDR: usize = 0x343;
// Machine interrupt pending.
pub const MIP: usize = 0x344;

// Supervisor-level CSRs.
// Supervisor status register.
pub const SSTATUS: usize = 0x100;
//exception delegaiton register
pub const SEDELEG: usize = 0x102;
//interrupt delegaiton register
pub const SIDELEG: usize = 0x103;
// Supervisor interrupt-enable register.
pub const SIE: usize = 0x104;
// Supervisor trap handler base address.
pub const STVEC: usize = 0x105;
// Scratch register for supervisor trap handlers.
pub const SSCRATCH: usize = 0x140;
// Supervisor exception program counter.
pub const SEPC: usize = 0x141;
// Supervisor trap cause.
pub const SCAUSE: usize = 0x142;
// Supervisor bad address or instruction.
pub const SBADADDR: usize = 0x143;
// Supervisor interrupt pending.
pub const SIP: usize = 0x144;
// Supervisor address translation and protection.
pub const SATP: usize = 0x180;

