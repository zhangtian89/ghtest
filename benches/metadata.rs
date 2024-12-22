use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn consume_metadata(m: fs::Metadata) {
    let e = (
        m.file_type(),
        m.len(),
        m.permissions(),
        m.modified().ok(),
        m.accessed().ok(),
        m.created().ok(),
    );
    black_box(e);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::MetadataExt;
        let e = (
            m.dev(),
            m.ino(),
            m.mode(),
            m.nlink(),
            m.uid(),
            m.gid(),
            m.rdev(),
            m.size(),
            m.atime(),
            m.atime_nsec(),
            m.mtime(),
            m.mtime_nsec(),
            m.ctime(),
            m.ctime_nsec(),
            m.blksize(),
            m.blocks(),
        );
        black_box(e);
    }

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        let e = (
            m.file_attributes(),
            m.creation_time(),
            m.last_access_time(),
            m.last_write_time(),
            m.file_size(),
            // m.volume_serial_number(),
            // m.number_of_links(),
            // m.file_index(),
            // m.change_time(),
        );
        black_box(e);
    }
}

fn ignore<T>(x: T) {
    black_box(x);
}

fn bench_metadata(c: &mut Criterion) {
    let mut group = c.benchmark_group("Metadata");

    let test_dir = "./target/test_dir_metadata";

    if fs::remove_dir_all(test_dir).is_ok() {
        println!("Removed: {test_dir}");
    }
    fs::create_dir_all(test_dir).unwrap();

    let dir = {
        let mut d = PathBuf::from(test_dir);
        d.push("dir");
        fs::create_dir(&d).unwrap();
        d
    };
    let file = {
        let mut d = PathBuf::from(test_dir);
        d.push("file");
        fs::File::create(&d)
            .unwrap()
            .write_all("test c".as_bytes())
            .unwrap();
        d
    };

    let entries: Vec<_> = fs::read_dir(test_dir)
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();

    let (dir_entry, file_entry) = {
        let (a, b) = (&entries[0], &entries[1]);
        if fs::metadata(a.path()).unwrap().is_dir() {
            (a, b)
        } else {
            (b, a)
        }
    };

    macro_rules! bench {
        ($mode:expr, $method: expr, $kind:expr) => {{
            group.bench_function(
                format!(
                    "{} {}() on {}",
                    stringify!($mode),
                    stringify!($method),
                    stringify!($kind)
                )
                .as_str(),
                |b| {
                    b.iter(|| ($mode)(($method)(black_box(&($kind))).unwrap()));
                },
            );
        }};
    }

    use fs::DirEntry;
    let consume = consume_metadata;

    bench!(ignore, fs::metadata, dir);
    bench!(ignore, fs::metadata, file);
    bench!(ignore, fs::symlink_metadata, dir);
    bench!(ignore, fs::symlink_metadata, file);
    bench!(ignore, DirEntry::metadata, dir_entry);
    bench!(ignore, DirEntry::metadata, file_entry);
    bench!(consume, fs::metadata, dir);
    bench!(consume, fs::metadata, file);
    bench!(consume, fs::symlink_metadata, dir);
    bench!(consume, fs::symlink_metadata, file);
    bench!(consume, DirEntry::metadata, dir_entry);
    bench!(consume, DirEntry::metadata, file_entry);
}

criterion_group!(benches, bench_metadata);
criterion_main!(benches);
