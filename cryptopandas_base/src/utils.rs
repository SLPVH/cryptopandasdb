use bitvec::prelude::*;
use std::convert::TryInto;

pub fn pack_genes(genes: &[u8; 48]) -> [u8; 32] {
    let gene_bv = BitVec::<BigEndian, u8>::from_slice(&genes[..]);
    let mut bv = BitVec::<BigEndian, u8>::with_capacity(32);
    for i in 0..48 {
        let b_slice = &gene_bv[3 + 8 * i..8 * (i + 1)][..];
        bv.extend(b_slice);
    }
    let padding = BitVec::<BigEndian, u8>::from_slice(&[0, 0][..]);
    bv.extend(&padding[..]);
    (&bv.into_vec()[..]).try_into().unwrap()
}

pub fn unpack_genes(genes: &[u8; 32]) -> [u8; 48] {
    let packed_bv = BitVec::<BigEndian, u8>::from_slice(&genes[..]);
    let mut unpacked_bv = BitVec::<BigEndian, u8>::with_capacity(48);
    for i in 0..48 {
        let b_slice = &packed_bv[5 * i..5 * (i + 1)][..];
        unpacked_bv.push(false);
        unpacked_bv.push(false);
        unpacked_bv.push(false);
        unpacked_bv.extend(b_slice);
    }
    let mut output = [0; 48];
    for (i, byte) in unpacked_bv.into_vec().iter().enumerate() {
        output[i] = *byte;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pack_unpack() {
        let genes = [1; 48];
        assert_eq!(&unpack_genes(&pack_genes(&genes))[..], &genes[..])
    }

    #[test]
    fn pack_sanity() {
        let genes = [31; 48];
        let packed = pack_genes(&genes);
        println!("{:?}", &packed[..]);

        assert_eq!(&packed[..30], &[255; 30][..])
    }

    #[test]
    fn unpack_sanity() {
        let packed_genes = [255; 32];
        let unpacked = unpack_genes(&packed_genes);
        println!("{:?}", &unpacked[..]);

        assert_eq!(&unpacked[..], &[31; 48][..])
    }
}
