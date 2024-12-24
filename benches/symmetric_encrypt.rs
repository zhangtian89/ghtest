#![allow(unused)]

mod _utils;
use _utils::{bench_chunk_util::bench_chunk, *};
use criterion::{measurement::Measurement, BenchmarkGroup};

const ASSOCIATED: &[u8] = b"ASSOCIATED data";
const BENCH_DECRYPT: bool = false;
const BENCH_ENCRYPT_SUFFIX: &str = if BENCH_DECRYPT { " encrypt" } else { "" };

macro_rules! bench_name {
    (@encrypt, $name: expr) => {
        if BENCH_DECRYPT {
            format!("{} encrypt", $name)
        } else {
            $name.to_string()
        }
        .as_str()
    };
    (@decrypt, $name: expr) => {
        format!("{} decrypt", $name).as_str()
    };
}
macro_rules! template {
    (@encrypt, $cipher:expr, $nonce:expr) => {
        |mut x| {
            template!(@encrypt, inner: $cipher, $nonce, &mut x);
            x
        }
    };
    (@encrypt, inner: $cipher:expr, $nonce:expr, $buf:expr) => {
        $cipher.encrypt_in_place(black_box(&($nonce)), black_box(ASSOCIATED), black_box($buf)).unwrap()
    };
    (@encrypt, @block, $cipher:expr, $size:expr) => {
        |mut x| {
            template!(@encrypt, @block, inner: $cipher, $size, &mut x);
            x
        }
    };
    (@encrypt, @block, inner: $cipher:expr, $size:expr, $buf:expr) => {
        {
            let cipher = black_box($cipher);
            for block in black_box($buf).chunks_exact_mut($size) {
                cipher.encrypt_block(block.into());
            }
        }
    };

    (@decrypt, $cipher:expr, $nonce:expr) => {
        |mut x| {
            template!(@decrypt, inner: $cipher, $nonce, &mut x);
            x
        }
    };
    (@decrypt, inner: $cipher:expr, $nonce:expr, $buf:expr) => {
        $cipher.decrypt_in_place(black_box(&($nonce)), black_box(ASSOCIATED), black_box($buf)).unwrap();
    };
    (@decrypt, @block, $cipher:expr, $size:expr) => {
        |mut x| {
            template!(@decrypt, @block, inner: $cipher, $size, &mut x);
            x
        }
    };
    (@decrypt, @block, inner: $cipher:expr, $size:expr, $buf:expr) => {
        {
            let cipher = black_box($cipher);
            for block in black_box($buf).chunks_exact_mut($size) {
                $cipher.decrypt_block(block.into());
            }
        }
    };
}

