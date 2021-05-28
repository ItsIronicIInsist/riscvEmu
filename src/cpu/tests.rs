use super::*;

#[test]
fn ADDI_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::ADDI,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	inst.imm = -1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0);

	
	inst.imm = -1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1] as i64, -1);
}

#[test]
fn ADDIW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::ADDIW,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	inst.imm = -1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0);

	
	inst.imm = -1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1] as i64, -1);

	cpu.regs[1] = 0xffffffff;
	inst.imm = 1;

	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1] as i64, 0);
}

#[test]
fn ADD_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rs2: 2,
		rd: 1,
		instName: Instruction::ADD,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[1] = 0;
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[2] = u64::MAX - 1;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], u64::MAX);

	
	cpu.regs[2] = 1;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0);
}


#[test]
fn ADDW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::ADDW,
		rs2: 2,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[2] = u64::MAX;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0);


	cpu.regs[1] = 0xffffffff;
	cpu.regs[2] = 1;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0);
}

#[test]
fn SUB_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rs2: 2,
		rd: 1,
		instName: Instruction::SUB,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[1] = 0;
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],u64::MAX);

	cpu.regs[2] = 1;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], u64::MAX - 1);
}


#[test]
fn SUBW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SUBW,
		rs2: 2,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],u64::MAX);

	cpu.regs[2] = u64::MAX;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0);


	cpu.regs[1] = 0xffffffff;
	cpu.regs[2] = 1;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], u64::MAX -1);
}


#[test]
fn SLTI_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SLTI,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	inst.imm = -1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0);
}

#[test]
fn SLT_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SLT,
		rs2: 2,
	};
	cpu.regs[1] = 0;
	cpu.regs[2] = u64::MAX;
	let mut instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],0);

	cpu.regs[1] = 1;
	cpu.regs[2] = 2;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 1);
}

#[test]
fn SLTIU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SLTIU,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	inst.imm = -1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 1);
}

#[test]
fn SLTU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SLTU,
		rs2: 2,
	};
	cpu.regs[1] = 0;
	cpu.regs[2] = u64::MAX;
	let mut instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[1] = 1;
	cpu.regs[2] = 2;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 1);
}

#[test]
#[allow(overflowing_literals)]
fn ANDI_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::ANDI,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],0);

	inst.imm = 0b1111101010101010;
	cpu.regs[1] = 0xffff;

	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0b1111101010101010);
}

#[test]
fn AND_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::AND,
		rs2: 2,
	};
	cpu.regs[1] = 1;
	cpu.regs[2] = 0;
	let mut instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],0);

	cpu.regs[2] = 0b1111101010101010;
	cpu.regs[1] = 0xffff;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0b1111101010101010);
}

#[test]
fn ORI_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::ORI,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	inst.imm = 0b0000111100001111;
	cpu.regs[1] = 0b0000111111111111;

	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0b0000111111111111);
}


#[test]
fn OR_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::OR,
		rs2: 2,
	};
	cpu.regs[1] = 0;
	cpu.regs[2] = 1;
	let mut instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[2] = 0b0000111100001111;
	cpu.regs[1] = 0b0000111111111111;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0b0000111111111111);
}

#[test]
fn XORI_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::XORI,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	inst.imm = 0b0000111100001111;
	cpu.regs[1] = 0b0000111111110000;

	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0b0000000011111111);
}


#[test]
fn XOR_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::XOR,
		rs2: 2,
	};
	cpu.regs[1] = 1;
	cpu.regs[2] = 0;
	let mut instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[2] = 0b0000111100001111;
	cpu.regs[1] = 0b0000111111110000;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0b0000000011111111);
}

#[test]
fn SLL_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SLL,
		rs2: 2,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[1] = 2;
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],4);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 14);

	cpu.regs[1] = 0xffff;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x1fffe);
}

#[test]
fn SLLI_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SLLI,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.regs[1] = 2;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],4);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 14);

	cpu.regs[1] = 0xffff;

	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x1fffe);
}

#[test]
fn SLLIW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SLLIW,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.regs[1] = 2;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],4);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 14);

	cpu.regs[1] = 0xffffffffffff;

	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0xfffffffffffffffe);
}

#[test]
fn SLLW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SLLW,
		rs2:2,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[1] = 2;
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],4);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 14);

	cpu.regs[1] = 0xffffffff;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0xfffffffffffffffe);
}



#[test]
fn SRL_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SRL,
		rs2: 2,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[1] = 2;
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 3);

	cpu.regs[1] = 0xffff;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x7fff);
}

#[test]
fn SRLI_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SRLI,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.regs[1] = 2;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 3);

	cpu.regs[1] = 0xffff;

	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x7fff);
}

#[test]
fn SRLIW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SRLIW,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.regs[1] = 2;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 3);

	cpu.regs[1] = 0xffff;

	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x7fff);
}

#[test]
fn SRLW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SRLW,
		rs2: 2,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[1] = 2;
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 3);

	cpu.regs[1] = 0xffff;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x7fff);
}


#[test]
fn SRA_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SRA,
		rs2: 2,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[1] = 2;
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 3);

	cpu.regs[1] = 0xffffffffffffffff;

	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0xffffffffffffffff);
}


