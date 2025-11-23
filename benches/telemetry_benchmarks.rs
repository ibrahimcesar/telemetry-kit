use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use telemetry_kit::prelude::*;
use telemetry_kit::privacy::{PrivacyConfig, PrivacyManager};

/// Benchmark: Event creation overhead
fn bench_event_creation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("event_creation_command", |b| {
        b.to_async(&rt).iter(|| async {
            let telemetry = TelemetryKit::builder()
                .service_name("bench-app")
                .unwrap()
                .build()
                .unwrap();

            telemetry
                .track_command("test", |e| e.success(true).duration_ms(100))
                .await
                .unwrap();
        });
    });
}

/// Benchmark: Privacy sanitization overhead
fn bench_privacy_sanitization(c: &mut Criterion) {
    let mut group = c.benchmark_group("privacy_sanitization");

    // Path sanitization
    group.bench_function("sanitize_path", |b| {
        let path = "/Users/testuser/Documents/secret.txt";
        b.iter(|| PrivacyManager::sanitize_path(black_box(path)));
    });

    // Email sanitization
    group.bench_function("sanitize_email", |b| {
        let email = "user@example.com";
        b.iter(|| PrivacyManager::sanitize_email(black_box(email)));
    });

    // Nested data sanitization
    group.bench_function("sanitize_nested_data", |b| {
        let config = PrivacyConfig::default();
        let manager = PrivacyManager::new(config, "bench").unwrap();

        b.iter(|| {
            let mut data = serde_json::json!({
                "email": "user@example.com",
                "path": "/Users/test/file.txt",
                "nested": {
                    "email": "another@example.com",
                    "deep": {
                        "path": "/home/user/data.txt"
                    }
                }
            });
            manager.sanitize_data(black_box(&mut data));
        });
    });

    group.finish();
}

/// Benchmark: DO_NOT_TRACK check overhead
fn bench_dnt_check(c: &mut Criterion) {
    c.bench_function("dnt_check", |b| {
        // Remove DNT to test the check itself
        std::env::remove_var("DO_NOT_TRACK");

        b.iter(|| TelemetryKit::is_do_not_track_enabled());
    });
}

/// Benchmark: Consent check overhead
fn bench_consent_check(c: &mut Criterion) {
    let config = PrivacyConfig::strict();
    let manager = PrivacyManager::new(config, "bench-consent").unwrap();

    // Grant consent for the benchmark
    manager.grant_consent("bench-consent").unwrap();

    c.bench_function("consent_check", |b| {
        b.iter(|| manager.should_track().unwrap());
    });
}

/// Benchmark: Event serialization
fn bench_event_serialization(c: &mut Criterion) {
    use chrono::Utc;
    use telemetry_kit::event::{
        Environment, Event, EventData, Metadata, ServiceInfo, SCHEMA_VERSION,
    };
    use uuid::Uuid;

    let event = Event {
        schema_version: SCHEMA_VERSION.to_string(),
        event_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        service: ServiceInfo {
            name: "bench-service".to_string(),
            version: "1.0.0".to_string(),
            language: "rust".to_string(),
            language_version: Some("1.75.0".to_string()),
        },
        user_id: "client_bench123".to_string(),
        session_id: Some("sess_bench456".to_string()),
        environment: Environment {
            os: "linux".to_string(),
            os_version: None,
            arch: Some("x86_64".to_string()),
            ci: Some(false),
            shell: None,
        },
        event: EventData {
            event_type: "test_event".to_string(),
            category: Some("test".to_string()),
            data: serde_json::json!({"test": true}),
        },
        metadata: Metadata {
            sdk_version: "0.2.0".to_string(),
            transmission_timestamp: Utc::now(),
            batch_size: 1,
            retry_count: 0,
        },
    };

    c.bench_function("event_serialization", |b| {
        b.iter(|| serde_json::to_string(black_box(&event)).unwrap());
    });
}

// Note: HMAC signature benchmarking removed as auth module is private
// This is an internal implementation detail and doesn't need benchmarking

/// Benchmark: Batch event processing
fn bench_batch_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_processing");

    for size in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let rt = tokio::runtime::Runtime::new().unwrap();

            b.to_async(&rt).iter(|| async move {
                let telemetry = TelemetryKit::builder()
                    .service_name("bench-batch")
                    .unwrap()
                    .build()
                    .unwrap();

                for i in 0..size {
                    telemetry
                        .track_command(&format!("cmd_{}", i), |e| {
                            e.success(true).duration_ms(i as u64)
                        })
                        .await
                        .unwrap();
                }
            });
        });
    }

    group.finish();
}

/// Benchmark: Storage write performance
fn bench_storage_write(c: &mut Criterion) {
    use chrono::Utc;
    use telemetry_kit::event::{
        Environment, Event, EventData, Metadata, ServiceInfo, SCHEMA_VERSION,
    };
    use telemetry_kit::storage::EventStorage;
    use uuid::Uuid;

    let mut group = c.benchmark_group("storage_write");
    group.measurement_time(Duration::from_secs(10));

    // Create test database path
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join("bench_storage.db");

    // Clean up if exists
    let _ = std::fs::remove_file(&db_path);

    let storage = EventStorage::new(&db_path).unwrap();

    group.bench_function("write_single_event", |b| {
        b.iter(|| {
            let event = Event {
                schema_version: SCHEMA_VERSION.to_string(),
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                service: ServiceInfo {
                    name: "bench-service".to_string(),
                    version: "1.0.0".to_string(),
                    language: "rust".to_string(),
                    language_version: Some("1.75.0".to_string()),
                },
                user_id: "client_bench123".to_string(),
                session_id: Some("sess_bench456".to_string()),
                environment: Environment {
                    os: "linux".to_string(),
                    os_version: None,
                    arch: Some("x86_64".to_string()),
                    ci: Some(false),
                    shell: None,
                },
                event: EventData {
                    event_type: "test_event".to_string(),
                    category: Some("test".to_string()),
                    data: serde_json::json!({"test": true}),
                },
                metadata: Metadata {
                    sdk_version: "0.2.0".to_string(),
                    transmission_timestamp: Utc::now(),
                    batch_size: 1,
                    retry_count: 0,
                },
            };

            storage.insert(black_box(&event)).unwrap();
        });
    });

    // Cleanup
    drop(storage);
    let _ = std::fs::remove_file(&db_path);

    group.finish();
}

/// Benchmark: Builder pattern overhead
fn bench_builder_overhead(c: &mut Criterion) {
    c.bench_function("builder_minimal", |b| {
        b.iter(|| {
            TelemetryKit::builder()
                .service_name(black_box("bench-app"))
                .unwrap()
                .build()
                .unwrap()
        });
    });

    c.bench_function("builder_with_privacy", |b| {
        b.iter(|| {
            TelemetryKit::builder()
                .service_name(black_box("bench-app"))
                .unwrap()
                .strict_privacy()
                .build()
                .unwrap()
        });
    });
}

criterion_group!(
    benches,
    bench_event_creation,
    bench_privacy_sanitization,
    bench_dnt_check,
    bench_consent_check,
    bench_event_serialization,
    bench_batch_processing,
    bench_storage_write,
    bench_builder_overhead,
);

criterion_main!(benches);
