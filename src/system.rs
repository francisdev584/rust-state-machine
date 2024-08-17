use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;
#[derive(Debug)]
pub struct Pallet {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	pub fn block_number(self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		// crashes if overflow for purpose
		self.block_number = self.block_number.checked_add(1).unwrap();
	}

	pub fn inc_nonce(&mut self, who: &AccountId) {
		let nonce = self.nonce.get(who).unwrap_or(&0);
		let new_nonce = nonce.checked_add(1).unwrap();
		self.nonce.insert(who.clone(), new_nonce);
	}

	pub fn get_nonce(&self, who: &AccountId) -> Nonce {
		*self.nonce.get(who).unwrap_or(&0)
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
