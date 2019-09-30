use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    result::Error as DieselError
};

use panda_base::traits::*;
use crate::{models::*, schema};

pub fn insert_panda_from_traits(
    genesis_tx: &i64, 
    owner_tx: &i64, 
    owner_tx_idx: &i32,
    panda_traits: &PandaTraits,
    secret_genes: &[u8; 12],
    conn: &PgConnection
) -> Result<i64, DieselError> {
    use self::schema::panda::dsl as panda_dsl;

    // Pull panda attributes from traits
    let pa = panda_traits.to_attributes();
    
    // Get public genes
    let public_genes = panda_traits.to_byte_public_genes();

    // Extend public genes
    let genes_full_vec = &[&public_genes[..], &secret_genes[..]]
        .concat();
    let mut genes_full = [0; 48];
    genes_full.copy_from_slice(genes_full_vec);

    // Create panda row
    let new_panda = NewPanda {
        genesis_tx,
        owner_tx,
        owner_tx_idx,
        physique: &pa.physique,
        pattern: &pa.pattern,
        eye_color: &pa.eye_color,
        eye_shape: &pa.eye_shape,
        base_color: &pa.base_color,
        highlight_color: &pa.highlight_color,
        accent_color: &pa.accent_color,
        wild_element: &pa.wild_element,
        mouth: &pa.mouth,
        genes: &genes_full[..]
    };

    // Insert record
    diesel::insert_into(panda_dsl::panda)
        .values(&new_panda)
        .returning(panda_dsl::id)
        .get_results(conn).map(|res_vec| res_vec[0])  
}

pub fn insert_panda_from_genes(
    genesis_tx: &i64, 
    owner_tx: &i64, 
    owner_tx_idx: &i32,
    genes: &[u8; 48],
    conn: &PgConnection
) -> Result<i64, DieselError> {
    use self::schema::panda::dsl as panda_dsl;

    // Create attributes
    let pa = PandaAttributes::from_genes(genes).unwrap(); // TODO: ? error here

    // Create new panda
    let new_panda = NewPanda {
        genesis_tx,
        owner_tx,
        owner_tx_idx,
        physique: &pa.physique,
        pattern: &pa.pattern,
        eye_color: &pa.eye_color,
        eye_shape: &pa.eye_shape,
        base_color: &pa.base_color,
        highlight_color: &pa.highlight_color,
        accent_color: &pa.accent_color,
        wild_element: &pa.wild_element,
        mouth: &pa.mouth,
        genes: &genes[..]
    };

    // Insert record
    diesel::insert_into(panda_dsl::panda)
        .values(&new_panda)
        .returning(panda_dsl::id)
        .get_results(conn).map(|res_vec| res_vec[0])  
}

pub fn get_panda_by_id(panda_id: &i64, conn: &PgConnection) -> Result<DbPanda, DieselError> {
    use self::schema::panda::dsl as panda_dsl;
    panda_dsl::panda
        .filter(panda_dsl::id.eq(panda_id))
        .select((
            panda_dsl::id,
            panda_dsl::genesis_tx,
            panda_dsl::owner_tx,
            panda_dsl::owner_tx_idx,
            panda_dsl::genes))
        .first(conn)
}

pub fn get_panda_by_token_id(token_id: &[u8], conn:&PgConnection) -> Result<DbPanda, DieselError> {
    use self::schema::{panda::dsl as panda_dsl, tx::dsl as tx_dsl};

    panda_dsl::panda
        .inner_join(tx_dsl::tx)
        .filter(tx_dsl::hash.eq(token_id))
        .select((
            panda_dsl::id,
            panda_dsl::genesis_tx,
            panda_dsl::owner_tx,
            panda_dsl::owner_tx_idx,
            panda_dsl::genes))
        .first::<DbPanda>(conn)
}

pub fn get_full_panda_by_token_id(token_id: &[u8], conn:&PgConnection) -> Result<DbPandaFull, DieselError> {
    use self::schema::{panda::dsl as panda_dsl, tx::dsl as tx_dsl, tx_output::dsl as output_dsl};

    output_dsl::tx_output
        .inner_join(tx_dsl::tx)
        .inner_join(panda_dsl::panda.on(
            panda_dsl::owner_tx.eq(output_dsl::tx).and(
                panda_dsl::owner_tx_idx.eq(output_dsl::idx))
        ))
        .filter(tx_dsl::hash.eq(token_id))
        .select((
            tx_dsl::hash,
            output_dsl::address,
            panda_dsl::physique,
            panda_dsl::pattern,
            panda_dsl::eye_color,
            panda_dsl::eye_shape,
            panda_dsl::base_color,
            panda_dsl::highlight_color,
            panda_dsl::accent_color,
            panda_dsl::wild_element,
            panda_dsl::mouth,
            panda_dsl::genes))
        .first::<DbPandaFull>(conn)
}

