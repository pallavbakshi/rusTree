use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustree::{get_tree_nodes, RustreeLibConfig, ListingOptions, MetadataOptions, SortingOptions, SortKey};
use std::path::Path;
use tempfile::TempDir;
use std::fs;

fn create_benchmark_directory() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();
    
    // Create a reasonably complex directory structure for benchmarking
    for i in 0..10 {
        let dir_path = root.join(format!("dir_{}", i));
        fs::create_dir(&dir_path).unwrap();
        
        for j in 0..20 {
            let file_path = dir_path.join(format!("file_{}.txt", j));
            fs::write(&file_path, format!("Content for file {} in dir {}\nLine 2\nLine 3", j, i)).unwrap();
        }
        
        // Create subdirectories
        for k in 0..5 {
            let subdir_path = dir_path.join(format!("subdir_{}", k));
            fs::create_dir(&subdir_path).unwrap();
            
            for l in 0..10 {
                let subfile_path = subdir_path.join(format!("subfile_{}.log", l));
                fs::write(&subfile_path, "Log entry 1\nLog entry 2\nLog entry 3").unwrap();
            }
        }
    }
    
    temp_dir
}

fn benchmark_tree_walking(c: &mut Criterion) {
    let temp_dir = create_benchmark_directory();
    let root_path = temp_dir.path();
    
    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(3),
            show_hidden: false,
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            ..Default::default()
        },
        ..Default::default()
    };

    c.bench_function("tree_walking", |b| {
        b.iter(|| {
            get_tree_nodes(black_box(root_path), black_box(&config)).unwrap()
        })
    });
}

fn benchmark_tree_walking_with_analysis(c: &mut Criterion) {
    let temp_dir = create_benchmark_directory();
    let root_path = temp_dir.path();
    
    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(3),
            show_hidden: false,
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            calculate_line_count: true,
            calculate_word_count: true,
            ..Default::default()
        },
        ..Default::default()
    };

    c.bench_function("tree_walking_with_analysis", |b| {
        b.iter(|| {
            get_tree_nodes(black_box(root_path), black_box(&config)).unwrap()
        })
    });
}

fn benchmark_sorting(c: &mut Criterion) {
    let temp_dir = create_benchmark_directory();
    let root_path = temp_dir.path();
    
    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(3),
            ..Default::default()
        },
        metadata: MetadataOptions {
            report_sizes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Size),
            reverse_sort: false,
        },
        ..Default::default()
    };

    c.bench_function("sorting", |b| {
        b.iter(|| {
            get_tree_nodes(black_box(root_path), black_box(&config)).unwrap()
        })
    });
}

criterion_group!(benches, benchmark_tree_walking, benchmark_tree_walking_with_analysis, benchmark_sorting);
criterion_main!(benches); 