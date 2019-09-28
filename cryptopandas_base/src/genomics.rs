use rand::Rng;

/// Mixes parents genes. Parents genes are given by 48 5-bit integers, 
/// and represented by 48 bytes.
pub fn mix_genes(mut m_genes: [u8; 48], mut s_genes: [u8; 48]) -> [u8; 48] {
    let mut rng = rand::thread_rng(); // TODO: Use seed sourced from blockchain

    // Scramble parent genes
    for i in (0..48).step_by(4) {
        for j in (1..4).rev() {
            if rng.gen_range(0, 4) == 0 {
                m_genes.swap(i + j, i + j - 1);
            }
            if rng.gen_range(0, 4) == 0 {
                s_genes.swap(i + j, i + j - 1);
            }
        }
    }

    // Generate baby genes
    let mut baby_genes = [0; 48];
    for i in 0..48 {
        let mut mutation = 0;
        if i % 4 == 0 {
            // Set gene two > gene one
            let (gene_one, gene_two) = if m_genes[i] > s_genes[i] {
                (s_genes[i], m_genes[i])
            } else {
                (m_genes[i], s_genes[i])
            };

            // If genes are successive and gene one is even
            if gene_two - gene_one == 1 && gene_one % 2 == 0 {
                // Create mutation
                let mut sample_size = 4;
                if gene_one > 23 {
                    sample_size *= 2;
                }
                if rng.gen_range(0, sample_size) == 0 {
                    mutation = gene_one / 2 + 16;
                }
            }
        }

        // Assign baby gene
        if mutation != 0 {
            // Assign mutation to baby
            baby_genes[i] = mutation;
        } else if rng.gen_range(0, 2) == 0 {
            baby_genes[i] = m_genes[i];
        } else {
            baby_genes[i] = s_genes[i];
        }
    }
    baby_genes
}