#[test]
fn SRAI_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SRAI,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.regs[1] = 2;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 3);

	cpu.regs[1] = 0xffffffffffffffff;

	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0xffffffffffffffff);
}


#[test]
#[allow(overflowing_literals)]
fn SRAIW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SRAIW,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.regs[1] = 2;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 3);

	cpu.regs[1] = 0xfffffffe;
	
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0xffffffffffffffff);
}


#[test]
#[allow(overflowing_literals)]
fn SRAW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rs1: 1,
		rd: 1,
		instName: Instruction::SRAW,
		rs2: 2,
	};
	let mut instFmt = InstructionFormat::R(inst);
	cpu.regs[1] = 2;
	cpu.regs[2] =1;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1],1);

	cpu.regs[1] = 7;
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 3);

	cpu.regs[1] = 0xfffffffe;
	
	instFmt = InstructionFormat::R(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0xffffffffffffffff);
}


#[test]
fn BEQ_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = BranchInst {
		rs1: 1,
		rs2: 2,
		
		instName: Instruction::BEQ,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::B(inst);

	cpu.pc = 0;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	cpu.regs[2] = 1;
	cpu.execute(instFmt);

	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;

	cpu.regs[1] = 2;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;

	cpu.pc = 0x1000;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	inst.imm = -0x100;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 0xe00);
	let oldPc = cpu.pc;

	inst.imm = -0xe00;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	//dff because u64::max is 1 less than 0 (if you look at it was signed)
	assert_eq!(cpu.pc, u64::MAX-0xdff);
	let oldPc = cpu.pc;
}

#[test]
fn BNE_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = BranchInst {
		rs1: 1,
		rs2: 2,
		
		instName: Instruction::BNE,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::B(inst);

	cpu.pc = 0;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	cpu.regs[2] = 2;
	cpu.execute(instFmt);

	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;

	cpu.regs[1] = 2;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;


	cpu.pc = 0x1000;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	inst.imm = -0x100;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 0xe00);
	let oldPc = cpu.pc;

	inst.imm = -0xe00;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, u64::MAX-0xdff);
	let oldPc = cpu.pc;
}


#[test]
fn BLTU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = BranchInst {
		rs1: 1,
		rs2: 2,
		
		instName: Instruction::BLTU,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::B(inst);

	cpu.pc = 0;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	cpu.regs[2] = 2;
	cpu.execute(instFmt);

	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;

	cpu.regs[1] = 2;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;


	cpu.regs[1] = 3;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;

	cpu.pc = 0x1000;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	inst.imm = -0x100;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 0xe00);
	let oldPc = cpu.pc;

	inst.imm = -0xe00;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, u64::MAX-0xdff);
	let oldPc = cpu.pc;
}


#[test]
fn BLT_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = BranchInst {
		rs1: 1,
		rs2: 2,
		
		instName: Instruction::BLT,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::B(inst);

	cpu.pc = 0;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	cpu.regs[2] = 2;
	cpu.execute(instFmt);

	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;

	cpu.regs[1] = 2;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;


	cpu.regs[1] = 3;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;

	cpu.regs[1] = u64::MAX;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 4);
	let oldPc = cpu.pc;

	cpu.pc = 0x1000;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	inst.imm = -0x100;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 0xe00);
	let oldPc = cpu.pc;

	inst.imm = -0xe00;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, u64::MAX-0xdff);
	let oldPc = cpu.pc;
}

#[test]
fn BGEU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = BranchInst {
		rs1: 1,
		rs2: 2,
		
		instName: Instruction::BGEU,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::B(inst);

	cpu.pc = 0;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	cpu.regs[2] = 2;
	cpu.execute(instFmt);

	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 0);
	let oldPc = cpu.pc;

	cpu.regs[1] = 2;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;


	cpu.regs[1] = 3;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 4);
	let oldPc = cpu.pc;

	cpu.regs[1] = u64::MAX;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 6);
	let oldPc = cpu.pc;

	cpu.pc = 0x1000;
	let oldPc = cpu.pc;
	cpu.regs[1] = 3;
	inst.imm = -0x100;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 0xe00);
	let oldPc = cpu.pc;

	inst.imm = -0xe00;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, u64::MAX-0xdff);
	let oldPc = cpu.pc;
}


#[test]
fn BGE_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = BranchInst {
		rs1: 1,
		rs2: 2,
		instName: Instruction::BGE,
		imm: 1,
	};
	let mut instFmt = InstructionFormat::B(inst);

	cpu.pc = 0;
	let oldPc = cpu.pc;
	cpu.regs[1] = 1;
	cpu.regs[2] = 2;
	cpu.execute(instFmt);

	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 0);
	let oldPc = cpu.pc;

	cpu.regs[1] = 2;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 2);
	let oldPc = cpu.pc;


	cpu.regs[1] = 3;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 4);
	let oldPc = cpu.pc;

	cpu.regs[1] = u64::MAX;
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 4);
	let oldPc = cpu.pc;

	cpu.pc = 0x1000;
	let oldPc = cpu.pc;
	cpu.regs[1] = 3;
	inst.imm = -0x100;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, 0xe00);
	let oldPc = cpu.pc;

	inst.imm = -0xe00;
	instFmt = InstructionFormat::B(inst);
	cpu.execute(instFmt);
	if cpu.pc != oldPc {
		cpu.pc = cpu.pc.wrapping_add(4);
	}
	assert_eq!(cpu.pc, u64::MAX-0xdff);
	let oldPc = cpu.pc;
}

