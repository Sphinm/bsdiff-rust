use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

// 生成测试数据
fn generate_test_data(size: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);
    for i in 0..size {
        data.push((i % 256) as u8);
    }
    data
}

// 生成差异化的测试数据
fn generate_diff_data(base_data: &[u8], change_ratio: f64) -> Vec<u8> {
    let mut new_data = base_data.to_vec();
    let change_count = (base_data.len() as f64 * change_ratio) as usize;
    
    for i in 0..change_count {
        let index = i % base_data.len();
        new_data[index] = (new_data[index] + 1) % 256;
    }
    
    new_data
}

// 基准测试：文件 I/O 性能
fn benchmark_file_io(c: &mut Criterion) {
    let mut group = c.benchmark_group("File I/O Performance");
    
    let sizes = vec![
        ("1KB", 1024),
        ("10KB", 10 * 1024),
        ("100KB", 100 * 1024),
        ("1MB", 1024 * 1024),
    ];
    
    for (name, size) in sizes {
        group.bench_function(&format!("write_{}", name), |b| {
            b.iter(|| {
                let data = generate_test_data(size);
                let file = NamedTempFile::new().unwrap();
                fs::write(&file, &data).unwrap();
                black_box(file);
            });
        });
        
        group.bench_function(&format!("read_{}", name), |b| {
            let data = generate_test_data(size);
            let file = NamedTempFile::new().unwrap();
            fs::write(&file, &data).unwrap();
            
            b.iter(|| {
                let read_data = fs::read(&file).unwrap();
                black_box(read_data);
            });
        });
    }
    
    group.finish();
}

// 基准测试：数据生成性能
fn benchmark_data_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Data Generation");
    
    let sizes = vec![
        ("1KB", 1024),
        ("10KB", 10 * 1024),
        ("100KB", 100 * 1024),
        ("1MB", 1024 * 1024),
    ];
    
    for (name, size) in sizes {
        group.bench_function(&format!("generate_{}", name), |b| {
            b.iter(|| {
                let data = generate_test_data(size);
                black_box(data);
            });
        });
        
        group.bench_function(&format!("diff_generate_{}", name), |b| {
            let base_data = generate_test_data(size);
            b.iter(|| {
                let diff_data = generate_diff_data(&base_data, 0.1);
                black_box(diff_data);
            });
        });
    }
    
    group.finish();
}

// 基准测试：压缩比计算
fn benchmark_compression_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compression Calculation");
    
    let test_cases = vec![
        ("small", 1024, 0.05),
        ("medium", 1024 * 1024, 0.1),
        ("large", 10 * 1024 * 1024, 0.15),
    ];
    
    for (name, size, change_ratio) in test_cases {
        group.bench_function(&format!("ratio_{}", name), |b| {
            let old_data = generate_test_data(size);
            let new_data = generate_diff_data(&old_data, change_ratio);
            
            b.iter(|| {
                let old_size = old_data.len() as f64;
                let new_size = new_data.len() as f64;
                let total_size = old_size + new_size;
                let ratio = if total_size > 0.0 {
                    (old_size / total_size) * 100.0
                } else {
                    0.0
                };
                black_box(ratio);
            });
        });
    }
    
    group.finish();
}

// 基准测试：内存分配
fn benchmark_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Allocation");
    
    let sizes = vec![
        ("1MB", 1024 * 1024),
        ("10MB", 10 * 1024 * 1024),
        ("50MB", 50 * 1024 * 1024),
    ];
    
    for (name, size) in sizes {
        group.bench_function(&format!("allocate_{}", name), |b| {
            b.iter(|| {
                let mut data = Vec::with_capacity(size);
                for i in 0..size {
                    data.push((i % 256) as u8);
                }
                black_box(data);
            });
        });
    }
    
    group.finish();
}

// 基准测试：字符串操作
fn benchmark_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("String Operations");
    
    let test_strings = vec![
        "short",
        "medium length string",
        "very long string with many characters for testing purposes",
    ];
    
    for (i, test_str) in test_strings.iter().enumerate() {
        group.bench_function(&format!("format_{}", i), |b| {
            b.iter(|| {
                let formatted = format!("test_{}_{}", test_str, i);
                black_box(formatted);
            });
        });
        
        group.bench_function(&format!("contains_{}", i), |b| {
            b.iter(|| {
                let contains = test_str.contains("test");
                black_box(contains);
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_file_io,
    benchmark_data_generation,
    benchmark_compression_calculation,
    benchmark_memory_allocation,
    benchmark_string_operations
);
criterion_main!(benches);
