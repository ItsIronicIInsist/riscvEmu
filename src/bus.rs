
//will add stuff for signals later
//this is what carries signals from the cpu to DRAM and other devices


use crate::dram::*;
pub struct Bus {
	pub dram: Dram,
}

impl Bus {
	pub fn New(code: Vec<u8>) -> Bus {
		Self {
			dram: Dram::New(code),
		}
	}

	pub fn load(&self, addr: u64, size: u8) -> u64 {
		self.dram.load(addr, size)
	}

	pub fn store(&mut self, addr: u64, data: u64, size: u8) {
		self.dram.store(addr, data,size);
	}
}