#[test]
fn JAL_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = JumpInst {
		instName: Instruction::JAL,
		imm: 1,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::J(inst);

	cpu.pc = 10;
	cpu.regs[1] = 0;
	cpu.execute(instFmt);
	assert_eq!(cpu.pc.wrapping_add(4), 12);
	assert_eq!(cpu.regs[1], 14);

	cpu.pc = 10;
	cpu.regs[1] = 0;
	inst.imm = -1;
	instFmt = InstructionFormat::J(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.pc.wrapping_add(4), 8);
	assert_eq!(cpu.regs[1], 14);

	cpu.pc = 0;
	inst.imm = -100;
	instFmt = InstructionFormat::J(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.pc.wrapping_add(4), u64::MAX-199);
	assert_eq!(cpu.regs[1], 4);

}

#[test]
fn JALR_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		instName: Instruction::JALR,
		imm: 2,
		rs1: 2,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);

	cpu.regs[2] = 10;
	cpu.regs[1] = 0;
	cpu.execute(instFmt);
	assert_eq!(cpu.pc.wrapping_add(4), 12);
	assert_eq!(cpu.regs[1], 4);

	cpu.regs[2] = 10;
	cpu.regs[1] = 0;
	inst.imm = -1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.pc.wrapping_add(4), 8);
	assert_eq!(cpu.regs[1], 12);

	cpu.regs[2] = 0;
	inst.imm = -100;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);

	assert_eq!(cpu.pc.wrapping_add(4), u64::MAX-99);
	assert_eq!(cpu.regs[1], 8);
}






#[test]
fn LUI_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = UpperImmInst {
		instName: Instruction::LUI,
		imm: 1,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::U(inst);

	cpu.regs[1] = 0;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 4096);

	inst.imm = 0x10;
	instFmt = InstructionFormat::U(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 65536);

	cpu.regs[1] = 0xff;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 65536);


	inst.imm = 0x80000;
	instFmt = InstructionFormat::U(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0xffffffff80000000);
}


#[test]
fn AUIPC_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = UpperImmInst {
		instName: Instruction::AUIPC,
		imm: 1,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::U(inst);

	cpu.regs[1] = 0;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 4096);

	inst.imm = 0x10;
	instFmt = InstructionFormat::U(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 65536);

	cpu.regs[1] = 0xff;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 65536);


	inst.imm = 0x80000;
	instFmt = InstructionFormat::U(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0xffffffff80000000);

	cpu.pc = 0x4;
	inst.imm = 1;
	instFmt = InstructionFormat::U(inst);
	cpu.regs[1] = 0;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 4100);

	cpu.pc = 100;
	inst.imm = 0x10;
	instFmt = InstructionFormat::U(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 65636);

	cpu.pc= 200;
	cpu.regs[1] = 0xff;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 65736);



	inst.imm = 0x80000;
	instFmt = InstructionFormat::U(inst);
	cpu.pc = 0x100000000;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x80000000);
	
}


#[test]
fn LBU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		instName: Instruction::LBU,
		imm: 0,
		rs1: 2,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.bus.store(0,0x10,1);
	cpu.regs[2] = 0;

	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x10);
	
	cpu.bus.store(0,0x20,8);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x20);

	cpu.bus.store(3, 0x4142434445464748, 8);
	cpu.regs[2] = 3;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x48);

	inst.imm = 1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x47);

	inst.imm = -1;
	cpu.bus.store(2,0x40,1);
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x40);
	
	cpu.bus.store(0,0xff, 1);
	inst.imm = 0;
	instFmt = InstructionFormat::I(inst);
	cpu.regs[2] = 0;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1], 0xff);
}


#[test]
fn LB_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		instName: Instruction::LB,
		imm: 0,
		rs1: 2,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.bus.store(0,0x10,1);
	cpu.regs[2] = 0;

	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x10);
	
	cpu.bus.store(0,0x20,8);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x20);

	cpu.bus.store(3, 0x4142434445464748, 8);
	cpu.regs[2] = 3;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x48);

	inst.imm = 1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x47);

	inst.imm = -1;
	cpu.bus.store(2,0x40,1);
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x40);
	
	cpu.bus.store(0,0xff, 1);
	inst.imm = 0;
	instFmt = InstructionFormat::I(inst);
	cpu.regs[2] = 0;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1] as i64, -1);
}


#[test]
fn LHU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		instName: Instruction::LHU,
		imm: 0,
		rs1: 2,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.bus.store(0,0x10,1);
	cpu.regs[2] = 0;

	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x10);
	
	cpu.bus.store(0,0x20,8);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x20);

	cpu.bus.store(3, 0x4142434445464748, 8);
	cpu.regs[2] = 3;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x4748);

	inst.imm = 1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x4647);

	inst.imm = -1;
	cpu.bus.store(2,0x40,1);
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x4840);

	cpu.bus.store(0,0xffff,2);
	inst.imm = 0;
	instFmt = InstructionFormat::I(inst);
	cpu.regs[2] = 0;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1], 0xffff);
}


