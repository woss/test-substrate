#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, traits::Get, StorageMap,
};
// use serde::{Deserialize, Serialize};
// use serde_json;
use frame_system::ensure_signed;
use sp_runtime::traits::{Hash, MaybeSerializeDeserialize};
use sp_std::{fmt::Debug, vec::Vec};

/// The pallet's configuration trait.
pub trait Trait: frame_system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    /// Generic rule trait
    type Rule: Codec + Default + Copy + MaybeSerializeDeserialize + Debug;
    /// test for the get rule
    type GetRule: Get<Self::Rule>;
}

/// List of equipment that needs rules generated
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq)]
enum ForWhat {
    /// Any photo
    Photo = 0,
    /// Any camera, not a smartphone
    Camera = 1,
    /// Any Lens
    Lens = 2,
    /// Any Smartphone
    SmartPhone = 3,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Rule<Vec> {
    rules: Vec, // parsable into JSON, what would be the best way to store it, encode the bytes?
    version: u8, // 1,2,3
    for_what: Vec, // a string, similar to rule
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Proof<AccountId, Hash, BlockNumber> {
    account_id: AccountId,
    block_number: BlockNumber,
    rules: Hash,
    proof: Hash,
    content_hash: Hash,
    photo: bool,
}

// implement default
impl<A, H, B> Default for Proof<A, H, B>
where
    A: Default,
    H: Default,
    B: Default,
{
    fn default() -> Self {
        Proof {
            account_id: A::default(),
            block_number: B::default(),
            proof: H::default(),
            rules: H::default(),
            content_hash: H::default(),
            photo: true,
        }
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

         /// Rules for the PoE
        const rule: Vec<u8> = (b"sdadsadasd").encode();
        // const rule: Option<str> = Rule {
        //     rules: (b"[2,3,4,5,6]").encode(), // parsable into JSON, what would be the best way to store it, encode the bytes?
        //     version: 1,    // 1,2,3
        //     for_what: (b"photo").encode(), // a string, similar to rule
        // };
        //
        // const j = json!({
        //     "code": 200,
        //     "success": true,
        //     "payload": {
        //         "features": [
        //             "serde",
        //             "json"
        //         ]
        //     }
        // });

        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;

        // Initializing events
        // this is needed only if you are using events in your pallet
        fn deposit_event() = default;

         /// Allow a user to claim ownership of an unclaimed proof
        fn create_claim(origin, proof: Vec<u8>) {
            // Verify that the incoming transaction is signed and store who the
            // caller of this function is.
            let sender = ensure_signed(origin)?;
            // let nonce = Self::nonce();
            // Verify that the specified proof has not been claimed yet or error with the message
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);


            let data_hash =<T as frame_system::Trait>::Hashing::hash(b"sdadsadasd");

            // Call the `system` pallet to get the current block number
            let current_block = <frame_system::Module<T>>::block_number();
            //
            // let p = Proof {
            //     account_id : sender.clone(),
            //     block_number: current_block,
            //     proof: proof.clone(),
            //     rules: "uri:ipfs:QM....".as_bytes().to_vec(),
            //     content_hash: data_hash.encode(),
            //     photo: true,
            // };
            //
            //
            // // Store the proof with the sender and the current block number
            // <Proofs::<T>>::insert(&proof, p);
            <Proofs::<T>>::insert(&proof, (sender.clone(), current_block, data_hash));
            //
            // // Emit an event that the claim was created
            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }


        /// Allow the owner to revoke their claim
        fn revoke_claim(origin, proof: Vec<u8>) {
            // Determine who is calling the function
            let sender = ensure_signed(origin)?;
            //
            // // Verify that the specified proof has been claimed
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
            //
            // // Get owner of the claim
            // let (owner, _, _) = Proofs::<T>::get(&proof);
            //
            // // Verify that sender of the current call is the claim owner
            // ensure!(sender == owner, Error::<T>::NotProofOwner);
            //
            // // Remove claim from storage
            // Proofs::<T>::remove(&proof);
            //
            // // Emit an event that the claim was erased
            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
        }
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        /// Just a dummy event.
        /// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
        /// To emit this event, we call the deposit function, from our runtime functions
        // SomethingStored(u32, AccountId),
        /// Event emitted when a proof has been claimed.
        ClaimCreated(AccountId, Vec<u8>),
        /// Event emitted when a claim is revoked by the owner.
        ClaimRevoked(AccountId, Vec<u8>),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Value was None
        NoneValue,
        /// Value reached maximum and cannot be incremented further
        StorageOverflow,
         /// This proof has already been claimed
        ProofAlreadyClaimed,
        /// The proof does not exist, so it cannot be revoked
        NoSuchProof,
        /// The proof is claimed by another account, so caller can't revoke it
        NotProofOwner,
    }
}

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.

    trait Store for Module<T: Trait> as PoEModule
    {
        // https://github.com/paritytech/substrate/blob/c34e0641abe52249866b62fdb0c2aeed41903be4/frame/support/procedural/src/lib.rs#L132
         Proofs2: map hasher(blake2_128_concat) Vec<u8> => Proof<T::AccountId, T::Hash, T::BlockNumber>;
         Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, T::Hash);

    }
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
