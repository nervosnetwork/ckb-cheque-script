use core::result::Result;

use super::helper;
use crate::error::Error;
use ckb_std::{ckb_constants::Source, high_level::load_script_hash};

pub fn validate(
    sender_lock_hash: &[u8; 20],
    receiver_lock_hash: &[u8; 20],
    cheque_witness_is_none: bool,
) -> Result<(), Error> {
    let script_hash = load_script_hash()?;
    let mut cheque_lock_hash = [0u8; 20];
    cheque_lock_hash.copy_from_slice(&script_hash[0..20]);

    if check_cheque_inputs_since_not_zero() {
        return Err(Error::ClaimChequeInputSinceNotZero);
    }

    if !check_sender_cells_capacity_same(sender_lock_hash, &cheque_lock_hash) {
        return Err(Error::SenderCapacityNotSame);
    }

    if cheque_witness_is_none {
        match helper::position_input_by_lock_hash(receiver_lock_hash) {
            Some(position) => helper::check_witness_args(position),
            None => Err(Error::NoMatchedInputs),
        }
    } else {
        Ok(())
    }
}

fn check_sender_cells_capacity_same(
    sender_lock_hash: &[u8; 20],
    cheque_lock_hash: &[u8; 20],
) -> bool {
    let sender_inputs = helper::filter_cells_by_lock_hash(sender_lock_hash, Source::Input);
    let sum_sender_inputs_capacity = helper::calc_cells_capacity_sum(sender_inputs);

    let sender_outputs = helper::filter_cells_by_lock_hash(sender_lock_hash, Source::Output);
    let sum_sender_outputs_capacity = helper::calc_cells_capacity_sum(sender_outputs);
    
    let diff_capacity_of_sender_outputs_and_inputs = sum_sender_outputs_capacity - sum_sender_inputs_capacity;

    let cheque_inputs = helper::filter_cells_by_lock_hash(cheque_lock_hash, Source::Input);
    let sum_cheque_inputs_capacity = helper::calc_cells_capacity_sum(cheque_inputs);

    sum_cheque_inputs_capacity == diff_capacity_of_sender_outputs_and_inputs
}

fn check_cheque_inputs_since_not_zero() -> bool {
    let cheque_inputs_since = helper::load_group_inputs_since();
    cheque_inputs_since.into_iter().any(|since| since != 0)
}
