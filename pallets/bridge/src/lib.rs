//! # Bridge Pallet
//!
//! A pallet for interchain communication using XCM (Cross-Consensus Messaging).
//! This pallet provides a foundation for sending and receiving cross-chain messages.
//!
//! ## Overview
//!
//! This bridge pallet provides:
//! - Message sending to other parachains
//! - Message receiving and processing
//! - Cross-chain communication tracking
//! - Event emission for cross-chain operations
//!
//! ## Features
//!
//! - **Message Sending**: Send messages to other parachains
//! - **Message Tracking**: Track the status of sent messages
//! - **Event System**: Emit events for cross-chain operations
//! - **Storage Management**: Store message metadata and status

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame::prelude::*;
use core::sync::atomic::{AtomicU64, Ordering};
use sp_runtime::traits::Get;
use alloc::vec::Vec;

// Static counter for message IDs to avoid type resolution issues
static MESSAGE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[frame::pallet]
pub mod pallet {
	use super::*;
	use crate::weights::WeightInfo;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: crate::weights::WeightInfo;

		/// The maximum number of messages that can be queued for a destination.
		#[pallet::constant]
		type MaxQueuedMessages: Get<u32>;

		/// The maximum size of a message.
		#[pallet::constant]
		type MaxMessageSize: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Storage for tracking message statuses.
	#[pallet::storage]
	#[pallet::getter(fn message_status)]
	pub type MessageStatus<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AccountId, MessageId),
		MessageStatusInfo<BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Storage for queued messages to be sent.
	#[pallet::storage]
	#[pallet::getter(fn queued_messages)]
	pub type QueuedMessages<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, ConstU32<32>>, // Destination identifier with size limit
		BoundedVec<QueuedMessage<T>, T::MaxQueuedMessages>,
		ValueQuery,
	>;

	/// Storage for message IDs to prevent duplicates.
	#[pallet::storage]
	pub type MessageIds<T: Config> = StorageMap<_, Blake2_128Concat, MessageId, (), ValueQuery>;

	/// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Message sent to another chain.
		MessageSent {
			who: T::AccountId,
			destination: BoundedVec<u8, ConstU32<32>>,
			message: BoundedVec<u8, ConstU32<1024>>,
			message_id: MessageId,
		},
		/// Message received from another chain.
		MessageReceived {
			from: BoundedVec<u8, ConstU32<32>>,
			message: BoundedVec<u8, ConstU32<1024>>,
			message_id: MessageId,
		},
		/// Message execution failed.
		MessageFailed {
			message_id: MessageId,
			error: BoundedVec<u8, ConstU32<256>>,
		},
		/// Cross-chain function call executed.
		CrossChainCallExecuted {
			who: T::AccountId,
			destination: BoundedVec<u8, ConstU32<32>>,
			call_data: BoundedVec<u8, ConstU32<1024>>,
		},
	}

	/// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Message destination is invalid.
		InvalidDestination,
		/// Message is too large.
		MessageTooLarge,
		/// Failed to send message.
		MessageSendFailed,
		/// Message ID already exists.
		DuplicateMessageId,
		/// Queue is full for this destination.
		QueueFull,
		/// Invalid message format.
		InvalidMessageFormat,
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Send a message to another chain.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::send_message())]
		pub fn send_message(
			origin: OriginFor<T>,
			destination: Vec<u8>,
			message: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(!destination.is_empty(), Error::<T>::InvalidDestination);

			ensure!(
				message.len() <= T::MaxMessageSize::get() as usize,
				Error::<T>::MessageTooLarge
			);

			let message_id = Self::generate_message_id(&who, &message);

			ensure!(
				!MessageIds::<T>::contains_key(message_id),
				Error::<T>::DuplicateMessageId
			);

			let bounded_destination = BoundedVec::try_from(destination.clone()).unwrap();

			Self::queue_message(&destination, &message, &who)?;

			MessageIds::<T>::insert(message_id, ());

			MessageStatus::<T>::insert(
				(&who, message_id),
				MessageStatusInfo {
					status: MessageStatusType::Sent,
					block_number: frame_system::Pallet::<T>::block_number(),
					destination: bounded_destination.clone(),
				},
			);

			Self::deposit_event(Event::MessageSent {
				who,
				destination: bounded_destination,
				message: BoundedVec::try_from(message).unwrap(),
				message_id,
			});

			Ok(().into())
		}

		/// Execute a cross-chain function call.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::execute_cross_chain_call())]
		pub fn execute_cross_chain_call(
			origin: OriginFor<T>,
			destination: Vec<u8>,
			call_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(!destination.is_empty(), Error::<T>::InvalidDestination);

			ensure!(
				call_data.len() <= T::MaxMessageSize::get() as usize,
				Error::<T>::MessageTooLarge
			);

			Self::deposit_event(Event::CrossChainCallExecuted {
				who,
				destination: BoundedVec::try_from(destination).unwrap(),
				call_data: BoundedVec::try_from(call_data).unwrap(),
			});

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Generate a unique message ID using a static counter.
		fn generate_message_id(who: &T::AccountId, message: &[u8]) -> MessageId {
			let counter = MESSAGE_COUNTER.fetch_add(1, Ordering::SeqCst);
			
			let mut input = Vec::new();
			counter.encode_to(&mut input);
			who.encode_to(&mut input);
			message.encode_to(&mut input);
			
			let hash = frame::traits::BlakeTwo256::hash(&input);
			hash.into()
		}

		/// Queue a message for sending.
		fn queue_message(
			destination: &[u8],
			message: &[u8],
			sender: &T::AccountId,
		) -> Result<(), Error<T>> {
			let bounded_dest = BoundedVec::try_from(destination.to_vec())
				.map_err(|_| Error::<T>::InvalidDestination)?;

			let queued_message = QueuedMessage {
				message: BoundedVec::try_from(message.to_vec()).unwrap(),
				queued_at: frame_system::Pallet::<T>::block_number(),
				sender: sender.clone(),
			};

			QueuedMessages::<T>::try_mutate(bounded_dest, |messages| {
				messages.try_push(queued_message)
			})
			.map_err(|_| Error::<T>::QueueFull)?;

			Ok(())
		}

		/// Process an incoming message.
		pub fn process_incoming_message(
			from: &[u8],
			message: &[u8],
		) -> Result<(), Error<T>> {
			let message_id = Self::generate_message_id_from_incoming(from, message);

			let default_account = T::AccountId::decode(&mut &[0u8; 32][..])
				.unwrap_or_else(|_| {
					let account_bytes = [0u8; 32];
					T::AccountId::decode(&mut &account_bytes[..]).unwrap_or_else(|_| {
						panic!("Failed to create default account ID")
					})
				});

			MessageStatus::<T>::insert(
				(&default_account, message_id),
				MessageStatusInfo {
					status: MessageStatusType::Received,
					block_number: frame_system::Pallet::<T>::block_number(),
					destination: BoundedVec::try_from(from.to_vec()).unwrap(),
				},
			);

			Self::deposit_event(Event::MessageReceived {
				from: BoundedVec::try_from(from.to_vec()).unwrap(),
				message: BoundedVec::try_from(message.to_vec()).unwrap(),
				message_id,
			});

			Ok(())
		}

		/// Generate message ID from incoming message.
		fn generate_message_id_from_incoming(from: &[u8], message: &[u8]) -> MessageId {
			let counter = MESSAGE_COUNTER.fetch_add(1, Ordering::SeqCst);
			
			let mut input = Vec::new();
			counter.encode_to(&mut input);
			from.encode_to(&mut input);
			message.encode_to(&mut input);
			
			let hash = frame::traits::BlakeTwo256::hash(&input);
			hash.into()
		}
	}
}

/// Message ID type for tracking messages.
pub type MessageId = [u8; 32];

/// Information about the status of a message.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Debug)]
pub struct MessageStatusInfo<BlockNumber> {
	/// Current status of the message.
	pub status: MessageStatusType,
	/// Block number when the status was last updated.
	pub block_number: BlockNumber,
	/// Destination of the message.
	pub destination: BoundedVec<u8, ConstU32<32>>,
}

/// Types of message statuses.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Debug)]
pub enum MessageStatusType {
	/// Message has been sent.
	Sent,
	/// Message has been received.
	Received,
	/// Message execution failed.
	Failed,
	/// Message execution completed successfully.
	Completed,
}

/// A queued message waiting to be sent.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Debug)]
pub struct QueuedMessage<T: Config> {
	/// The message to be sent.
	pub message: BoundedVec<u8, ConstU32<1024>>,
	/// When the message was queued.
	pub queued_at: BlockNumberFor<T>,
	/// The sender of the message.
	pub sender: T::AccountId,
}
