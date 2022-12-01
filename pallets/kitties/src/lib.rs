#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, ValueQuery, OptionQuery}, Parameter};
	use frame_system::{pallet_prelude::*};
	use frame_support::dispatch::Vec;
	use frame_system::pallet;

	// Bắt buộc 
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}


	// Định nghĩa giới tính
	pub enum KittyGender {
		Male, 
		Female,
	}

	// Định nghĩa Struct
	pub struct Kitty<T: Config> {
		pub dna: Vec<u8>,
		pub owner: T::AccountId,
		pub price: u32,
		pub gender: KittyGender,
	}
	
	// Giới tính từ vec dna 
	pub fn gender_of_kittty_from_vec (dna: Vec<u8>) -> KittyGender{
		if dna.len()%2==0 {
			return KittyGender::Male;
		}else{
			return KittyGender::Female;
		}
	}
	// Giới tính từ struct Kitty 
	// pub fn gender_of_kitty<T:Config> (Kitty: Kitty<T>) -> KittyGender{
	// 	if Kitty.dna.len()%2==0 {
	// 		return KittyGender::Male;
	// 	}else{
	// 		return KittyGender::Female;
	// 	}
		
	// }
	// Tạo một Kitty mới, chuyển Ower
	impl<T:Config> Kitty<T> {
		fn new_kitty (owner: T::AccountId, dna: Vec<u8>, price: u32) -> Kitty<T>{
			let kittygender = gender_of_kittty_from_vec(dna.clone());
			Kitty {
				dna: dna,
				owner: owner,
				price: price,
				gender: kittygender,
			}
		}

		fn change_owner (Kitty: &mut Kitty<T>, newowner: T::AccountId) {
			Kitty.owner = newowner;
		}
	} 

	// Số  lượng Kitty
	#[pallet::storage]
	#[pallet::getter(fn get_amount_kitty)]
	pub type Amount_Kitty<T> = StorageValue<_,u32,ValueQuery>;

	// // Lưu Map(dna, structKitty) 
	// #[pallet::storage]
	// #[pallet::getter(fn get_kitty)]
	// pub(super) type KittyInfo<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, ValueQuery>;



	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_,u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		// Thêm vào
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		// pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			// let who = ensure_signed(origin)?;
			// let kittygender = gender_of_kittty_from_vec(dna);
			// let New_Kitty: Kitty<T:Config> = Kitty {
			// 	dna: dna,
			// 	owner: T::AccountId,
			// 	price: price,
			// 	gender: kittygender,
			// };
			// Chỗ  này chưa biết store như thế nào
			// <Something<T>>::put(something);

			// // Emit an event.
			// Self::deposit_event(Event::SomethingStored {something, who });
			// Return a successful DispatchResultWithPostInfo
		// 	Ok(())
		// }
	}
}