#[test]
fn LH_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		instName: Instruction::LH,
		imm: 0,
		rs1: 2,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.bus.store(0,0x10,1);
	cpu.regs[2] = 0;

	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x10);
	
	cpu.bus.store(0,0x20,8);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x20);

	cpu.bus.store(3, 0x4142434445464748, 8);
	cpu.regs[2] = 3;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x4748);

	inst.imm = 1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x4647);

	inst.imm = -1;
	cpu.bus.store(2,0x40,1);
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x4840);

	cpu.bus.store(0,0xffff,2);
	inst.imm = 0;
	instFmt = InstructionFormat::I(inst);
	cpu.regs[2] = 0;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1] as i64, -1);
}

#[test]
fn LWU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		instName: Instruction::LWU,
		imm: 0,
		rs1: 2,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.bus.store(0,0x10,1);
	cpu.regs[2] = 0;

	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x10);
	
	cpu.bus.store(0,0x20,8);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x20);

	cpu.bus.store(3, 0x4142434445464748, 8);
	cpu.regs[2] = 3;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x45464748);

	inst.imm = 1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x44454647);

	inst.imm = -1;
	cpu.bus.store(2,0x40,1);
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x46474840);
	

	cpu.bus.store(0,0xffffffff,4);
	inst.imm = 0;
	instFmt = InstructionFormat::I(inst);
	cpu.regs[2] = 0;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1], 0xffffffff);
}


#[test]
fn LW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		instName: Instruction::LW,
		imm: 0,
		rs1: 2,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.bus.store(0,0x10,1);
	cpu.regs[2] = 0;

	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x10);
	
	cpu.bus.store(0,0x20,8);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x20);

	cpu.bus.store(3, 0x4142434445464748, 8);
	cpu.regs[2] = 3;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x45464748);

	inst.imm = 1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x44454647);

	inst.imm = -1;
	cpu.bus.store(2,0x40,1);
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x46474840);
	

	cpu.bus.store(0,0xffffffff,4);
	inst.imm = 0;
	instFmt = InstructionFormat::I(inst);
	cpu.regs[2] = 0;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1] as i64, -1);
}


#[test]
fn LD_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		instName: Instruction::LD,
		imm: 0,
		rs1: 2,
		rd: 1,
	};
	let mut instFmt = InstructionFormat::I(inst);
	cpu.bus.store(0,0x10,1);
	cpu.regs[2] = 0;

	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x10);
	
	cpu.bus.store(0,0x20,8);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x20);

	cpu.bus.store(3, 0x4142434445464748, 8);
	cpu.regs[2] = 3;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x4142434445464748);

	inst.imm = 1;
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x0041424344454647);

	inst.imm = -1;
	cpu.bus.store(2,0x40,1);
	instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[1], 0x4243444546474840);
	

	cpu.bus.store(0,0xffffffffffffffff,8);
	inst.imm = 0;
	instFmt = InstructionFormat::I(inst);
	cpu.regs[2] = 0;
	cpu.execute(instFmt);

	assert_eq!(cpu.regs[1] as i64, -1);
}



#[test]
fn SB_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = StoreInst {
		instName: Instruction::SB,
		imm: 0,
		rs1: 1,
		rs2: 2,
	};
	let mut instFmt  = InstructionFormat::S(inst);
	cpu.regs[2] = 0x10;
	cpu.regs[1] = 0;
	cpu.execute(instFmt);
	
	assert_eq!(cpu.bus.load(0,1),0x10);

	cpu.regs[2] = 0x2010;
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(0,1), 0x10);

	cpu.regs[1] = 0x4;
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(4,1), 0x10);

	inst.imm = 1;
	instFmt = InstructionFormat::S(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(5,1), 0x10);

	inst.imm = -1;
	instFmt = InstructionFormat::S(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(3,1), 0x10);
}


#[test]
fn SH_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = StoreInst {
		instName: Instruction::SH,
		imm: 0,
		rs1: 1,
		rs2: 2,
	};
	let mut instFmt  = InstructionFormat::S(inst);
	cpu.regs[2] = 0x10;
	cpu.regs[1] = 0;
	cpu.execute(instFmt);
	
	assert_eq!(cpu.bus.load(0,2),0x10);

	cpu.regs[2] = 0x2010;
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(0,2), 0x2010);

	cpu.regs[1] = 0x4;
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(4,2), 0x2010);

	inst.imm = 1;
	instFmt = InstructionFormat::S(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(5,2), 0x2010);

	inst.imm = -1;
	instFmt = InstructionFormat::S(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(3,2), 0x2010);
}

#[test]
fn SW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = StoreInst {
		instName: Instruction::SW,
		imm: 0,
		rs1: 1,
		rs2: 2,
	};
	let mut instFmt  = InstructionFormat::S(inst);
	cpu.regs[2] = 0x10;
	cpu.regs[1] = 0;
	cpu.execute(instFmt);
	
	assert_eq!(cpu.bus.load(0,4),0x10);

	cpu.regs[2] = 0x2010;
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(0,4), 0x2010);

	cpu.regs[1] = 0x4;
	cpu.regs[2] = 0x302010;
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(4,4), 0x302010);

	inst.imm = 1;
	cpu.regs[2] = 0x40302010;
	instFmt = InstructionFormat::S(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(5,4), 0x40302010);

	inst.imm = -1;
	instFmt = InstructionFormat::S(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.bus.load(3,4), 0x40302010);
}

