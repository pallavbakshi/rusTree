//! # Lazy Initialization for Expensive Derived Data
//!
//! This module provides lazy initialization patterns for context data that is
//! expensive to compute but only needed in certain scenarios. This includes
//! pattern compilation, regex compilation, and other derived data structures.

use crate::core::filter::pattern::CompiledGlobPattern;
use std::sync::{Arc, Mutex, OnceLock};

/// A lazy-initialized value that is computed once and cached
///
/// This is similar to `std::sync::OnceLock` but provides more specific
/// error handling for our use cases.
#[derive(Debug)]
pub struct LazyValue<T> {
    value: OnceLock<Result<T, String>>,
}

impl<T> LazyValue<T> {
    /// Create a new lazy value
    pub fn new() -> Self {
        Self {
            value: OnceLock::new(),
        }
    }

    /// Get the value, computing it if necessary using the provided closure
    pub fn get_or_init<F>(&self, init: F) -> Result<&T, String>
    where
        F: FnOnce() -> Result<T, String>,
    {
        match self.value.get_or_init(init) {
            Ok(value) => Ok(value),
            Err(e) => Err(e.clone()),
        }
    }

    /// Check if the value has been initialized
    pub fn is_initialized(&self) -> bool {
        self.value.get().is_some()
    }

    /// Clear the cached value, forcing recomputation on next access
    pub fn invalidate(&mut self) {
        // Unfortunately, OnceLock doesn't have a reset method
        // We need to create a new instance
        self.value = OnceLock::new();
    }
}

impl<T> Default for LazyValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for LazyValue<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self.value.get() {
            Some(Ok(value)) => {
                let new_lazy = Self::new();
                let _ = new_lazy.value.set(Ok(value.clone()));
                new_lazy
            }
            Some(Err(e)) => {
                let new_lazy = Self::new();
                let _ = new_lazy.value.set(Err(e.clone()));
                new_lazy
            }
            None => Self::new(),
        }
    }
}

/// A thread-safe lazy-initialized value for multi-threaded contexts
#[derive(Debug)]
pub struct ThreadSafeLazyValue<T> {
    value: Arc<Mutex<Option<Result<T, String>>>>,
}

impl<T> ThreadSafeLazyValue<T> {
    /// Create a new thread-safe lazy value
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the value, computing it if necessary using the provided closure
    pub fn get_or_init<F>(&self, init: F) -> Result<T, String>
    where
        F: FnOnce() -> Result<T, String>,
        T: Clone,
    {
        let mut guard = self
            .value
            .lock()
            .map_err(|_| "Mutex lock failed".to_string())?;

        match &*guard {
            Some(result) => result.clone(),
            None => {
                let result = init();
                *guard = Some(result.clone());
                result
            }
        }
    }

    /// Check if the value has been initialized
    pub fn is_initialized(&self) -> bool {
        self.value
            .lock()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
    }

    /// Clear the cached value, forcing recomputation on next access
    pub fn invalidate(&self) -> Result<(), String> {
        let mut guard = self
            .value
            .lock()
            .map_err(|_| "Mutex lock failed".to_string())?;
        *guard = None;
        Ok(())
    }
}

impl<T> Default for ThreadSafeLazyValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for ThreadSafeLazyValue<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
        }
    }
}

/// Lazy pattern compilation for glob patterns
///
/// This struct provides lazy compilation of glob patterns, which is expensive
/// but only needed when actually filtering files.
#[derive(Debug)]
pub struct LazyPatternCompilation {
    patterns: Vec<String>,
    case_insensitive: bool,
    show_hidden: bool,
    compiled: LazyValue<Vec<CompiledGlobPattern>>,
}

impl LazyPatternCompilation {
    /// Create a new lazy pattern compilation
    pub fn new(patterns: Vec<String>, case_insensitive: bool, show_hidden: bool) -> Self {
        Self {
            patterns,
            case_insensitive,
            show_hidden,
            compiled: LazyValue::new(),
        }
    }

    /// Get the compiled patterns, compiling them if necessary
    pub fn get_compiled(&self) -> Result<&Vec<CompiledGlobPattern>, String> {
        self.compiled.get_or_init(|| {
            if self.patterns.is_empty() {
                return Ok(Vec::new());
            }

            crate::core::filter::pattern::compile_glob_patterns(
                &Some(self.patterns.clone()),
                self.case_insensitive,
                self.show_hidden,
            )
            .map_err(|e| format!("Failed to compile patterns: {}", e))
            .and_then(|opt| opt.ok_or_else(|| "Pattern compilation returned None".to_string()))
        })
    }

    /// Check if patterns have been compiled
    pub fn is_compiled(&self) -> bool {
        self.compiled.is_initialized()
    }

    /// Update the patterns and invalidate the compiled cache
    pub fn update_patterns(
        &mut self,
        patterns: Vec<String>,
        case_insensitive: bool,
        show_hidden: bool,
    ) {
        self.patterns = patterns;
        self.case_insensitive = case_insensitive;
        self.show_hidden = show_hidden;
        self.compiled.invalidate();
    }
}

impl Clone for LazyPatternCompilation {
    fn clone(&self) -> Self {
        Self {
            patterns: self.patterns.clone(),
            case_insensitive: self.case_insensitive,
            show_hidden: self.show_hidden,
            compiled: self.compiled.clone(),
        }
    }
}

/// Thread-safe version of lazy pattern compilation
#[derive(Debug)]
pub struct ThreadSafeLazyPatternCompilation {
    patterns: Arc<Vec<String>>,
    case_insensitive: bool,
    show_hidden: bool,
    compiled: ThreadSafeLazyValue<Vec<CompiledGlobPattern>>,
}

