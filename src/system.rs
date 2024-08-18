use std::collections::BTreeMap;

use num::traits::{CheckedAdd, One, Zero};

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + CheckedAdd + Copy;
	type Nonce: Zero + One + Copy + CheckedAdd;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		// crashes if overflow for purpose
		self.block_number = self.block_number.checked_add(&T::BlockNumber::one()).unwrap();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce.checked_add(&T::Nonce::one()).unwrap();
		self.nonce.insert(who.clone(), new_nonce);
	}

	pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&T::Nonce::zero())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let system: super::Pallet<TestConfig> = super::Pallet::new();

		assert_eq!(system.block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		let mut system: super::Pallet<TestConfig> = super::Pallet::new();
		system.inc_block_number();

		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let alice = String::from("alice");
		let mut system: super::Pallet<TestConfig> = super::Pallet::new();

		system.inc_nonce(&alice);

		assert_eq!(system.get_nonce(&alice), 1);
	}
}
