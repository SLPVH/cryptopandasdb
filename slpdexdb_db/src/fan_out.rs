use cashcontracts::{UnsignedTx, UnsignedInput, Tx, Address, TxOutpoint, TxOutput, P2PKHOutput,
                    SLPSend, Output, double_sha256, AddressType};
use slpdexdb_base::SLPAmount;

pub fn slp_fan_out_tx(secret: &[u8],
                      inputs: &[(TxOutpoint, u64)],
                      token_id: [u8; 32],
                      token_type: u8,
                      n_outputs: usize,
                      slp_amount_each: SLPAmount,
                      bch_amount_each: u64,
                      dust_limit: u64,
                      fee_per_kb: u64) -> Result<Tx, u64> {
    let curve = secp256k1::Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(secret).unwrap();
    let pub_key = secp256k1::PublicKey::from_secret_key(&curve, &secret_key).serialize().to_vec();
    let address = Address::from_serialized_pub_key("bitcoincash", AddressType::P2PKH, &pub_key);

    let mut tx_build = UnsignedTx::new_simple();
    for (outpoint, amount) in inputs {
        tx_build.add_input(UnsignedInput {
            outpoint: outpoint.clone(),
            output: Box::new(P2PKHOutput {
                value: *amount,
                address: address.clone(),
            }),
            sequence: 0xffff_ffff,
        });
    }
    tx_build.add_output(TxOutput {
        value: 0,
        script: SLPSend {
            token_id,
            token_type,
            output_quantities: (0..n_outputs).into_iter()
                .map(|_| slp_amount_each.base_amount() as u64)
                .collect(),
        }.into_output().script(),
    });
    for _ in 0..n_outputs {
        tx_build.add_output(TxOutput {
            value: bch_amount_each,
            script: P2PKHOutput {
                value: 0,
                address: address.clone(),
            }.script(),
        });
    }
    tx_build.add_leftover_output(address.clone(), fee_per_kb, dust_limit)?;
    let pre_images = tx_build.pre_images(0x41);
    Ok(tx_build.sign(
        pre_images.iter().map(|pre_image| {
            let mut pre_image_ser = Vec::new();
            pre_image.write_to_stream(&mut pre_image_ser).unwrap();
            let hash = double_sha256(&pre_image_ser);
            curve.sign(&secp256k1::Message::from_slice(&hash).unwrap(),
                       &secret_key).serialize_der().as_ref().to_vec()
        }).collect(),
        pre_images.iter().map(|_| pub_key.clone()).collect(),
    ))
}
