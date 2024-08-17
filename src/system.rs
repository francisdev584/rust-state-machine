use std::collections::BTreeMap;

pub struct Pallet {
	block_number: u32,
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	pub fn block_number(self) -> u32 {
        unimplemented!();
	}

	pub fn inc_block_number(&mut self) {
        unimplemented!();
	}

	pub fn inc_nonce(&mut self, who: &String) {
        unimplemented!();
	}

	pub fn get_nonce(&self, who: &String) -> u32 {
        unimplemented!();
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let system = super::Pallet::new();

		assert_eq!(system.block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		let mut system = super::Pallet::new();
		system.inc_block_number();

		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let alice = String::from("alice");
		let mut system = super::Pallet::new();

		system.inc_nonce(&alice);

		assert_eq!(system.get_nonce(&alice), 1);
	}
}
