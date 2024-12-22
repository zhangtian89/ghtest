use rand::RngCore;

#[inline(always)]
pub fn gen_vec(n: usize) -> Vec<u8> {
    let mut data = vec![0u8; n];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut data);
    data
}

#[inline(always)]
pub fn gen_array<const N: usize>() -> [u8; N] {
    let mut data = [0u8; N];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut data);
    data
}
