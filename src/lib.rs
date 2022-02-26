use metaplex_token_metadata::state::{Key, Metadata};
use pyo3::prelude::*;
use serde::Serialize;
use serde_json::json;
use solana_program::borsh::try_from_slice_unchecked;

#[derive(Debug, Serialize)]
pub struct JSONCreator {
    pub address: String,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

#[pyfunction]
fn deserialize_metadata(base58_string: String) -> PyResult<String> {
    let decoded: Vec<u8> = bs58::decode(base58_string)
        .into_vec()
        .expect("Failed to decode base58 string");

    let m: Metadata = try_from_slice_unchecked(&decoded).unwrap();

    let creators = m
        .data
        .creators
        .unwrap()
        .iter()
        .map(|c| JSONCreator {
            address: c.address.to_string(),
            verified: c.verified,
            share: c.share,
        })
        .collect::<Vec<JSONCreator>>();

    let data = json!({
        "name": m.data.name.to_string().trim_matches(char::from(0)),
        "symbol": m.data.symbol.to_string().trim_matches(char::from(0)),
        "seller_fee_basis_points": m.data.seller_fee_basis_points,
        "uri": m.data.uri.to_string().trim_matches(char::from(0)),
        "creators": [creators],
    });

    let metadata = json!({
        "key": key_to_string(m.key).trim_matches(char::from(0)),
        "update_authority": m.update_authority.to_string().trim_matches(char::from(0)),
        "mint": m.mint.to_string().trim_matches(char::from(0)),
        "data": data,
        "primary_sale_happened": m.primary_sale_happened,
        "is_mutable": m.is_mutable,
        "edition_nonce": m.edition_nonce,
    });

    Ok(metadata.to_string())
}

fn key_to_string(key: Key) -> String {
    match key {
        Key::Uninitialized => "Uninitialized".to_string(),
        Key::EditionV1 => "EditionV1".to_string(),
        Key::MasterEditionV1 => "MasterEditionV1".to_string(),
        Key::ReservationListV1 => "ReservationListV1".to_string(),
        Key::MetadataV1 => "MetadataV1".to_string(),
        Key::ReservationListV2 => "ReservationListV2".to_string(),
        Key::MasterEditionV2 => "MasterEditionV2".to_string(),
        Key::EditionMarker => "EditionMarker".to_string(),
    }
}

#[pymodule]
fn metaplex_decoder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(deserialize_metadata, m)?)?;

    Ok(())
}