macro_rules! encrypt_init_template {
    ($cipher:expr, $nonce:expr) => {
        |size| {
            let mut data = gen_vec(size);
            template!(@encrypt, inner: $cipher, $nonce, &mut data);
            data
        }
    };
    (@raw, $data:ident, $x: expr) => {
        |size| {
            let mut $data = gen_vec(size);
            $x
        }
    };
    (@block, $cipher:expr, $size:expr) => {
        |size| {
            let mut data = gen_vec(size);
            template!(@encrypt, @block, inner: $cipher, $size, &mut data);
            data
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
        bench_name!(@encrypt, "Aes128Gcm rust-crypto"),
        gen_vec,
        template!(@encrypt, cipher, nonce),
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "Aes128Gcm rust-crypto"),
            encrypt_init_template!(cipher, nonce),
            template!(@decrypt, cipher, nonce),
        );
    }

    let key: [_; 32] = gen_array();
    let key = Key::<Aes256>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);
    bench_chunk(
        group,
        bench_name!(@encrypt, "Aes256Gcm rust-crypto"),
        gen_vec,
        template!(@encrypt, cipher, nonce),
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "Aes256Gcm rust-crypto"),
            encrypt_init_template!(cipher, nonce),
            template!(@decrypt, cipher, nonce),
        );
    }

    use ring::aead::{
        Aad, LessSafeKey, Nonce as RingNonce, OpeningKey, UnboundKey, AES_128_GCM, AES_256_GCM,
    };

    let nonce: [_; 12] = gen_array();
    let key: [_; 16] = gen_array();
    let key = black_box(LessSafeKey::new(
        UnboundKey::new(&AES_128_GCM, &key).unwrap(),
    ));
    let aad = Aad::from(&ASSOCIATED);
    bench_chunk(
        group,
        bench_name!(@encrypt, "Aes128Gcm ring"),
        gen_vec,
        |mut x| {
            let nonce = RingNonce::assume_unique_for_key(nonce);
            key.seal_in_place_append_tag(nonce, aad, &mut x).unwrap();
            x
        },
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "Aes128Gcm ring"),
            encrypt_init_template!(@raw, x, {
                let nonce = RingNonce::assume_unique_for_key(nonce);
                key.seal_in_place_append_tag(nonce, aad, &mut x).unwrap();
                x
            }),
            |mut x| {
                let nonce = RingNonce::assume_unique_for_key(nonce);
                key.open_in_place(nonce, aad, &mut x).unwrap();
                x
            },
        );
    }

    let nonce: [_; 12] = gen_array();
    let key: [_; 32] = gen_array();
    let key = black_box(LessSafeKey::new(
        UnboundKey::new(&AES_256_GCM, &key).unwrap(),
    ));
    let aad = Aad::from(&ASSOCIATED);
    bench_chunk(
        group,
        bench_name!(@encrypt, "Aes256Gcm ring"),
        gen_vec,
        |mut x| {
            let nonce = RingNonce::assume_unique_for_key(nonce);
            key.seal_in_place_append_tag(nonce, aad, &mut x).unwrap();
            x
        },
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "Aes256Gcm ring"),
            encrypt_init_template!(@raw, x, {
                let nonce = RingNonce::assume_unique_for_key(nonce);
                key.seal_in_place_append_tag(nonce, aad, &mut x).unwrap();
                x
            }),
            |mut x| {
                let nonce = RingNonce::assume_unique_for_key(nonce);
                key.open_in_place(nonce, aad, &mut x).unwrap();
                x
            },
        );
    }
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
        bench_name!(@encrypt, "ChaCha20Poly1305 rust-crypto"),
        gen_vec,
        template!(@encrypt, cipher, nonce),
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "ChaCha20Poly1305 rust-crypto"),
            encrypt_init_template!(cipher, nonce),
            template!(@decrypt, cipher, nonce),
        );
    }

    let key = XChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    bench_chunk(
        group,
        bench_name!(@encrypt, "XChaCha20Poly1305 rust-crypto"),
        gen_vec,
        template!(@encrypt, cipher, nonce),
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "XChaCha20Poly1305 rust-crypto"),
            encrypt_init_template!(cipher, nonce),
            template!(@decrypt, cipher, nonce),
        );
    }

    use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};
    let nonce: [_; 12] = gen_array();
    let key: [_; 32] = gen_array();
    let key = black_box(LessSafeKey::new(
        UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap(),
    ));
    let aad = Aad::from(&ASSOCIATED);
    bench_chunk(
        group,
        bench_name!(@encrypt, "CHACHA20_POLY1305 ring"),
        gen_vec,
        |mut x| {
            let nonce = Nonce::assume_unique_for_key(nonce);
            key.seal_in_place_append_tag(nonce, aad, &mut x);
            x
        },
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "CHACHA20_POLY1305 ring"),
            encrypt_init_template!(@raw, x, {
                let nonce = Nonce::assume_unique_for_key(nonce);
                key.seal_in_place_append_tag(nonce, aad, &mut x).unwrap();
                x
            }),
            |mut x| {
                let nonce = Nonce::assume_unique_for_key(nonce);
                key.open_in_place(nonce, aad, &mut x).unwrap();
                x
            },
        );
    }
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
        bench_name!(@encrypt, "Sm4"),
        gen_vec,
        template!(@encrypt, @block, &cipher, 16),
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "Sm4"),
            encrypt_init_template!(@block, &cipher, 16),
            template!(@decrypt, @block, &cipher, 16),
        );
    }
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
        bench_name!(@encrypt, "Camellia128"),
        gen_vec,
        template!(@encrypt, @block, &cipher, 16),
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "Camellia128"),
            encrypt_init_template!(@block, &cipher, 16),
            template!(@decrypt, @block, &cipher, 16),
        );
    }

    let key = gen_array::<32>();
    let cipher = Camellia256::new(&key.into());
    bench_chunk(
        group,
        bench_name!(@encrypt, "Camellia256"),
        gen_vec,
        template!(@encrypt, @block, &cipher, 16),
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "Camellia256"),
            encrypt_init_template!(@block, &cipher, 16),
            template!(@decrypt, @block, &cipher, 16),
        );
    }
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
        bench_name!(@encrypt, "Blowfish"),
        gen_vec,
        template!(@encrypt, @block, &cipher, 8),
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "Blowfish"),
            encrypt_init_template!(@block, &cipher, 8),
            template!(@decrypt, @block, &cipher, 8),
        );
    }
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
        bench_name!(@encrypt, "Twofish"),
        gen_vec,
        template!(@encrypt, @block, &cipher, 16),
    );
    if BENCH_DECRYPT {
        bench_chunk(
            group,
            bench_name!(@decrypt, "Twofish"),
            encrypt_init_template!(@block, &cipher, 16),
            template!(@decrypt, @block, &cipher, 16),
        );
    }
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

    let key: [_; 32] = black_box(gen_array());
    bench_chunk(group, "xor 32B", gen_vec, |mut x| {
        x.iter_mut()
            .zip(key.iter().cycle())
            .for_each(|(x, k)| x.bitxor_assign(k));
        x
    });

    let key: [_; 16] = black_box(gen_array());
    bench_chunk(group, "xor 16B", gen_vec, |mut x| {
        x.iter_mut()
            .zip(key.iter().cycle())
            .for_each(|(x, k)| x.bitxor_assign(k));
        x
    });

    let key = black_box(gen_array::<1>()[0]);
    bench_chunk(group, "xor 1B", gen_vec, |mut x| {
        x.iter_mut().for_each(|x| x.bitxor_assign(black_box(key)));
        x
    });
}

fn benching(c: &mut Criterion) {
    let funcs = [
        bench_chacha20poly1305,
        bench_aes,
        bench_xor,
        // bench_salsa20,
        // bench_twofish,
        // bench_blowfish,
        // bench_camellia,
        // bench_sm4,
    ];
    let mut group = c.benchmark_group("Symmetric Encrypt");
    for func in funcs {
        func(&mut group);
    }
}

criterion_group!(benches, benching);
criterion_main!(benches);
