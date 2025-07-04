mod balances;
mod support;
mod system;

use types::AccountId;

use crate::support::Dispatch;

mod types {
	pub type Balance = u128;
	pub type AccountId = String;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
	BalancesTransfer { to: types::AccountId, amount: types::Balance },
}

#[derive(Debug)]
pub struct Runtime {
	pub system: system::Pallet<Self>,
	pub balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	fn new() -> Self {
		Runtime { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number(); // increment the system's block number
		if self.system.block_number() != block.header.block_number {
			return Err("Block number doesn't match current block");
		}
		// An extrinsic error is not enough to trigger the block to be invalid. We capture the
		// result, and emit an error message if one is emitted.
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::BalancesTransfer { to, amount } => {
				self.balances.transfer(caller, to, amount)?
			},
		}

		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();

	// specify users
	let alice = String::from("alice");
	let bob = String::from("bob");
	let charlie = String::from("charlie");

	// Initialize the system with some inital balance
	runtime.balances.set_balance(&alice, 100);

	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::BalancesTransfer { to: bob.clone(), amount: 30 },
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::BalancesTransfer { to: charlie.clone(), amount: 20 },
			},
		],
	};

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: charlie.clone(),
				call: RuntimeCall::BalancesTransfer { to: alice.clone(), amount: 10 },
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::BalancesTransfer { to: charlie.clone(), amount: 10 },
			},
		],
	};

	// Execute the extrinsics which make up our block.
	// If there are any errors, our system panics, since we should not execute invalid blocks
	runtime.execute_block(block_1).expect("invalid block");
	runtime.execute_block(block_2).expect("invalid block");

	println!("{:#?}", runtime);
}
