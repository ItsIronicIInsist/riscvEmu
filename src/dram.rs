extern crate num_traits;

use std::mem;
use std::ops::Shr;
use std::ops::BitAnd;
use num_traits::int::PrimInt;
pub struct Dram {
	pub dram : Vec<u8>,
}


impl Dram {
	pub fn New(code: Vec<u8>) -> Dram {
		Self {
			dram: code,
		}
	}

	pub fn load(&self, addr: u64, size: u8) -> u64 {
		//Idea
		//addr as type generic
		//dynamically find its size
		//loop for i in 0..size, grabbing a byte each time
		//after loop has finished, just take bottom 8 bytes
		//then assort a u64 from the array (padding if less than 8 bytes)
		//then return u64
		//then dont need like, a function per integer size
		let addrInd = addr as usize;
		if size > 8 {
			let size = 8;
		}
		let mut bytes : [u8;8] = [0;8];

		for i in (0..(size as usize)) {
			bytes[i] = (self.dram[addrInd + i ]);
		}

		let instCode = u64::from_le_bytes(bytes);
		
		instCode
	}

	//tried to make this generic being have data be any type T
	//but then it had to be able to be bitshifted by a u8 - thats fine
	//but then is had to be able to be coerced into a u8 - but not T, but the results of T after being bitshifted
	//how the hell does anyone do anything useful in a function with generics
	pub fn store(&mut self, addr: u64, data: u64, size: u8) {
		let addrInd = addr as usize;;
		if size > 8 {
			let size = 8;
		}

		for i in (0..(size as usize)) {
			self.dram[addrInd + i] = (data >> (((i as u8)*8)) & 0xff) as u8;	
		}
	}
}
