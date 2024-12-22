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

pub fn action_input() -> Option<String> {
    std::env::var_os("ACTION_INPUT")
        .map(|x| x.to_string_lossy().into_owned())
        .filter(|x| !x.trim().is_empty())
}
