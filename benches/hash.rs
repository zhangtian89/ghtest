mod _utils;
use _utils::{bench_chunk_util::bench_chunk, *};

fn bench_xor(c: &mut Criterion) {
    use std::ops::BitXorAssign;

    let mut group = c.benchmark_group("XOR");
    group.sample_size(20);

    bench_chunk(
        &mut group,
        "xor",
        |size| (gen_vec(size), [0u8; 32]),
        |(data, mut buf)| {
            data.iter()
                .zip((0..).map(|x| x % 32))
                .for_each(|(x, k)| buf[k].bitxor_assign(x));
            buf
        },
    );

    group.finish();
}

macro_rules! init_template {
    ($kind: ident) => {
        |size| -> ($kind, Vec<u8>) { ($kind::new(), gen_vec(size)) }
    };
    ($kind: ty, $init: expr) => {
        |size| -> ($kind, Vec<u8>) { ($init, gen_vec(size)) }
    };
    ($kind: ty, $init: expr, $size: ident) => {
        |$size| -> ($kind, Vec<u8>) { ($init, gen_vec($size)) }
    };
}

macro_rules! bench_template {
    ($y: ident, $e: expr) => {
        |(mut hasher, data)| {
            hasher.update(data.as_slice());
            let $y = hasher.finalize();
            ($e)
        }
    };
    () => {
        |(mut hasher, data)| {
            hasher.update(data.as_slice());
            hasher.finalize()
        }
    };
}

fn bench_sha2(c: &mut Criterion) {
    use sha2::{Digest, Sha256, Sha512};

    let mut group = c.benchmark_group("Sha2");
    group.sample_size(20);

    bench_chunk(
        &mut group,
        "Sha256",
        init_template!(Sha256),
        bench_template!(),
    );
    bench_chunk(
        &mut group,
        "Sha512",
        init_template!(Sha512),
        bench_template!(),
    );

    group.finish();
}

fn bench_sha3(c: &mut Criterion) {
    use sha3::{Digest, Sha3_256, Sha3_512};

    let mut group = c.benchmark_group("Sha3");
    group.sample_size(20);

    bench_chunk(
        &mut group,
        "Sha3_256",
        init_template!(Sha3_256),
        bench_template!(),
    );
    bench_chunk(
        &mut group,
        "Sha3_512",
        init_template!(Sha3_512),
        bench_template!(),
    );

    group.finish();
}

fn bench_hmac(c: &mut Criterion) {
    use hmac::{Hmac, Mac};
    use sha2::{Sha256, Sha512};

    let mut group = c.benchmark_group("Hmac");
    group.sample_size(20);

    let key: [_; 32] = gen_array();
    bench_chunk(
        &mut group,
        "Hmac<Sha256> hash",
        init_template!(Hmac<Sha256>, Hmac::new_from_slice(&key).unwrap()),
        bench_template!(x, x.into_bytes()),
    );
    bench_chunk(
        &mut group,
        "Hmac<Sha512> hash",
        init_template!(Hmac<Sha512>, Hmac::new_from_slice(&key).unwrap()),
        bench_template!(x, x.into_bytes()),
    );

    bench_chunk(
        &mut group,
        "Hmac<Sha256> verify",
        |size| {
            let data = gen_vec(size);
            let mut hasher: Hmac<Sha256> = Hmac::new_from_slice(&key).unwrap();
            hasher.update(data.as_slice());
            let hash = hasher.finalize().into_bytes();
            (hash, data)
        },
        |(hash, data)| {
            let mut hasher: Hmac<Sha256> = Hmac::new_from_slice(&key).unwrap();
            hasher.update(data.as_slice());
            hasher.verify_slice(&hash[..]).unwrap();
            (hash, data)
        },
    );
    bench_chunk(
        &mut group,
        "Hmac<Sha512> verify",
        init_template!(Hmac<Sha512>, Hmac::new_from_slice(&key).unwrap()),
        bench_template!(x, x.into_bytes()),
    );

    group.finish();
}

fn bench_whirlpool(c: &mut Criterion) {
    use whirlpool::{Digest, Whirlpool};

    let mut group = c.benchmark_group("Whirlpool");
    group.sample_size(20);

    bench_chunk(
        &mut group,
        "Whirlpool",
        init_template!(Whirlpool),
        bench_template!(),
    );

    group.finish();
}

fn bench_sm3(c: &mut Criterion) {
    use sm3::{Digest, Sm3};

    let mut group = c.benchmark_group("Sm3");
    group.sample_size(20);

    bench_chunk(&mut group, "Sm3", init_template!(Sm3), bench_template!());

    group.finish();
}

fn bench_blake2(c: &mut Criterion) {
    use blake2::{digest::Digest, Blake2b512, Blake2s256};

    let mut group = c.benchmark_group("Blake2b");
    group.sample_size(20);

    bench_chunk(
        &mut group,
        "Blake2s256",
        init_template!(Blake2s256),
        bench_template!(),
    );
    bench_chunk(
        &mut group,
        "Blake2b512",
        init_template!(Blake2b512),
        bench_template!(),
    );

    group.finish();
}

fn bench_blake2s_simd(c: &mut Criterion) {
    let mut group = c.benchmark_group("blake2s_simd");
    group.sample_size(20);

    bench_chunk(
        &mut group,
        "blake2s",
        |s| gen_vec(s),
        |x| blake2s_simd::blake2s(x.as_slice()),
    );
    bench_chunk(
        &mut group,
        "blake2sp",
        |s| gen_vec(s),
        |x| blake2s_simd::blake2sp::blake2sp(x.as_slice()),
    );

    group.finish();
}

fn bench_blake3(c: &mut Criterion) {
    use blake3::hash;

    let mut group = c.benchmark_group("Blake3");
    group.sample_size(20);

    bench_chunk(&mut group, "Blake3", |s| gen_vec(s), |x| hash(x.as_slice()));

    group.finish();
}

criterion_group!(
    benches,
    bench_xor,
    bench_whirlpool,
    bench_sha2,
    bench_sha3,
    bench_hmac,
    bench_sm3,
    bench_blake2,
    bench_blake3,
    bench_blake2s_simd,
);
criterion_main!(benches);
