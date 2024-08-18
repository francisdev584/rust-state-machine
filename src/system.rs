use std::collections::BTreeMap;

use num::traits::{CheckedAdd, One, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
	AccountId: Ord + Clone,
	BlockNumber: Zero + One + CheckedAdd,
	Nonce: Zero + One + Copy + CheckedAdd,
{
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		// crashes if overflow for purpose
		self.block_number = self.block_number.checked_add(&BlockNumber::one()).unwrap();
	}

	pub fn inc_nonce(&mut self, who: &AccountId) {
		let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
		let new_nonce = nonce.checked_add(&Nonce::one()).unwrap();
		self.nonce.insert(who.clone(), new_nonce);
	}

	pub fn get_nonce(&self, who: &AccountId) -> Nonce {
		*self.nonce.get(who).unwrap_or(&Nonce::zero())
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let system = super::Pallet::<String, u32, u32>::new();

		assert_eq!(system.block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		let mut system = super::Pallet::<String, u32, u32>::new();
		system.inc_block_number();

		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let alice = String::from("alice");
		let mut system = super::Pallet::<String, u32, u32>::new();

		system.inc_nonce(&alice);

		assert_eq!(system.get_nonce(&alice), 1);
	}
}
