use snarkos_errors::objects::TransactionError;
use snarkos_utilities::bytes::{FromBytes, ToBytes};

use std::hash::Hash;

pub mod block;
pub use self::block::*;

pub mod ledger;
pub use self::ledger::*;

pub mod transactions;
pub use self::transactions::*;

pub trait Transaction: Clone + Eq + FromBytes + ToBytes {
    type SerialNumber: Clone + Eq + Hash + FromBytes + ToBytes;
    type Commitment: Clone + Eq + Hash + FromBytes + ToBytes;
    type Memorandum: Clone + Eq + Hash + FromBytes + ToBytes;
    type Stuff;

    /// Returns the old serial numbers.
    fn old_serial_numbers(&self) -> &[Self::SerialNumber];

    /// Returns the new commitments.
    fn new_commitments(&self) -> &[Self::Commitment];

    /// Returns the memorandum.
    fn memorandum(&self) -> &Self::Memorandum;

    /// Returns the stuff field.
    fn stuff(&self) -> &Self::Stuff;

    /// Returns the transaction identifier.
    fn transaction_id(&self) -> Result<[u8; 32], TransactionError>;
}
