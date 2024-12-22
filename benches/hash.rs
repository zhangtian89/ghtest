mod _utils;
use _utils::{bench_chunk_util::bench_chunk, *};
use criterion::BenchmarkGroup;

fn bench_xor<M: criterion::measurement::Measurement>(group: &mut BenchmarkGroup<M>) {
    use std::ops::BitXorAssign;

    bench_chunk(
        group,
        "xor",
        |size| (gen_vec(size), [0u8; 32]),
        |(data, mut buf)| {
            data.iter()
                .zip((0..).map(|x| x % 32))
                .for_each(|(x, k)| buf[k].bitxor_assign(x));
            buf
        },
    );
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

fn bench_sha2<M: criterion::measurement::Measurement>(group: &mut BenchmarkGroup<M>) {
    use sha2::{Digest, Sha256, Sha512};

    bench_chunk(group, "Sha256", init_template!(Sha256), bench_template!());
    bench_chunk(group, "Sha512", init_template!(Sha512), bench_template!());
}

fn bench_sha3<M: criterion::measurement::Measurement>(group: &mut BenchmarkGroup<M>) {
    use sha3::{Digest, Sha3_256, Sha3_512};

    bench_chunk(
        group,
        "Sha3_256",
        init_template!(Sha3_256),
        bench_template!(),
    );
    bench_chunk(
        group,
        "Sha3_512",
        init_template!(Sha3_512),
        bench_template!(),
    );
}

fn bench_hmac<M: criterion::measurement::Measurement>(group: &mut BenchmarkGroup<M>) {
    use hmac::{Hmac, Mac};
    use sha2::{Sha256, Sha512};

    let key: [_; 32] = gen_array();
    bench_chunk(
        group,
        "Hmac<Sha256> hash",
        init_template!(Hmac<Sha256>, Hmac::new_from_slice(&key).unwrap()),
        bench_template!(x, x.into_bytes()),
    );
    bench_chunk(
        group,
        "Hmac<Sha512> hash",
        init_template!(Hmac<Sha512>, Hmac::new_from_slice(&key).unwrap()),
        bench_template!(x, x.into_bytes()),
    );

    bench_chunk(
        group,
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
        group,
        "Hmac<Sha512> verify",
        init_template!(Hmac<Sha512>, Hmac::new_from_slice(&key).unwrap()),
        bench_template!(x, x.into_bytes()),
    );
}

fn bench_whirlpool<M: criterion::measurement::Measurement>(group: &mut BenchmarkGroup<M>) {
    use whirlpool::{Digest, Whirlpool};

    bench_chunk(
        group,
        "Whirlpool",
        init_template!(Whirlpool),
        bench_template!(),
    );
}

fn bench_sm3<M: criterion::measurement::Measurement>(group: &mut BenchmarkGroup<M>) {
    use sm3::{Digest, Sm3};

    // let mut group = c.benchmark_group("Sm3");
    bench_chunk(group, "Sm3", init_template!(Sm3), bench_template!());
}

fn bench_blake2<M: criterion::measurement::Measurement>(group: &mut BenchmarkGroup<M>) {
    use blake2::{digest::Digest, Blake2b512, Blake2s256};

    // let mut group = c.benchmark_group("Blake2b");
    bench_chunk(
        group,
        "Blake2s256",
        init_template!(Blake2s256),
        bench_template!(),
    );
    bench_chunk(
        group,
        "Blake2b512",
        init_template!(Blake2b512),
        bench_template!(),
    );
}

fn bench_blake2s_simd<M: criterion::measurement::Measurement>(group: &mut BenchmarkGroup<M>) {
    bench_chunk(group, "blake2s_simd", gen_vec, |x| {
        blake2s_simd::blake2s(x.as_slice())
    });
    bench_chunk(group, "blake2sp_simd", gen_vec, |x| {
        blake2s_simd::blake2sp::blake2sp(x.as_slice())
    });
}

fn bench_blake3<M: criterion::measurement::Measurement>(group: &mut BenchmarkGroup<M>) {
    use blake3::hash;
    bench_chunk(group, "Blake3", gen_vec, |x| hash(x.as_slice()));
}

fn benching(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hash");
    let funcs = [
        bench_xor,
        bench_whirlpool,
        bench_sha2,
        bench_sha3,
        bench_hmac,
        bench_sm3,
        bench_blake2,
        bench_blake3,
        bench_blake2s_simd,
    ];
    for f in funcs {
        f(&mut group)
    }
    group.finish();
}

criterion_group!(benches, benching);
criterion_main!(benches);
