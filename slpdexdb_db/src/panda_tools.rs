use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    result::Error as DieselError
};
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

use panda_base::traits::*;
use crate::models::*;

pub fn create_panda(
    genesis_tx: &i64, 
    owner_tx: &i64, 
    owner_tx_idx: &i64, 
    pa: &PandaAttributes, 
    pool: Pool
) -> Result<i64, DieselError> {
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
        mouth: &pa.mouth
    };
}