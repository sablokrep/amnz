/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn hamming_distance(x: &[u8], y: &[u8]) -> u32 {
    let mut distance = x
        .chunks_exact(8)
        .zip(y.chunks_exact(8))
        .map(|(a, b)| {
            let xa = u64::from_ne_bytes(a.try_into().unwrap());
            let xb = u64::from_ne_bytes(b.try_into().unwrap());
            (xa ^ xb).count_ones()
        })
        .sum::<u32>();
    distance += x
        .chunks_exact(8)
        .remainder()
        .iter()
        .zip(y.chunks_exact(8).remainder())
        .map(|(&a, &b)| (a ^ b).count_ones() as u32)
        .sum::<u32>();
    distance
}
