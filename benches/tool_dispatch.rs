use criterion::{criterion_group, criterion_main, Criterion};

fn bench_json_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("tool_dispatch");

    let simple_params = r#"{"command": "echo hello"}"#;
    let complex_params = r#"{
        "command": "find",
        "args": ["-name", "*.rs", "-type", "f"],
        "working_dir": "/home/user/project",
        "env": {"RUST_LOG": "debug", "PATH": "/usr/bin"},
        "timeout": 30,
        "capture_output": true
    }"#;

    group.bench_function("parse_simple_params", |b| {
        b.iter(|| serde_json::from_str::<serde_json::Value>(simple_params).unwrap())
    });

    group.bench_function("parse_complex_params", |b| {
        b.iter(|| serde_json::from_str::<serde_json::Value>(complex_params).unwrap())
    });

    // Benchmark schema-like validation pattern
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "command": {"type": "string"},
            "args": {"type": "array", "items": {"type": "string"}},
            "timeout": {"type": "integer"}
        },
        "required": ["command"]
    });

    group.bench_function("schema_access_pattern", |b| {
        let params: serde_json::Value = serde_json::from_str(complex_params).unwrap();
        b.iter(|| {
            // Simulate tool parameter validation
            let required = schema["required"].as_array().unwrap();
            for field in required {
                let field_name = field.as_str().unwrap();
                assert!(params.get(field_name).is_some());
            }
        })
    });

    // Benchmark tool output serialization
    let tool_output = serde_json::json!({
        "success": true,
        "output": "total 42\ndrwxr-xr-x  2 user group 4096 Mar  9 12:00 src\n-rw-r--r--  1 user group  256 Mar  9 11:30 Cargo.toml",
        "exit_code": 0,
        "duration_ms": 15
    });

    group.bench_function("serialize_tool_output", |b| {
        b.iter(|| serde_json::to_string(&tool_output).unwrap())
    });

    group.finish();
}

criterion_group!(benches, bench_json_parsing);
criterion_main!(benches);
