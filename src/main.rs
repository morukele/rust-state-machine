mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime {
	pub system: system::Pallet,
	pub balances: balances::Pallet,
}

impl Runtime {
	fn new() -> Self {
		Runtime { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();

	// specify users
	let alice = String::from("alice");
	let bob = String::from("bob");
	let charlie = String::from("charlie");

	runtime.balances.set_balance(&alice, 100);

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// First transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob, 30)
		.map_err(|e| eprintln!("Error: {}", e));

	// Second transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), charlie.clone(), 20)
		.map_err(|e| eprintln!("Error: {}", e));

	println!("{:#?}", runtime);
}
