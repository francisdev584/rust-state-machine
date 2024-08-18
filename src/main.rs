mod balances;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
}
// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}
fn main() {
	let mut runtime = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice.clone(), 100);

	runtime.system.inc_block_number();

	// assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(&alice);

	let _ = runtime
		.balances
		.transfer(alice.clone(), bob.clone(), 30)
		.map_err(|e| println!("Error:{:?}", e));

	runtime.system.inc_nonce(&alice);

	let _ = runtime
		.balances
		.transfer(alice.clone(), charlie.clone(), 20)
		.map_err(|e| println!("Error:{:?}", e));

	println!("{:#?}", runtime);
}
