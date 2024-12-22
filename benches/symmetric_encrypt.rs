mod _utils;
use _utils::{bench_chunk_util::bench_chunk, *};
use criterion::{measurement::Measurement, BenchmarkGroup};

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
    (@block, $cipher:expr, $size:expr) => {
        |mut x| {
            encrypt_template!(@block, inner: $cipher, $size, &mut x);
            x
        }
    };
    (@block, inner: $cipher:expr, $size:expr, $buf:expr) => {
        for block in $buf.chunks_exact_mut($size) {
            $cipher.encrypt_block(block.into());
        }
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
    (@block, $cipher:expr, $size:expr) => {
        |size| {
            let mut data = gen_vec(size);
            encrypt_template!(@block, inner: $cipher, $size, &mut data);
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
    (@block, $cipher:expr, $size:expr) => {
        |mut x| {
            decrypt_template!(@block, inner: $cipher, $size, &mut x);
            x
        }
    };
    (@block, inner: $cipher:expr, $size:expr, $buf:expr) => {
        for block in $buf.chunks_exact_mut($size) {
            $cipher.decrypt_block(block.into());
        }
    };
}

fn bench_aes<M: Measurement>(group: &mut BenchmarkGroup<M>) {
    use aes_gcm::{
        aes::{Aes128, Aes256},
        AeadInPlace, Aes128Gcm, Aes256Gcm, Key, KeyInit, Nonce,
    };

    let nonce: [_; 12] = gen_array();
    let nonce = Nonce::from_slice(&nonce);

    let key: [_; 16] = gen_array();
    let key = Key::<Aes128>::from_slice(&key);
    let cipher = Aes128Gcm::new(key);
    bench_chunk(
        group,
        "Aes128Gcm encrypt_in_place",
        gen_vec,
        encrypt_template!(cipher, nonce),
    );
    bench_chunk(
        group,
        "Aes128Gcm decrypt_in_place",
        encrypt_init_template!(cipher, nonce),
        decrypt_template!(cipher, nonce),
    );

    let key: [_; 32] = gen_array();
    let key = Key::<Aes256>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);
    bench_chunk(
        group,
        "Aes256Gcm encrypt_in_place",
        gen_vec,
        encrypt_template!(cipher, nonce),
    );
    bench_chunk(
        group,
        "Aes256Gcm decrypt_in_place",
        encrypt_init_template!(cipher, nonce),
        decrypt_template!(cipher, nonce),
    );
}

fn bench_chacha20poly1305<M: Measurement>(group: &mut BenchmarkGroup<M>) {
    use chacha20poly1305::{
        aead::{AeadCore, AeadInPlace, KeyInit, OsRng},
        ChaCha20Poly1305, XChaCha20Poly1305,
    };

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    bench_chunk(
        group,
        "ChaCha20Poly1305 encrypt_in_place",
        gen_vec,
        encrypt_template!(cipher, nonce),
    );
    bench_chunk(
        group,
        "ChaCha20Poly1305 decrypt_in_place",
        encrypt_init_template!(cipher, nonce),
        decrypt_template!(cipher, nonce),
    );

    let key = XChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    bench_chunk(
        group,
        "XChaCha20Poly1305 encrypt_in_place",
        gen_vec,
        encrypt_template!(cipher, nonce),
    );
    bench_chunk(
        group,
        "XChaCha20Poly1305 decrypt_in_place",
        encrypt_init_template!(cipher, nonce),
        decrypt_template!(cipher, nonce),
    );
}

fn bench_sm4<M: Measurement>(group: &mut BenchmarkGroup<M>) {
    use sm4::{
        cipher::{BlockDecrypt, BlockEncrypt, KeyInit},
        Sm4,
    };

    let key = gen_array::<16>();
    let cipher = Sm4::new(&key.into());
    bench_chunk(
        group,
        "Sm4 encrypt",
        gen_vec,
        encrypt_template!(@block, cipher, 16),
    );
    bench_chunk(
        group,
        "Sm4 decrypt",
        encrypt_init_template!(@block, cipher, 16),
        decrypt_template!(@block, cipher, 16),
    );
}

fn bench_camellia<M: Measurement>(group: &mut BenchmarkGroup<M>) {
    use camellia::{
        cipher::{BlockDecrypt, BlockEncrypt, KeyInit},
        Camellia128, Camellia256,
    };

    let key = gen_array::<16>();
    let cipher = Camellia128::new(&key.into());
    bench_chunk(
        group,
        "Camellia128 encrypt",
        gen_vec,
        encrypt_template!(@block, cipher, 16),
    );
    bench_chunk(
        group,
        "Camellia128 decrypt",
        encrypt_init_template!(@block, cipher, 16),
        decrypt_template!(@block, cipher, 16),
    );

    let key = gen_array::<32>();
    let cipher = Camellia256::new(&key.into());
    bench_chunk(
        group,
        "Camellia256 encrypt",
        gen_vec,
        encrypt_template!(@block, cipher, 16),
    );
    bench_chunk(
        group,
        "Camellia256 decrypt",
        encrypt_init_template!(@block, cipher, 16),
        decrypt_template!(@block, cipher, 16),
    );
}

fn bench_blowfish<M: Measurement>(group: &mut BenchmarkGroup<M>) {
    use blowfish::{
        cipher::{BlockDecrypt, BlockEncrypt, KeyInit},
        Blowfish, BlowfishLE,
    };

    let key = gen_array::<56>();
    let cipher: BlowfishLE = Blowfish::new((&key).into());
    bench_chunk(
        group,
        "Blowfish encrypt",
        gen_vec,
        encrypt_template!(@block, cipher, 8),
    );
    bench_chunk(
        group,
        "Blowfish decrypt",
        encrypt_init_template!(@block, cipher, 8),
        decrypt_template!(@block, cipher, 8),
    );
}

fn bench_twofish<M: Measurement>(group: &mut BenchmarkGroup<M>) {
    use twofish::{
        cipher::{BlockDecrypt, BlockEncrypt, KeyInit},
        Twofish,
    };

    let key = gen_array::<32>();
    let cipher: Twofish = Twofish::new((&key).into());
    bench_chunk(
        group,
        "Twofish encrypt",
        gen_vec,
        encrypt_template!(@block, cipher, 16),
    );
    bench_chunk(
        group,
        "Twofish decrypt",
        encrypt_init_template!(@block, cipher, 16),
        decrypt_template!(@block, cipher, 16),
    );
}

fn bench_salsa20<M: Measurement>(group: &mut BenchmarkGroup<M>) {
    use salsa20::cipher::{KeyIvInit, StreamCipher};
    use salsa20::Salsa20;

    let key: [_; 32] = gen_array();
    let nonce: [_; 8] = gen_array();
    let mut cipher = Salsa20::new(&key.into(), &nonce.into());
    bench_chunk(group, "Salsa20", gen_vec, |mut x| {
        cipher.apply_keystream(&mut x);
        x
    });
}

fn bench_xor<M: Measurement>(group: &mut BenchmarkGroup<M>) {
    use std::ops::BitXorAssign;

    let key: [_; 32] = gen_array();
    bench_chunk(group, "xor", gen_vec, |mut x| {
        x.iter_mut()
            .zip(key.iter().cycle())
            .for_each(|(x, k)| x.bitxor_assign(k));
        x
    });
}

fn benching(c: &mut Criterion) {
    let funcs = [
        bench_twofish,
        bench_xor,
        bench_salsa20,
        bench_chacha20poly1305,
        bench_aes,
        bench_blowfish,
        bench_camellia,
        bench_sm4,
    ];
    let mut group = c.benchmark_group("Symmetric Encrypt");
    for func in funcs {
        func(&mut group);
    }
}

criterion_group!(benches, benching);
criterion_main!(benches);