#[test]
fn MUL_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::MUL,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 156);

	//-1 * -1 == 1
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);


	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -0x10);


	cpu.regs[1] = 0x7fffffffffffffff;
	cpu.regs[2] = 0x7fffffffffffffff;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);
}

#[test]
fn MULW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::MULW,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 156);

	//-1 * -1 == 1
	cpu.regs[1] = u32::MAX as u64;
	cpu.regs[2] = u32::MAX as u64;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);


//	panic!("ayy");
	cpu.regs[1] = u32::MAX as u64;
	cpu.regs[2] = 10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -10);


	cpu.regs[1] = 0x7fffffff;
	cpu.regs[2] = 0x7fffffff;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);

	cpu.regs[1] = 0xaaaaaaaa7fffffff;
	cpu.regs[2] = 0xaaaaaaaa7fffffff;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);
}


#[test]
fn MULH_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::MULH,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	//-1 * -1 == 1
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -0x1_i64);


	cpu.regs[1] = 0x7fffffffffffffff;
	cpu.regs[2] = 0x7fffffffffffffff;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0x3fffffffffffffff);
}


#[test]
fn MULHU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::MULHU,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], u64::MAX-1);


	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0xf);


	cpu.regs[1] = 0x7fffffffffffffff;
	cpu.regs[2] = 0x7fffffffffffffff;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0x3fffffffffffffff);
}


#[test]
fn MULHSU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::MULHSU,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -1);

	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = 10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -1);


	cpu.regs[1] = 0x7fffffffffffffff;
	cpu.regs[2] = 0x7fffffffffffffff;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, 0x3fffffffffffffff);
}


#[test]
fn DIV_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::DIV,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);


	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	cpu.regs[1] = 24;
	cpu.regs[2] = 12;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 2);

}



#[test]
fn DIVU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::DIVU,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);


	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0xfffffffffffffff);

	cpu.regs[1] = 24;
	cpu.regs[2] = 12;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 2);


}



#[test]
fn DIVW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::DIVW,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	
	cpu.regs[1] = 0x100000000;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);

	cpu.regs[1] = 24;
	cpu.regs[2] = 12;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 2);

	cpu.regs[1] = 1;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -1);
}

#[test]
fn DIVUW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::DIVUW,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);


	cpu.regs[1] = 0x100000000;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	cpu.regs[1] = 24;
	cpu.regs[2] = 12;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 2);


	cpu.regs[1] = 1;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);
}



#[test]
fn REM_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::REM,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 12);

	
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = 0x100000000;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	cpu.regs[1] = 24;
	cpu.regs[2] = 12;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = u64::MAX -6;
	cpu.regs[2] = u64::MAX -1;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -1);


	cpu.regs[1] = u64::MAX -6;
	cpu.regs[2] = 2;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -1);
}



#[test]
fn REMU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::REMU,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 12);

	
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = 0x100000000;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);

	cpu.regs[1] = 24;
	cpu.regs[2] = 12;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = u64::MAX -6;
	cpu.regs[2] = u64::MAX -1;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], u64::MAX -6);


	cpu.regs[1] = u64::MAX -6;
	cpu.regs[2] = 2;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);
}



#[test]
fn REMW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::REMW,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 12);

	
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = 0x100000001;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);

	cpu.regs[1] = 24;
	cpu.regs[2] = 12;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = u64::MAX -6;
	cpu.regs[2] = u64::MAX -1;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -1);


	cpu.regs[1] = u64::MAX -6;
	cpu.regs[2] = 2;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -1);
}


#[test]
fn REMUW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::REMUW,
		rd: 0,
		rs1: 1,
		rs2: 2,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.regs[2] = 13;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 12);

	
	cpu.regs[1] = u64::MAX;
	cpu.regs[2] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = 0x100000001;
	cpu.regs[2] = 0x10;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);

	cpu.regs[1] = 24;
	cpu.regs[2] = 12;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.regs[1] = u64::MAX -6;
	cpu.regs[2] = u64::MAX -1;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as u32, u32::MAX -6);


	cpu.regs[1] = u64::MAX -6;
	cpu.regs[2] = 2;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);
}

#[test]
fn FLW_test() {
			let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegImmInst {
		instName: Instruction::FLW,
		rd: 0,
		rs1: 1,
		imm: 0,
	};
	let instFmt = InstructionFormat::I(inst);
	cpu.bus.store(0,(1.0f32).to_bits() as u64,4);
	cpu.regs[1] = 0;

	cpu.execute(instFmt);
	assert_eq!(cpu.fregs[0], 1.0);
	
	cpu.bus.store(4,(1.23456f32).to_bits() as u64,4);
	cpu.regs[1] = 4;
	cpu.execute(instFmt);
	assert_eq!(cpu.fregs[0], 1.23456);

	cpu.bus.store(8, (-1.234 * 10f32).to_bits() as u64,4);
	inst.imm = 4;
	let instFmt = InstructionFormat::I(inst);
	cpu.execute(instFmt);
	assert_eq!(cpu.fregs[0], -1.234*10f64);
}



