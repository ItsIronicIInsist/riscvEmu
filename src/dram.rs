#![allow(non_snake_case)]

pub struct Dram {
	pub dram : Vec<u8>,
}


impl Dram {
	pub fn New(code: Vec<u8>) -> Dram {
		let mut mem = Self {
			dram: vec![0; 1024*1024],
		};
		//funky splice method
		//good for replacing subsection of a vector
		//takes a range (usually a lloop, but could be every second)
		//and takes an iterator (or smn that can go into one)
		//needs .cloned() because it needs direct reference? Like, T instead of &T??
		//idk
		mem.dram.splice(..code.len(), code.iter().cloned());
		mem
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
		let mut sizeBound: usize  = size as usize;
		if size > 8 {
			sizeBound = 8;
		}
		let mut bytes : [u8;8] = [0;8];

		for i in (0..(sizeBound)) {
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
		let addrInd = addr as usize;
		let mut tmpSize = size;
		if tmpSize > 8 {
			tmpSize = 8;
		}
		for i in (0..(tmpSize as usize)) {
			self.dram[addrInd + i] = (data >> (i*8) & 0xff) as u8;
		//	println!(" mem val stored is {:x}", self.dram[addrInd + i]);
		}
	}

}
