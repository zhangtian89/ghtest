mod _utils;
use _utils::{bench_chunk_util::bench_chunk, *};

const ASSOCIATED: &[u8] = b"ASSOCIATED data";

macro_rules! encrypt_template {
    ($cipher:expr, $nonce:expr) => {
        |mut x| {
            encrypt_template!(inner: $cipher, $nonce, &mut x);
            x
        }
    };
    (inner: $cipher:expr, $nonce:expr, $buf:expr) => {
        $cipher.encrypt_in_place(&($nonce), ASSOCIATED, ($buf)).unwrap()
    };
}

macro_rules! encrypt_init_template {
    ($cipher:expr, $nonce:expr) => {
        |size| {
            let mut data = gen_vec(size);
            encrypt_template!(inner: $cipher, $nonce, &mut data);
            data
        }
    };
}

macro_rules! decrypt_template {
    ($cipher:expr, $nonce:expr) => {
        |mut x| {
            decrypt_template!(inner: $cipher, $nonce, &mut x);
            x
        }
    };
    (inner: $cipher:expr, $nonce:expr, $buf:expr) => {
        $cipher.decrypt_in_place(&($nonce), ASSOCIATED, $buf).unwrap();
    };
}

fn bench_aes(c: &mut Criterion) {
    use aes_gcm::{
        aes::{Aes128, Aes256},
        AeadInPlace, Aes128Gcm, Aes256Gcm, Key, KeyInit, Nonce,
    };

    let mut group = c.benchmark_group("AesGcm");
    group.sample_size(20);

    let nonce: [_; 12] = gen_array();
    let nonce = Nonce::from_slice(&nonce);

    let key: [_; 16] = gen_array();
    let key = Key::<Aes128>::from_slice(&key);
    let cipher = Aes128Gcm::new(key);
    bench_chunk(
        &mut group,
        "Aes128Gcm encrypt_in_place",
        |size| gen_vec(size),
        encrypt_template!(cipher, nonce),
    );
    bench_chunk(
        &mut group,
        "Aes128Gcm decrypt_in_place",
        encrypt_init_template!(cipher, nonce),
        decrypt_template!(cipher, nonce),
    );

    let key: [_; 32] = gen_array();
    let key = Key::<Aes256>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);
    bench_chunk(
        &mut group,
        "Aes256Gcm encrypt_in_place",
        |size| gen_vec(size),
        encrypt_template!(cipher, nonce),
    );
    bench_chunk(
        &mut group,
        "Aes256Gcm decrypt_in_place",
        encrypt_init_template!(cipher, nonce),
        decrypt_template!(cipher, nonce),
    );

    group.finish();
}

fn bench_chacha20poly1305(c: &mut Criterion) {
    use chacha20poly1305::{
        aead::{AeadCore, AeadInPlace, KeyInit, OsRng},
        ChaCha20Poly1305, XChaCha20Poly1305,
    };

    let mut group = c.benchmark_group("ChaCha20");
    group.sample_size(20);

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    bench_chunk(
        &mut group,
        "ChaCha20Poly1305 encrypt_in_place",
        |size| gen_vec(size),
        encrypt_template!(cipher, nonce),
    );
    bench_chunk(
        &mut group,
        "ChaCha20Poly1305 decrypt_in_place",
        encrypt_init_template!(cipher, nonce),
        decrypt_template!(cipher, nonce),
    );

    let key = XChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    bench_chunk(
        &mut group,
        "XChaCha20Poly1305 encrypt_in_place",
        |size| gen_vec(size),
        encrypt_template!(cipher, nonce),
    );
    bench_chunk(
        &mut group,
        "XChaCha20Poly1305 decrypt_in_place",
        encrypt_init_template!(cipher, nonce),
        decrypt_template!(cipher, nonce),
    );

    group.finish();
}

fn bench_salsa20(c: &mut Criterion) {
    use salsa20::cipher::{KeyIvInit, StreamCipher};
    use salsa20::Salsa20;

    let mut group = c.benchmark_group("Salsa20");
    group.sample_size(20);

    let key: [_; 32] = gen_array();
    let nonce: [_; 8] = gen_array();
    let mut cipher = Salsa20::new(&key.into(), &nonce.into());
    bench_chunk(
        &mut group,
        "Salsa20",
        |size| gen_vec(size),
        |mut x| {
            cipher.apply_keystream(&mut x);
            x
        },
    );

    group.finish();
}

fn bench_sm4(c: &mut Criterion) {
    use sm4::{
        cipher::{BlockEncrypt, KeyInit},
        Sm4,
    };

    let mut group = c.benchmark_group("Sm4");
    group.sample_size(20);

    let key = gen_array::<16>();
    let cipher = Sm4::new(&key.into());
    bench_chunk(
        &mut group,
        "Sm4",
        |size| gen_vec(size),
        |mut x| {
            for block in x.chunks_exact_mut(16) {
                cipher.encrypt_block(block.into());
            }
            x
        },
    );

    group.finish();
}

fn bench_camellia(c: &mut Criterion) {
    use camellia::{
        cipher::{BlockEncrypt, KeyInit},
        Camellia128, Camellia256,
    };

    let mut group = c.benchmark_group("Camellia");
    group.sample_size(20);

    let key = gen_array::<16>();
    let cipher = Camellia128::new(&key.into());
    bench_chunk(
        &mut group,
        "Camellia128",
        |size| gen_vec(size),
        |mut x| {
            for block in x.chunks_exact_mut(16) {
                cipher.encrypt_block(block.into());
            }
            x
        },
    );

    let key = gen_array::<32>();
    let cipher = Camellia256::new(&key.into());
    bench_chunk(
        &mut group,
        "Camellia256",
        |size| gen_vec(size),
        |mut x| {
            for block in x.chunks_exact_mut(16) {
                cipher.encrypt_block(block.into());
            }
            x
        },
    );

    group.finish();
}

fn bench_blowfish(c: &mut Criterion) {
    use blowfish::{
        cipher::{BlockEncrypt, KeyInit},
        Blowfish, BlowfishLE,
    };

    let mut group = c.benchmark_group("Blowfish");
    group.sample_size(20);

    let key = gen_array::<56>();
    let cipher: BlowfishLE = Blowfish::new((&key).into());
    bench_chunk(
        &mut group,
        "Blowfish",
        |size| gen_vec(size),
        |mut x| {
            for block in x.chunks_exact_mut(8) {
                cipher.encrypt_block(block.into());
            }
            x
        },
    );

    group.finish();
}

fn bench_xor(c: &mut Criterion) {
    use std::ops::BitXorAssign;

    let mut group = c.benchmark_group("XOR");
    group.sample_size(20);

    let key: [_; 32] = gen_array();
    bench_chunk(
        &mut group,
        "xor",
        |size| gen_vec(size),
        |mut x| {
            x.iter_mut()
                .zip(key.iter().cycle())
                .for_each(|(x, k)| x.bitxor_assign(k));
            x
        },
    );

    group.finish();
}

criterion_group!(
    benches,
    bench_xor,
    bench_salsa20,
    bench_chacha20poly1305,
    bench_aes,
    bench_blowfish,
    bench_camellia,
    bench_sm4,
);
criterion_main!(benches);