#[test]
fn FSW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = StoreInst {
		instName: Instruction::FSW,
		rs1: 0,
		rs2: 1,
		imm: 0,
	};
	let instFmt = InstructionFormat::S(inst);
	cpu.fregs[1] = 1.0;

	cpu.execute(instFmt);
	println!("{:b}", cpu.bus.load(0,4));
	assert_eq!(f32::from_bits(cpu.bus.load(0,4) as u32), 1.0);
	
	cpu.fregs[1] = 1.23456;
	cpu.execute(instFmt);
	assert_eq!(f32::from_bits(cpu.bus.load(0,4) as u32), 1.23456);

	inst.imm = 4;
	let instFmt = InstructionFormat::S(inst);
	cpu.fregs[1] = -1.234*10f64;
	cpu.execute(instFmt);
	assert_eq!(f32::from_bits(cpu.bus.load(4,4) as u32), -1.234*10f32);
}

#[test]
fn FADDS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::FADDS,
		rd: 0,
		rs1: 1,
		rs2:2,
	};
	let instFmt = InstructionFormat::R(inst);
	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 2.0);
	

	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 0.0);

	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,18)));
	cpu.fregs[2] = Cpu::box_float(1.2*(f32::powi(10.0,16)));

	cpu.execute(instFmt);
	let a  = 1.2465*(f32::powi(10.0,18));
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), a);

}


#[test]
fn FSUBS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::FSUBS,
		rd: 0,
		rs1: 1,
		rs2:2,
	};
	let instFmt = InstructionFormat::R(inst);
	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 0.0);


	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 2.0);

	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,18)));
	cpu.fregs[2] = Cpu::box_float(1.2*(f32::powi(10.0,16)));

	cpu.execute(instFmt);
	let a  = 1.2225*(f32::powi(10.0,18)); //f32 are so inaccurate, must assign to a variable before asserting
											//to avoid a failed asertion because the literal doesnt round but the var does
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), a);

}


#[test]
fn FMULS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::FMULS,
		rd: 0,
		rs1: 1,
		rs2:2,
	};
	let instFmt = InstructionFormat::R(inst);
	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.0);


	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -1.0);

	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,18)));
	cpu.fregs[2] = Cpu::box_float(1.2*(f32::powi(10.0,16)));

	cpu.execute(instFmt);
	let a =  1.4814*(f32::powi(10.0,34));
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), a);


	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,18)));
	cpu.fregs[2] = Cpu::box_float(1.0*(f32::powi(10.0,-2)));

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.2345*(f32::powi(10.0,16)));
}

#[test]
fn FDIVS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::FDIVS,
		rd: 0,
		rs1: 1,
		rs2:2,
	};
	let instFmt = InstructionFormat::R(inst);
	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.0);


	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -1.0);

	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,18)));
	cpu.fregs[2] = Cpu::box_float(1.2*(f32::powi(10.0,16)));

	cpu.execute(instFmt);
	let a : f32 = 102.875;
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), a);

	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,18)));
	cpu.fregs[2] = Cpu::box_float(1.0*(f32::powi(10.0,-2)));

	cpu.execute(instFmt);
	let a : f32 = 1.2345*(f32::powi(10.0,20));
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), a);
}

#[test]
fn FMAXS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::FMAXS,
		rd: 0,
		rs1: 1,
		rs2:2,
	};
	let instFmt = InstructionFormat::R(inst);
	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.0);


	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.0);

	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,18)));
	cpu.fregs[2] = Cpu::box_float(1.2*(f32::powi(10.0,16)));

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.2345*(f32::powi(10.0,18)));

	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,-18)));
	cpu.fregs[2] = Cpu::box_float(1.2*(f32::powi(10.0,16)));

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.2*(f32::powi(10.0,16)));
}

#[test]
fn FMINS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::FMINS,
		rd: 0,
		rs1: 1,
		rs2:2,
	};
	let instFmt = InstructionFormat::R(inst);
	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.0);


	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -1.0);

	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,18)));
	cpu.fregs[2] = Cpu::box_float(1.2*(f32::powi(10.0,16)));

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), Cpu::unbox_float(cpu.fregs[2]));

	cpu.fregs[1] = Cpu::box_float(1.2345*(f32::powi(10.0,-18)));
	cpu.fregs[2] = Cpu::box_float(1.2*(f32::powi(10.0,16)));

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), Cpu::unbox_float(cpu.fregs[1]));
}

#[test]
fn FSQRTS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		instName: Instruction::FSQRTS,
		rd: 0,
		rs1: 1,
		rs2:2,
	};
	let instFmt = InstructionFormat::R(inst);
	cpu.fregs[1] = Cpu::box_float(1.0);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.0);


	cpu.fregs[1] = Cpu::box_float(2.25);

	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 1.5);

	cpu.fregs[1] = Cpu::box_float(1522756000000000000000000.0);
	cpu.execute(instFmt);
	let a  = Cpu::box_float(1234.0*(f32::powi(10.0, 9)));
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), Cpu::unbox_float(a));
}


//need to test all the different rouding modes when implemented
#[test] //unsigned long to float
fn FCVTSLU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FCVTSLU
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 12.0);

	cpu.regs[1] = 13;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 13.0);

	cpu.regs[1] = 14;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 14.0);

}

