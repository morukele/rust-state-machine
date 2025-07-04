use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config {
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
	type AccountId: Ord + Clone;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(caller, new_caller_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {

	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 50),
			Err("Not enough funds.")
		);

		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 50), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 50);
		assert_eq!(balances.balance(&"bob".to_string()), 50);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 100),
			Err("Not enough funds.")
		);
	}
}