pub fn get_panda_by_addr(address: &[u8], conn:&PgConnection) -> Result<Vec<DbPanda>, DieselError> {
    use self::schema::{panda::dsl as panda_dsl, tx_output::dsl as output_dsl};

    output_dsl::tx_output
        .inner_join(panda_dsl::panda.on(
            panda_dsl::owner_tx.eq(output_dsl::tx).and(
                panda_dsl::owner_tx_idx.eq(output_dsl::idx))
        ))
        .filter(output_dsl::address.eq(Some(address)))
        .select((
            panda_dsl::id,
            panda_dsl::genesis_tx,
            panda_dsl::owner_tx,
            panda_dsl::owner_tx_idx,
            panda_dsl::genes))
        .load::<DbPanda>(conn)
}

pub fn get_full_panda_by_addr(address: &[u8], conn:&PgConnection) -> Result<Vec<DbPandaFull>, DieselError> {
    use self::schema::{panda::dsl as panda_dsl, tx::dsl as tx_dsl, tx_output::dsl as output_dsl};

    output_dsl::tx_output
        .inner_join(tx_dsl::tx)
        .inner_join(panda_dsl::panda.on(
            panda_dsl::owner_tx.eq(output_dsl::tx).and(
                panda_dsl::owner_tx_idx.eq(output_dsl::idx))
        ))
        .filter(output_dsl::address.eq(Some(address)))
        .select((
            tx_dsl::hash,
            output_dsl::address,
            panda_dsl::physique,
            panda_dsl::pattern,
            panda_dsl::eye_color,
            panda_dsl::eye_shape,
            panda_dsl::base_color,
            panda_dsl::highlight_color,
            panda_dsl::accent_color,
            panda_dsl::wild_element,
            panda_dsl::mouth,
            panda_dsl::genes))
        .load::<DbPandaFull>(conn)
}

pub fn get_active_addresses(conn:&PgConnection) -> Result<Vec<Option<Vec<u8>>>, DieselError> {
    use self::schema::{panda::dsl as panda_dsl, tx_output::dsl as output_dsl};

    output_dsl::tx_output
        .inner_join(panda_dsl::panda.on(
            panda_dsl::owner_tx.eq(output_dsl::tx).and(
                panda_dsl::owner_tx_idx.eq(output_dsl::idx))
        ))
        .select(output_dsl::address)
        .load::<Option<Vec<u8>>>(conn)
}

pub fn get_pandas_by_ids(panda_ids: Vec<i64>, conn: &PgConnection) -> Result<Vec<DbPanda>, DieselError> {
    use self::schema::panda::dsl as panda_dsl;
    panda_dsl::panda
        .filter(panda_dsl::id.eq_any(panda_ids))
        .select((
            panda_dsl::id,
            panda_dsl::genesis_tx,
            panda_dsl::owner_tx,
            panda_dsl::owner_tx_idx,
            panda_dsl::genes))
        .load(conn)
}

pub fn get_panda_by_owner_utxo(owner_tx_id: i64, owner_output_idx: i32, conn: &PgConnection) -> Result<Option<DbPanda>, DieselError> {
    use self::schema::panda::dsl as panda_dsl;
    panda_dsl::panda
        .filter(
            panda_dsl::owner_tx.eq(owner_tx_id)
                .and(panda_dsl::owner_tx_idx.eq(owner_output_idx))
        )
        .select((
            panda_dsl::id,
            panda_dsl::genesis_tx,
            panda_dsl::owner_tx,
            panda_dsl::owner_tx_idx,
            panda_dsl::genes))
        .first::<DbPanda>(conn)
        .optional()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_read() {
        // This test requires a tx_output with key (1, 0)
        let connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let connection = PgConnection::establish(&connection_str).unwrap();
        for i in 0..32 {
            let genes_expected = [i; 48];
            let id = insert_panda_from_genes(&1, &1, &0, &genes_expected, &connection).unwrap();
            let db_panda = get_panda_by_id(&id, &connection).unwrap();
            let genes_actual = db_panda.genes();
            assert_eq!(&genes_expected[..], &genes_actual[..]);
        }
    }
}