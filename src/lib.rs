//! Generic identification traits

/// Implementer can be identified.
pub trait Identified<T> {
    fn get_id(&self) -> Option<T>;
}