#[test] //signed long to float 
fn FCVTSL_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FCVTSL
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 12.0);

	cpu.regs[1] = 13;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 13.0);

	cpu.regs[1] = u64::MAX;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -1.0);

}

#[test] //unsigned word to float 
fn FCVTSWU_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FCVTSWU
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 12.0);

	cpu.regs[1] = 13;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 13.0);

	cpu.regs[1] = u32::MAX as u64;
	cpu.execute(instFmt);
	let a = Cpu::box_float(4294967295.0);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), Cpu::unbox_float(a));

}

#[test] //signed word to float  
fn FCVTSW_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FCVTSW
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.regs[1] = 12;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 12.0);

	cpu.regs[1] = 13;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 13.0);

	cpu.regs[1] = u32::MAX as u64;
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -1.0);
}

#[test] //float to unsigned word 
fn FCVTLUS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FCVTLUS
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.fregs[1] = 	Cpu::box_float(12.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 12);

	cpu.fregs[1] = 	Cpu::box_float(13.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 13);

	cpu.fregs[1] = 	Cpu::box_float(12.5);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 12);
	
	//technically incorrect accorind to riscv specification,
	//rust seems to consistenyl round down upon even
	//fix this test when rounding-mode function  is implemented
	cpu.fregs[1] = 	Cpu::box_float(13.5);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 13);


	cpu.fregs[1] = 	Cpu::box_float(13.4);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 13);
	
	
	cpu.fregs[1] =	Cpu::box_float(18446744073709551616.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0],u64::MAX);
}

#[test]
fn FCVTLS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FCVTLS
	};
	let instFmt = InstructionFormat::R(inst);


	cpu.fregs[1] = 	Cpu::box_float(12.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 12);

	cpu.fregs[1] = 	Cpu::box_float(13.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 13);

	cpu.fregs[1] = 	Cpu::box_float(13.5);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 13);

	//agony
	cpu.fregs[1] = 	Cpu::box_float(13.9);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 14);
	
	cpu.fregs[1] = 	Cpu::box_float(4294967295.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 4294967295);

	cpu.fregs[1] = 	Cpu::box_float(18446744073709551615.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i64, -1);

}

#[test]
fn FCVTWUS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FCVTWUS
	};
	let instFmt = InstructionFormat::R(inst);

	
	cpu.fregs[1] = Cpu::box_float(12.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 12);

	cpu.fregs[1] =Cpu::box_float(13.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 13);

	cpu.fregs[1] =Cpu::box_float(13.4);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 13);


	cpu.fregs[1] =Cpu::box_float(13.6);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 13);

	cpu.fregs[1] = Cpu::box_float(4294967295.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 4294967296);
}

#[test]
fn FCVTWS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FCVTWS
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.fregs[1] = Cpu::box_float(12.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 12);

	cpu.fregs[1] = Cpu::box_float(13.5);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 13);

	cpu.fregs[1] = Cpu::box_float(2147483647.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 2147483647);

	cpu.fregs[1] = Cpu::box_float(4294967295.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0] as i32, -1);

}

#[test]
fn FEQS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FEQS
	};
	let instFmt = InstructionFormat::R(inst);
	
	cpu.fregs[1] = 1.0;
	cpu.fregs[2] = 1.0;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);


	cpu.fregs[1] = 1.0;
	cpu.fregs[2] = 1.2;
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);
	
}

#[test]
fn FLTS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FLTS
	};
	let instFmt = InstructionFormat::R(inst);
	
	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);


	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.2);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);
	
	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-1.2);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);
}

#[test]
fn FLES_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FLES
	};
	let instFmt = InstructionFormat::R(inst);

	
	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.0);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);


	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(1.2);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 1);


	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-1.2);
	cpu.execute(instFmt);
	assert_eq!(cpu.regs[0], 0);
	
}

#[test]
fn FSGNJS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FSGNJS,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.fregs[1] = Cpu::box_float(f32::from_bits(0b1));
	cpu.fregs[2] = Cpu::box_float(f32::from_bits(0x80000000));
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), f32::from_bits(0b10000000000000000000000000000001));

	cpu.fregs[2] = Cpu::box_float(f32::from_bits(0x80000000-1));
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), f32::from_bits(0b00000000000000000000000000000001));

	
}
#[test]
fn FSGNJNS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FSGNJNS,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.fregs[1] = Cpu::box_float(f32::from_bits(0b1));
	cpu.fregs[2] = Cpu::box_float(f32::from_bits(0x80000000));
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), f32::from_bits(0b00000000000000000000000000000001));

	cpu.fregs[2] = Cpu::box_float(f32::from_bits(0x80000000 -1));
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), f32::from_bits(0b10000000000000000000000000000001));

	
}

#[test]
fn FSGNJXS_test() {
	let fakeData: Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(fakeData);
	let mut inst = RegRegInst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		instName: Instruction::FSGNJXS,
	};
	let instFmt = InstructionFormat::R(inst);

	cpu.fregs[1] = Cpu::box_float(f32::from_bits(0b1));
	cpu.fregs[2] = Cpu::box_float(f32::from_bits(0x80000000));
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), f32::from_bits(0b10000000000000000000000000000001));

	cpu.fregs[2] = Cpu::box_float(f32::from_bits(0x80000000 -1));
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), f32::from_bits(0b10000000000000000000000000000001));


	cpu.fregs[1] = Cpu::box_float(f32::from_bits(0x80000000 +1));
	cpu.fregs[2] = Cpu::box_float(f32::from_bits(0x80000000));
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), f32::from_bits(0b00000000000000000000000000000001));
	
}


