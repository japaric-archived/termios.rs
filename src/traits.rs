//! Traits

/// Overloaded `clear()` method
pub trait Clear<T> {
    /// Clears a flag
    fn clear(&mut self, T);
}

/// Overloaded `contains()` method
pub trait Contains<T> {
    /// Checks if the structure contains the flag
    fn contains(&self, T) -> bool;
}

/// Overloaded `get()` method
pub trait Get: Sized {
    /// Get some property
    fn get<T>(&self) -> T where T: GetFrom<Self> {
        GetFrom::get_from(self)
    }
}

/// Helper trait for the `get()` method
pub trait GetFrom<T> {
    /// Dispatch method used for `get()`
    fn get_from(&T) -> Self;
}

/// Overloaded `set()` method
pub trait Set<T> {
    /// Sets some property
    fn set(&mut self, T);
}

