// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Transaction execution format module.
//pub use petgraph::Graph;
use crate::cita_db::trie;
use crate::receipt::ReceiptError;
use crate::trace::{FlatTrace, VMTrace};
use crate::types::log_entry::LogEntry;
use crate::types::state_diff::StateDiff;
use cita_types::{Address, U256, U512};
use evm;
use std::fmt;
use util::Bytes;

/// Transaction execution receipt.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ipc", binary)]
pub struct Executed {
    /// True if the outer call/create resulted in an exceptional exit.
    pub exception: Option<evm::Error>,

    /// Gas paid up front for execution of transaction.
    pub gas: U256,

    /// Gas used during execution of transaction.
    pub gas_used: U256,

    /// Gas refunded after the execution of transaction.
    /// To get gas that was required up front, add `refunded` and `gas_used`.
    pub refunded: U256,

    /// Cumulative gas used in current block so far.
    ///
    /// `cumulative_gas_used = gas_used(t0) + gas_used(t1) + ... gas_used(tn)`
    ///
    /// where `tn` is current transaction.
    pub cumulative_gas_used: U256,

    /// Vector of logs generated by transaction.
    pub logs: Vec<LogEntry>,

    /// Addresses of contracts created during execution of transaction.
    /// Ordered from earliest creation.
    ///
    /// eg. sender creates contract A and A in constructor creates contract B
    ///
    /// B creation ends first, and it will be the first element of the vector.
    pub contracts_created: Vec<Address>,
    /// Transaction output.
    pub output: Bytes,
    /// The trace of this transaction.
    pub trace: Vec<FlatTrace>,
    /// The VM trace of this transaction.
    pub vm_trace: Option<VMTrace>,
    /// The state diff, if we traced it.
    pub state_diff: Option<StateDiff>,
    /// Transaction sender account nonce
    pub account_nonce: U256,
}

/// Result of executing the transaction.
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "ipc", binary)]
pub enum ExecutionError {
    /// Returned when there gas paid for transaction execution is
    /// lower than base gas required.
    NotEnoughBaseGas {
        /// Absolute minimum gas required.
        required: U256,
        /// Gas provided.
        got: U256,
    },
    /// Returned when block (gas_used + gas) > gas_limit.
    ///
    /// If gas =< gas_limit, upstream may try to execute the transaction
    /// in next block.
    BlockGasLimitReached {
        /// Gas limit of block for transaction.
        gas_limit: U256,
        /// Gas used in block prior to transaction.
        gas_used: U256,
        /// Amount of gas in block.
        gas: U256,
    },
    AccountGasLimitReached {
        /// Account Gas limit left
        gas_limit: U256,
        /// Amount of gas in transaction
        gas: U256,
    },
    /// Returned when transaction nonce does not match state nonce.
    InvalidNonce {
        /// Nonce expected.
        expected: U256,
        /// Nonce found.
        got: U256,
    },
    /// Returned when cost of transaction (value + gas_price * gas) exceeds
    /// current sender balance.
    NotEnoughCash {
        /// Minimum required balance.
        required: U512,
        /// Actual balance.
        got: U512,
    },
    NoTransactionPermission,
    NoContractPermission,
    NoCallPermission,
    /// Returned when internal evm error occurs.
    ExecutionInternal(String),
    /// Returned when generic transaction occurs
    TransactionMalformed(String),
}

impl From<Box<trie::TrieError>> for ExecutionError {
    fn from(err: Box<trie::TrieError>) -> Self {
        ExecutionError::ExecutionInternal(format!("{}", err))
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ExecutionError::*;

        let msg = match *self {
            NotEnoughBaseGas { ref required, ref got } => format!("Not enough base quota. {} is required, but only {} paid", required, got),
            BlockGasLimitReached {
                ref gas_limit,
                ref gas_used,
                ref gas,
            } => format!("Block quota limit reached. The limit is {}, {} has already been used, and {} more is required", gas_limit, gas_used, gas),
            AccountGasLimitReached { ref gas_limit, ref gas } => format!("Account quota limit reached. The limit is {}, {} more is required", gas_limit, gas),
            InvalidNonce { ref expected, ref got } => format!("Invalid transaction nonce: expected {}, found {}", expected, got),
            NotEnoughCash { ref required, ref got } => format!("Cost of transaction exceeds sender balance. {} is required but the sender only has {}", required, got),
            ExecutionInternal(ref msg) => msg.clone(),
            TransactionMalformed(ref err) => format!("Malformed transaction: {}", err),
            NoTransactionPermission => "No transaction permission".to_owned(),
            NoContractPermission => "No contract permission".to_owned(),
            NoCallPermission => "No call contract permission".to_owned(),
        };

        f.write_fmt(format_args!("Transaction execution error ({}).", msg))
    }
}

/// Result of executing the transaction.
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "ipc", binary)]
pub enum CallError {
    /// Couldn't find the transaction in the chain.
    TransactionNotFound,
    /// Couldn't find requested block's state in the chain.
    StatePruned,
    /// Couldn't find an amount of gas that didn't result in an exception.
    Exceptional,
    /// Corrupt state.
    StateCorrupt,
    /// Error executing.
    Execution(ExecutionError),
}

impl From<ExecutionError> for CallError {
    fn from(error: ExecutionError) -> Self {
        CallError::Execution(error)
    }
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CallError::*;

        let msg = match *self {
            TransactionNotFound => "Transaction couldn't be found in the chain".into(),
            StatePruned => "Couldn't find the transaction block's state in the chain".into(),
            Exceptional => "An exception happened in the execution".into(),
            StateCorrupt => "Stored state found to be corrupted.".into(),
            Execution(ref e) => format!("{}", e),
        };

        f.write_fmt(format_args!("Transaction execution error ({}).", msg))
    }
}

/// Transaction execution result.
pub type ExecutionResult = Result<Executed, ExecutionError>;

impl From<ExecutionError> for ReceiptError {
    fn from(error: ExecutionError) -> Self {
        match error {
            ExecutionError::NotEnoughBaseGas { .. } => ReceiptError::NotEnoughBaseQuota,
            ExecutionError::BlockGasLimitReached { .. } => ReceiptError::BlockQuotaLimitReached,
            ExecutionError::AccountGasLimitReached { .. } => ReceiptError::AccountQuotaLimitReached,
            ExecutionError::InvalidNonce { .. } => ReceiptError::InvalidNonce,
            ExecutionError::NotEnoughCash { .. } => ReceiptError::NotEnoughCash,
            ExecutionError::NoTransactionPermission => ReceiptError::NoTransactionPermission,
            ExecutionError::NoContractPermission => ReceiptError::NoContractPermission,
            ExecutionError::NoCallPermission => ReceiptError::NoCallPermission,
            ExecutionError::ExecutionInternal { .. } => ReceiptError::ExecutionInternal,
            ExecutionError::TransactionMalformed { .. } => ReceiptError::TransactionMalformed,
        }
    }
}
