use cashcontracts::{UnsignedTx, UnsignedInput, Tx, TxOutpoint, TxOutput, Output, double_sha256,
                    single_sha256,
                    Address, AddressType, P2PKHOutput, SLPGenesis, SLPSend, OpReturnOutput,
                    tx_hash_to_hex};
use slpdexdb_base::SLPDEXConfig;
use crate::tx_history::TokenType;
use crate::data::tx_hash_from_le_slice;

pub struct PandaTx {
    pub nft1_outpoint: TxOutpoint,
    pub nft1_amount: u64,
    pub secret_key: secp256k1::SecretKey,
    pub fee_inputs: Vec<(TxOutpoint, u64)>,
    pub owner_address: Address,
    pub panda_ticker: String,
    pub panda_name: String,
    pub genome: Vec<u8>,
    pub fee_per_kb: u64,
    pub dust_limit: u64,
}

pub struct PND1Tx {
    pub secret_key: secp256k1::SecretKey,
    pub inputs: Vec<(TxOutpoint, u64)>,
    pub fee_per_kb: u64,
    pub dust_limit: u64,
    pub name: String,
    pub father_token: [u8; 32],
    pub father_tx_hash: [u8; 32],
    pub father_output_idx: u32,
    pub mother_token: [u8; 32],
    pub mother_tx_hash: [u8; 32],
    pub mother_output_idx: u32,
}

impl PandaTx {
    pub fn into_tx(self) -> Result<Tx, u64> {
        let curve = secp256k1::Secp256k1::new();
        let mut tx_build = UnsignedTx::new_simple();
        let secret_key = self.secret_key;
        let pub_key = secp256k1::PublicKey::from_secret_key(&curve, &secret_key).serialize().to_vec();
        let address = Address::from_serialized_pub_key("bitcoincash", AddressType::P2PKH, &pub_key);
        tx_build.add_input(UnsignedInput {
            outpoint: self.nft1_outpoint,
            output: Box::new(P2PKHOutput {
                address: address.clone(),
                value: self.nft1_amount,
            }),
            sequence: 0xffff_ffff,
        });
        for (outpoint, amount) in self.fee_inputs {
            tx_build.add_input(UnsignedInput {
                outpoint,
                output: Box::new(P2PKHOutput {
                    address: address.clone(),
                    value: amount,
                }),
                sequence: 0xffff_ffff,
            });
        }
        tx_build.add_output(TxOutput {
            value: 0,
            script: SLPGenesis {
                token_type: TokenType::NFT1Child as u8,
                token_ticker: self.panda_ticker.into_bytes(),
                token_name: self.panda_name.into_bytes(),
                token_document_url: format!("https://pand.as.cash/genome/{}", hex::encode(&self.genome)).into_bytes(),
                token_document_hash: vec![],
                decimals: 0,
                mint_baton_vout: None,
                initial_token_mint_quantity: 1,
            }.into_output().script(),
        });
        tx_build.add_output(TxOutput {
            value: self.dust_limit,
            script: P2PKHOutput {
                value: self.dust_limit,
                address: self.owner_address,
            }.script(),
        });
        tx_build.add_leftover_output(address.clone(), self.fee_per_kb, self.dust_limit)?;
        let pre_images = tx_build.pre_images(0x41);
        Ok(tx_build.sign(
            pre_images.iter().map(|pre_image| {
                let mut pre_image_ser = Vec::new();
                pre_image.write_to_stream(&mut pre_image_ser).unwrap();
                println!("preimage:{}", hex::encode(&pre_image_ser));
                let hash = double_sha256(&pre_image_ser);
                curve.sign(&secp256k1::Message::from_slice(&hash).unwrap(),
                           &secret_key).serialize_der().as_ref().to_vec()
            }).collect(),
            pre_images.iter().map(|_| pub_key.clone()).collect(),
        ))
    }
}

impl PND1Tx {
    pub fn into_tx(self, config: &SLPDEXConfig) -> Result<Tx, u64> {
        let curve = secp256k1::Secp256k1::new();
        let mut tx_build = UnsignedTx::new_simple();
        let secret_key = self.secret_key;
        let pub_key = secp256k1::PublicKey::from_secret_key(&curve, &secret_key).serialize().to_vec();
        let address = Address::from_serialized_pub_key("bitcoincash", AddressType::P2PKH, &pub_key);

        let message = format!("PANDA S3X:{}+{}",
                              tx_hash_to_hex(&self.father_token),
                              tx_hash_to_hex(&self.mother_token));
        let signature = curve.sign(&secp256k1::Message::from_slice(&single_sha256(message.as_bytes())).unwrap(), &secret_key);

        for (outpoint, amount) in self.inputs {
            tx_build.add_input(UnsignedInput {
                outpoint,
                output: Box::new(P2PKHOutput {
                    address: address.clone(),
                    value: amount,
                }),
                sequence: 0xffff_ffff,
            });
        }

        tx_build.add_output(TxOutput {
            value: 0,
            script: OpReturnOutput {
                is_minimal_push: false,
                pushes: vec![
                    b"PND1".to_vec(),
                    self.name.into_bytes(),
                    tx_hash_from_le_slice(&self.father_tx_hash).to_vec(),
                    self.father_output_idx.to_le_bytes().to_vec(),
                    tx_hash_from_le_slice(&self.mother_tx_hash).to_vec(),
                    self.mother_output_idx.to_le_bytes().to_vec(),
                    pub_key.clone(),
                    signature.serialize_der().to_vec(),
                ],
            }.script(),
        });

        tx_build.add_output(TxOutput {
            value: config.panda_fee,
            script: P2PKHOutput {
                value: 0,
                address: config.fee_address.clone(),
            }.script(),
        });

        tx_build.add_leftover_output(address.clone(), self.fee_per_kb, self.dust_limit)?;
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
}