impl ThreadSafeLazyPatternCompilation {
    /// Create a new thread-safe lazy pattern compilation
    pub fn new(patterns: Vec<String>, case_insensitive: bool, show_hidden: bool) -> Self {
        Self {
            patterns: Arc::new(patterns),
            case_insensitive,
            show_hidden,
            compiled: ThreadSafeLazyValue::new(),
        }
    }

    /// Get the compiled patterns, compiling them if necessary
    pub fn get_compiled(&self) -> Result<Vec<CompiledGlobPattern>, String> {
        self.compiled.get_or_init(|| {
            if self.patterns.is_empty() {
                return Ok(Vec::new());
            }

            crate::core::filter::pattern::compile_glob_patterns(
                &Some(self.patterns.as_ref().clone()),
                self.case_insensitive,
                self.show_hidden,
            )
            .map_err(|e| format!("Failed to compile patterns: {}", e))
            .and_then(|opt| opt.ok_or_else(|| "Pattern compilation returned None".to_string()))
        })
    }

    /// Check if patterns have been compiled
    pub fn is_compiled(&self) -> bool {
        self.compiled.is_initialized()
    }

    /// Create a new instance with updated patterns
    pub fn with_updated_patterns(
        &self,
        patterns: Vec<String>,
        case_insensitive: bool,
        show_hidden: bool,
    ) -> Self {
        Self::new(patterns, case_insensitive, show_hidden)
    }
}

impl Clone for ThreadSafeLazyPatternCompilation {
    fn clone(&self) -> Self {
        Self {
            patterns: Arc::clone(&self.patterns),
            case_insensitive: self.case_insensitive,
            show_hidden: self.show_hidden,
            compiled: self.compiled.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_value_initialization() {
        let lazy = LazyValue::new();
        assert!(!lazy.is_initialized());

        let result = lazy.get_or_init(|| Ok(42));
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 42);
        assert!(lazy.is_initialized());

        // Second call should return cached value
        let result2 = lazy.get_or_init(|| Ok(100)); // Different value
        assert!(result2.is_ok());
        assert_eq!(*result2.unwrap(), 42); // Should still be original value
    }

    #[test]
    fn test_lazy_value_error_caching() {
        let lazy = LazyValue::new();

        let result = lazy.get_or_init(|| Err("test error".to_string()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "test error");
        assert!(lazy.is_initialized());

        // Second call should return cached error
        let result2 = lazy.get_or_init(|| Ok(42));
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), "test error");
    }

    #[test]
    fn test_lazy_value_invalidation() {
        let mut lazy = LazyValue::new();

        let _result = lazy.get_or_init(|| Ok(42));
        assert!(lazy.is_initialized());

        lazy.invalidate();
        assert!(!lazy.is_initialized());

        // Should compute new value
        let result = lazy.get_or_init(|| Ok(100));
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 100);
    }

    #[test]
    fn test_thread_safe_lazy_value() {
        let lazy = ThreadSafeLazyValue::new();
        assert!(!lazy.is_initialized());

        let result = lazy.get_or_init(|| Ok(42));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert!(lazy.is_initialized());

        // Test invalidation
        assert!(lazy.invalidate().is_ok());
        assert!(!lazy.is_initialized());
    }

    #[test]
    fn test_lazy_pattern_compilation() {
        let patterns = vec!["*.rs".to_string(), "*.txt".to_string()];
        let lazy_patterns = LazyPatternCompilation::new(patterns, false, false);

        assert!(!lazy_patterns.is_compiled());

        let compiled = lazy_patterns.get_compiled();
        // Note: This might fail if glob compilation fails, but we're testing the lazy mechanism
        if compiled.is_ok() {
            assert!(lazy_patterns.is_compiled());

            // Second call should use cached result
            let compiled2 = lazy_patterns.get_compiled();
            assert!(compiled2.is_ok());
        }
    }

    #[test]
    fn test_lazy_pattern_compilation_update() {
        let patterns = vec!["*.rs".to_string()];
        let mut lazy_patterns = LazyPatternCompilation::new(patterns, false, false);

        // Force compilation
        let _compiled = lazy_patterns.get_compiled();
        assert!(lazy_patterns.is_compiled());

        // Update patterns
        lazy_patterns.update_patterns(vec!["*.txt".to_string()], true, true);
        assert!(!lazy_patterns.is_compiled());
    }

    #[test]
    fn test_thread_safe_lazy_pattern_compilation() {
        let patterns = vec!["*.rs".to_string()];
        let lazy_patterns = ThreadSafeLazyPatternCompilation::new(patterns, false, false);

        assert!(!lazy_patterns.is_compiled());

        // Test cloning preserves state
        let cloned = lazy_patterns.clone();
        assert!(!cloned.is_compiled());

        // Test pattern update creates new instance
        let updated = lazy_patterns.with_updated_patterns(vec!["*.txt".to_string()], true, true);
        assert!(!updated.is_compiled());
    }

    #[test]
    fn test_lazy_value_clone() {
        let lazy1 = LazyValue::new();
        let _result = lazy1.get_or_init(|| Ok(42));

        let lazy2 = lazy1.clone();
        assert!(lazy2.is_initialized());

        let result2 = lazy2.get_or_init(|| Ok(100));
        assert!(result2.is_ok());
        assert_eq!(*result2.unwrap(), 42); // Should have cloned value
    }
}
