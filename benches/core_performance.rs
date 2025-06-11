use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rustree::config::metadata::BuiltInFunction;
use rustree::{
    ListingOptions, MetadataOptions, RustreeLibConfig, SortKey, SortingOptions, get_tree_nodes,
};
use std::fs;
use tempfile::TempDir;

fn create_benchmark_directory() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create a reasonably complex directory structure for benchmarking
    for i in 0..10 {
        let dir_path = root.join(format!("dir_{}", i));
        fs::create_dir(&dir_path).unwrap();

        for j in 0..20 {
            let file_path = dir_path.join(format!("file_{}.txt", j));
            fs::write(
                &file_path,
                format!("Content for file {} in dir {}\nLine 2\nLine 3", j, i),
            )
            .unwrap();
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
            show_size_bytes: true,
            ..Default::default()
        },
        ..Default::default()
    };

    c.bench_function("tree_walking", |b| {
        b.iter(|| get_tree_nodes(black_box(root_path), black_box(&config)).unwrap())
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
            show_size_bytes: true,
            calculate_line_count: true,
            calculate_word_count: true,
            ..Default::default()
        },
        ..Default::default()
    };

    c.bench_function("tree_walking_with_analysis", |b| {
        b.iter(|| get_tree_nodes(black_box(root_path), black_box(&config)).unwrap())
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
            show_size_bytes: true,
            ..Default::default()
        },
        sorting: SortingOptions {
            sort_by: Some(SortKey::Size),
            reverse_sort: false,
            ..Default::default()
        },
        ..Default::default()
    };

    c.bench_function("sorting", |b| {
        b.iter(|| get_tree_nodes(black_box(root_path), black_box(&config)).unwrap())
    });
}

fn benchmark_metadata_aggregation(c: &mut Criterion) {
    use rustree::core::metadata::MetadataAggregator;
    use rustree::core::tree::node::{NodeInfo, NodeType};
    use std::path::PathBuf;

    // Create a large dataset of nodes to test aggregation performance
    let mut nodes = Vec::new();

    // Create nodes with various metadata combinations
    for i in 0..10000 {
        let node = NodeInfo {
            name: format!("file_{}.txt", i),
            path: PathBuf::from(format!("test/file_{}.txt", i)),
            node_type: if i % 10 == 0 {
                NodeType::Directory
            } else {
                NodeType::File
            },
            depth: (i % 5) + 1,
            size: Some((i * 1024) as u64),
            permissions: None,
            mtime: None,
            change_time: None,
            create_time: None,
            line_count: Some(i * 10),
            word_count: Some(i * 50),
            custom_function_output: if i % 10 == 0 {
                Some(Ok(format!("{}f,{}d,{}B", i % 20, i % 5, i * 512)))
            } else {
                None
            },
        };
        nodes.push(node);
    }

    let config = RustreeLibConfig {
        metadata: MetadataOptions {
            calculate_line_count: true,
            calculate_word_count: true,
            show_size_bytes: true,
            apply_function: Some(BuiltInFunction::DirStats),
            ..Default::default()
        },
        ..Default::default()
    };

    c.bench_function("metadata_aggregation", |b| {
        b.iter(|| {
            let aggregator =
                MetadataAggregator::aggregate_from_nodes(black_box(&nodes), black_box(&config));
            black_box(aggregator.format_summary_additions())
        })
    });
}

fn benchmark_number_formatting(c: &mut Criterion) {
    use rustree::core::metadata::MetadataAggregator;

    let numbers = vec![
        0, 1, 12, 123, 1234, 12345, 123456, 1234567, 12345678, 123456789, 1234567890,
    ];

    c.bench_function("number_formatting", |b| {
        b.iter(|| {
            for &num in &numbers {
                black_box(MetadataAggregator::format_number(black_box(num)));
            }
        })
    });
}

fn benchmark_size_formatting(c: &mut Criterion) {
    use rustree::core::metadata::MetadataAggregator;

    let sizes = vec![
        0,
        512,
        1024,
        1536,
        2048,
        1048576,
        2097152,
        1073741824,
        1099511627776,
    ];

    c.bench_function("size_formatting", |b| {
        b.iter(|| {
            for &size in &sizes {
                black_box(MetadataAggregator::format_size(black_box(size)));
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_tree_walking,
    benchmark_tree_walking_with_analysis,
    benchmark_sorting,
    benchmark_metadata_aggregation,
    benchmark_number_formatting,
    benchmark_size_formatting
);
criterion_main!(benches);
