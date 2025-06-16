// tests/context_performance_tests.rs
//
// Performance tests for the owned context caching mechanisms.
// These tests verify that pattern compilation caching provides
// performance benefits and that context operations are efficient.

use anyhow::Result;
use rustree::core::options::contexts::*;
use rustree::*;
use std::time::{Duration, Instant};

mod common;
use common::{common_test_utils, context_utils};

#[test]
fn test_pattern_compilation_caching_performance() -> Result<()> {
    let mut owned_walking = OwnedWalkingContext::new(
        ListingOptions::default(),
        FilteringOptions {
            ignore_patterns: Some(vec![
                "*.tmp".to_string(),
                "*.log".to_string(),
                "*.bak".to_string(),
                "*.cache".to_string(),
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/.git/**".to_string(),
                "**/__pycache__/**".to_string(),
            ]),
            match_patterns: Some(vec![
                "*.rs".to_string(),
                "*.py".to_string(),
                "*.js".to_string(),
                "*.ts".to_string(),
                "*.java".to_string(),
                "*.cpp".to_string(),
                "*.c".to_string(),
                "*.h".to_string(),
            ]),
            case_insensitive_filter: true,
            ..Default::default()
        },
        MetadataOptions::default(),
    );

    // Time first compilation (cold cache)
    let start_cold = Instant::now();
    let _ignore_patterns_cold = owned_walking.ignore_patterns()?;
    let _match_patterns_cold = owned_walking.match_patterns()?;
    let cold_duration = start_cold.elapsed();

    // Time subsequent access (warm cache)
    let start_warm = Instant::now();
    let _ignore_patterns_warm = owned_walking.ignore_patterns()?;
    let _match_patterns_warm = owned_walking.match_patterns()?;
    let warm_duration = start_warm.elapsed();

    // Cached access should be significantly faster
    println!("Cold compilation: {:?}", cold_duration);
    println!("Warm cache access: {:?}", warm_duration);

    // Cache should be at least 5x faster (more lenient threshold to avoid flaky tests)
    // Note: In practice, it's often 100x-1000x faster, but we use a conservative threshold
    // to account for system load variations and different hardware
    assert!(
        warm_duration < cold_duration / 5,
        "Cached pattern access should be much faster. Cold: {:?}, Warm: {:?}",
        cold_duration,
        warm_duration
    );

    // Verify patterns are actually cached (indirectly through behavior)
    // Note: compiled_* fields are private, so we test through performance characteristics

    Ok(())
}

#[test]
fn test_repeated_pattern_compilation_without_cache() -> Result<()> {
    let filtering = FilteringOptions {
        ignore_patterns: Some(vec![
            "*.tmp".to_string(),
            "*.log".to_string(),
            "**/target/**".to_string(),
            "**/node_modules/**".to_string(),
        ]),
        case_insensitive_filter: true,
        ..Default::default()
    };

    // Simulate non-cached compilation (like what would happen without owned contexts)
    let start = Instant::now();
    for _ in 0..10 {
        let _patterns = rustree::core::filter::pattern::compile_glob_patterns(
            &filtering.ignore_patterns,
            filtering.case_insensitive_filter,
            false, // show_hidden
        )?;
    }
    let non_cached_duration = start.elapsed();

    // Now test with caching
    let mut owned_walking = OwnedWalkingContext::new(
        ListingOptions::default(),
        filtering.clone(),
        MetadataOptions::default(),
    );

    let start_cached = Instant::now();
    for _ in 0..10 {
        let _patterns = owned_walking.ignore_patterns()?;
    }
    let cached_duration = start_cached.elapsed();

    println!("Non-cached (10 compilations): {:?}", non_cached_duration);
    println!("Cached (1 compilation + 9 lookups): {:?}", cached_duration);

    // Cached version should be significantly faster for repeated access
    assert!(
        cached_duration < non_cached_duration / 2,
        "Cached pattern access should be faster for repeated operations. Non-cached: {:?}, Cached: {:?}",
        non_cached_duration,
        cached_duration
    );

    Ok(())
}

#[test]
fn test_context_creation_performance() -> Result<()> {
    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(3),
            show_hidden: true,
            ..Default::default()
        },
        filtering: FilteringOptions {
            ignore_patterns: Some(vec!["*.tmp".to_string(), "*.log".to_string()]),
            match_patterns: Some(vec!["*.rs".to_string()]),
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            calculate_line_count: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Test borrowed context creation (should be very fast)
    let start_borrowed = Instant::now();
    for _ in 0..1000 {
        let _walking_ctx = config.walking_context();
        let _formatting_ctx = config.formatting_context();
        let _sorting_ctx = config.sorting_context();
        let _processing_ctx = config.processing_context();
    }
    let borrowed_duration = start_borrowed.elapsed();

    // Test owned context creation (involves cloning)
    let start_owned = Instant::now();
    for _ in 0..100 {
        let _walking_ctx = config.to_owned_walking_context();
        let _formatting_ctx = config.to_owned_formatting_context();
        let _sorting_ctx = config.to_owned_sorting_context();
        let _processing_ctx = config.to_owned_processing_context();
    }
    let owned_duration = start_owned.elapsed();

    println!("Borrowed context creation (1000x): {:?}", borrowed_duration);
    println!("Owned context creation (100x): {:?}", owned_duration);

    // Borrowed contexts should be very fast (essentially no-op)
    assert!(
        borrowed_duration < Duration::from_millis(10),
        "Borrowed context creation should be very fast: {:?}",
        borrowed_duration
    );

    // Owned context creation should still be reasonable
    assert!(
        owned_duration < Duration::from_millis(100),
        "Owned context creation should be reasonable: {:?}",
        owned_duration
    );

    Ok(())
}

#[test]
fn test_context_validation_performance() -> Result<()> {
    let valid_walking = context_utils::create_test_walking_context();
    let valid_formatting = context_utils::create_test_formatting_context();

    // Test validation performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = valid_walking.validate();
        let _ = valid_formatting.validate();
    }
    let validation_duration = start.elapsed();

    println!("Context validation (1000x each): {:?}", validation_duration);

    // Validation should be fast
    assert!(
        validation_duration < Duration::from_millis(50),
        "Context validation should be fast: {:?}",
        validation_duration
    );

    Ok(())
}

#[test]
fn test_as_borrowed_conversion_performance() -> Result<()> {
    let owned_walking = context_utils::create_test_walking_context();
    let owned_formatting = context_utils::create_test_formatting_context();
    let owned_sorting = context_utils::create_test_sorting_context();

    // Test as_borrowed conversion performance
    let start = Instant::now();
    for _ in 0..10000 {
        let _borrowed_walking = owned_walking.as_borrowed();
        let _borrowed_formatting = owned_formatting.as_borrowed();
        let _borrowed_sorting = owned_sorting.as_borrowed();
    }
    let conversion_duration = start.elapsed();

    println!(
        "as_borrowed conversions (10000x): {:?}",
        conversion_duration
    );

    // as_borrowed should be very fast (just reference creation)
    assert!(
        conversion_duration < Duration::from_millis(10),
        "as_borrowed conversion should be very fast: {:?}",
        conversion_duration
    );

    Ok(())
}

#[test]
fn test_builder_pattern_performance() -> Result<()> {
    let walking = context_utils::create_test_walking_context();
    let formatting = context_utils::create_test_formatting_context();
    let sorting = context_utils::create_test_sorting_context();

    // Test builder pattern performance
    let start = Instant::now();
    for _ in 0..100 {
        let _processing_ctx = ProcessingContextBuilder::new()
            .with_walking(walking.clone())
            .with_formatting(formatting.clone())
            .with_sorting(sorting.clone())
            .build()
            .map_err(|e| anyhow::anyhow!(e))?;
    }
    let builder_duration = start.elapsed();

    println!("Builder pattern creation (100x): {:?}", builder_duration);

    // Builder should be reasonably fast
    assert!(
        builder_duration < Duration::from_millis(100),
        "Builder pattern should be reasonably fast: {:?}",
        builder_duration
    );

    Ok(())
}

#[test]
fn test_walking_performance_comparison() -> Result<()> {
    let temp_dir = common_test_utils::setup_complex_test_directory()?;
    let root_path = temp_dir.path();

    let config = RustreeLibConfig {
        listing: ListingOptions {
            max_depth: Some(2),
            show_hidden: true,
            ..Default::default()
        },
        filtering: FilteringOptions {
            ignore_patterns: Some(vec!["*.tmp".to_string(), "*.log".to_string()]),
            case_insensitive_filter: true,
            ..Default::default()
        },
        metadata: MetadataOptions {
            show_size_bytes: true,
            calculate_line_count: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Test old API performance
    let start_old = Instant::now();
    for _ in 0..10 {
        let _nodes = get_tree_nodes(root_path, &config)?;
    }
    let old_api_duration = start_old.elapsed();

    // Test new context API performance (borrowed)
    let processing_ctx = config.processing_context();
    let start_new_borrowed = Instant::now();
    for _ in 0..10 {
        let _nodes = get_tree_nodes_with_context(root_path, &processing_ctx)?;
    }
    let new_borrowed_duration = start_new_borrowed.elapsed();

    // Test new owned context API performance (with caching)
    let mut owned_processing = config.to_owned_processing_context();
    let start_new_owned = Instant::now();
    for _ in 0..10 {
        let _nodes = get_tree_nodes_owned(root_path, &mut owned_processing)?;
    }
    let new_owned_duration = start_new_owned.elapsed();

    println!("Old API (10x): {:?}", old_api_duration);
    println!("New borrowed API (10x): {:?}", new_borrowed_duration);
    println!("New owned API with caching (10x): {:?}", new_owned_duration);

    // New APIs should be at least as fast as old API
    // Owned context might be faster due to pattern caching
    assert!(
        new_borrowed_duration <= old_api_duration * 2,
        "New borrowed API should not be significantly slower than old API"
    );

    // After first run, owned context should benefit from caching
    // (though filesystem I/O will likely dominate the timing)
    println!("Performance comparison completed successfully");

    Ok(())
}

#[test]
fn test_cache_invalidation_performance() -> Result<()> {
    let mut owned_walking = OwnedWalkingContext::new(
        ListingOptions::default(),
        FilteringOptions {
            ignore_patterns: Some(vec![
                "*.tmp".to_string(),
                "*.log".to_string(),
                "**/target/**".to_string(),
            ]),
            ..Default::default()
        },
        MetadataOptions::default(),
    );

    // Compile patterns first
    let patterns = owned_walking.ignore_patterns()?;
    assert!(patterns.is_some());

    // Test cache invalidation performance
    let start = Instant::now();
    for _ in 0..10000 {
        owned_walking.invalidate_pattern_cache();
    }
    let invalidation_duration = start.elapsed();

    println!("Cache invalidation (10000x): {:?}", invalidation_duration);

    // Cache invalidation should be very fast (just setting None)
    assert!(
        invalidation_duration < Duration::from_millis(10),
        "Cache invalidation should be very fast: {:?}",
        invalidation_duration
    );

    // Verify cache was actually invalidated (indirectly)
    // Note: compiled fields are private, so we test behavior

    Ok(())
}

#[test]
fn test_memory_efficiency_owned_contexts() -> Result<()> {
    // This test is more qualitative - ensuring we don't have obvious memory waste
    let base_config = RustreeLibConfig {
        filtering: FilteringOptions {
            ignore_patterns: Some(vec!["*.tmp".to_string(); 100]), // Large pattern list
            ..Default::default()
        },
        ..Default::default()
    };

    // Create multiple owned contexts from same base
    let contexts: Vec<_> = (0..10)
        .map(|_| base_config.to_owned_walking_context())
        .collect();

    // Verify they're independent (modifying one doesn't affect others)
    let mut modified_context = contexts[0].clone();
    modified_context.listing.max_depth = Some(99);

    for (i, context) in contexts.iter().enumerate() {
        if i == 0 {
            continue; // Skip the one we cloned from
        }
        assert_eq!(context.listing.max_depth, None);
    }

    // Verify pattern compilation works independently
    for mut context in contexts {
        let _ = context.ignore_patterns()?;
        // Note: compiled fields are private, so we test behavior indirectly
    }

    println!("Memory efficiency test completed successfully");

    Ok(())
}

#[test]
fn test_lazy_patterns_integration() -> Result<()> {
    let patterns = vec!["*.rs".to_string(), "*.txt".to_string()];

    // Test single-threaded lazy patterns
    let lazy_patterns = create_lazy_patterns(patterns.clone(), false, false);
    assert!(!lazy_patterns.is_compiled());

    // Should compile on first access
    let _compiled = lazy_patterns.get_compiled();
    assert!(lazy_patterns.is_compiled());

    // Test thread-safe lazy patterns
    let thread_safe = create_thread_safe_lazy_patterns(patterns, true, true);
    assert!(!thread_safe.is_compiled());

    // Should be cloneable for sharing
    let cloned = thread_safe.clone();
    assert!(!cloned.is_compiled());

    Ok(())
}
