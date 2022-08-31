
// trailing bytes of the raw transaction.
// double Redeem, Replace or Refund
// https://pwning.mirror.xyz/jlT8OgtwN3mQf3KdYmXdcSXbE4s95JzT3eR3wxiLmpw
// https://github.com/interlay/interbtc/commit/3ef1476649697d6f7bbf785e39882ad965bd2ba8 report_vault_double_payment.
#[test]
fn is_equal() {
    // add any extra bytes to right raw_tx.
    let raw_txs = (0x01, 0x010);

    // they are not the same, pass this condition.
    assert_ne!(raw_txs.0, raw_txs.1);

    // parse_and_verify function not verify extra bytes like before.
    // let left_tx = parse_and_verify(&raw_txs.0, &raw_merkle_proofs.0)?;
    // let right_tx = parse_and_verify(&raw_txs.1, &raw_merkle_proofs.1)?;
    // let left_tx_id = left_tx.tx_id();
    // let right_tx_id = right_tx.tx_id();

    // transaction must be unique, so they must be equal. If not verify them, maybe it's vulunerable.
    // ensure!(left_tx_id != right_tx_id, Error::<T>::DuplicateTransaction);

}



#[macro_export]
macro_rules! balance {
    ($value:literal) => {{
        use $crate::fixnum::_priv::parse_fixed;
        const VALUE_SIGNED: i128 = parse_fixed(stringify!($value), 1_000_000_000_000_000_000);
        const VALUE: $crate::Balance = VALUE_SIGNED.abs() as u128;
        VALUE
    }};
}

#[test]
fn test_overflow() {
    let a = i32::MIN; // Error
    println!("i32 MIN: {}", a);
    let b = a - 1;
    println!("b: {}", b);
}