#[test]
fn FMADDS_test() {
	let fakeDat : Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(faleData);
	let mut inst = R3Inst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		rs3: 3,
		instName: Instruction::FMADDS,
	};
	let instFmt = InstructionFormat::R3(inst);

	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-2.0);
	cpu.fregs[3] = Cpu::box_float(0.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -2.0);


	cpu.fregs[1] = Cpu::box_float(0.0);
	cpu.fregs[2] = Cpu::box_float(0.0);
	cpu.fregs[3] = Cpu::box_float(3.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 3.0);
	
	cpu.fregs[1] = Cpu::box_float(1.5);
	cpu.fregs[2] = Cpu::box_float(-1.5);
	cpu.fregs[3] = Cpu::box_float(3.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 0.75);
	
	cpu.fregs[1] = Cpu::box_float(1.5*f32::powi(10.0, 16));
	cpu.fregs[2] = Cpu::box_float(-2.0*f32::powi(10.0, 16));
	cpu.fregs[3] = Cpu::box_float(-0.5*f32::powi(10.0,16)); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -3.5*f32::powi(10.0,16));
}


#[test]
fn FMSUBS_test() {
	let fakeDat : Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(faleData);
	let mut inst = R3Inst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		rs3: 3,
		instName: Instruction::FMSUBS,
	};
	let instFmt = InstructionFormat::R3(inst);

	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-2.0);
	cpu.fregs[3] = Cpu::box_float(0.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -2.0);


	cpu.fregs[1] = Cpu::box_float(0.0);
	cpu.fregs[2] = Cpu::box_float(0.0);
	cpu.fregs[3] = Cpu::box_float(3.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -3.0);
	
	cpu.fregs[1] = Cpu::box_float(1.5);
	cpu.fregs[2] = Cpu::box_float(-1.5);
	cpu.fregs[3] = Cpu::box_float(3.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -5.25);
	
	cpu.fregs[1] = Cpu::box_float(1.5*f32::powi(10.0, 16));
	cpu.fregs[2] = Cpu::box_float(-2.0*f32::powi(10.0, 16));
	cpu.fregs[3] = Cpu::box_float(-0.5*f32::powi(10.0,16)); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -2.5*f32::powi(10.0,16));
}

#[test]
fn FNMADDS_test() {
	let fakeDat : Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(faleData);
	let mut inst = R3Inst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		rs3: 3,
		instName: Instruction::FNMADDS,
	};
	let instFmt = InstructionFormat::R3(inst);

	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-2.0);
	cpu.fregs[3] = Cpu::box_float(0.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 2.0);


	cpu.fregs[1] = Cpu::box_float(0.0);
	cpu.fregs[2] = Cpu::box_float(0.0);
	cpu.fregs[3] = Cpu::box_float(3.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -3.0);
	
	cpu.fregs[1] = Cpu::box_float(1.5);
	cpu.fregs[2] = Cpu::box_float(-1.5);
	cpu.fregs[3] = Cpu::box_float(3.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), -0.75);
	
	cpu.fregs[1] = Cpu::box_float(1.5*f32::powi(10.0, 16));
	cpu.fregs[2] = Cpu::box_float(-2.0*f32::powi(10.0, 16));
	cpu.fregs[3] = Cpu::box_float(-0.5*f32::powi(10.0,16)); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 3.5*f32::powi(10.0,16));
}
#[test]
fn FNMSUBS_test() {
	let fakeDat : Vec<u8> = vec![0;10];
	let mut cpu = Cpu::new(faleData);
	let mut inst = R3Inst {
		rd: 0,
		rs1: 1,
		rs2: 2,
		rs3: 3,
		instName: Instruction::FNMSUBS,
	};
	let instFmt = InstructionFormat::R3(inst);

	cpu.fregs[1] = Cpu::box_float(1.0);
	cpu.fregs[2] = Cpu::box_float(-2.0);
	cpu.fregs[3] = Cpu::box_float(0.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 2.0);


	cpu.fregs[1] = Cpu::box_float(0.0);
	cpu.fregs[2] = Cpu::box_float(0.0);
	cpu.fregs[3] = Cpu::box_float(3.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 3.0);
	
	cpu.fregs[1] = Cpu::box_float(1.5);
	cpu.fregs[2] = Cpu::box_float(-1.5);
	cpu.fregs[3] = Cpu::box_float(3.0); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 5.25);
	
	cpu.fregs[1] = Cpu::box_float(1.5*f32::powi(10.0, 16));
	cpu.fregs[2] = Cpu::box_float(-2.0*f32::powi(10.0, 16));
	cpu.fregs[3] = Cpu::box_float(-0.5*f32::powi(10.0,16)); 
	cpu.execute(instFmt);
	assert_eq!(Cpu::unbox_float(cpu.fregs[0]), 2.5*f32::powi(10.0,16));
}
