// Heavily inspired by http::Extensions

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};

type AnyMap = HashMap<TypeId, Box<dyn AnyClone + Send + Sync>, BuildHasherDefault<IdHasher>>;

/// Hashes come from compiler. The IdHasher just holds the u64 of
/// the TypeId, and then returns it, instead of doing any bit fiddling.
#[derive(Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

/// A pile of variable data types
#[derive(Clone, Default)]
pub struct DataPile {
    // If extensions are never used, no need to carry around an empty HashMap.
    // That's 3 words. Instead, this is only 1 word.
    map: Box<AnyMap>,
}

impl DataPile {
    /// Create an empty `DataPile`.
    #[inline]
    pub fn new() -> DataPile {
        DataPile {
            map: Box::default(),
        }
    }

    /// Insert data type into pile.
    ///
    /// If type exists, replaces and returns previous value.
    pub fn insert<T: Clone + Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(|boxed| boxed.into_any().downcast().ok().map(|boxed| *boxed))
    }

    /// Get a reference to a type in the pile.
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.map
            .as_ref()
            .get(&TypeId::of::<T>())
            .and_then(|boxed| (**boxed).as_any().downcast_ref())
    }

    /// Get a mutable reference to a type in the pile.
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.map
            .as_mut()
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
    }

    /// Get a mutable reference to a type in the pile, or insert it if it doesn't exist.
    pub fn get_or_insert<T: Clone + Send + Sync + 'static>(&mut self, value: T) -> &mut T {
        self.get_or_insert_with(|| value)
    }

    /// Get a mutable reference to a type in the pile, or insert it if it doesn't exist.
    pub fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(
        &mut self,
        f: F,
    ) -> &mut T {
        let out = self
            .map
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(f()));

        (**out).as_any_mut().downcast_mut().unwrap()
    }

    /// Get a mutable reference to a type in the pile, or insert the default if it doesn't exist.
    pub fn get_or_insert_default<T: Default + Clone + Send + Sync + 'static>(&mut self) -> &mut T {
        self.get_or_insert_with(T::default)
    }

    /// Remove a type from the pile, returning it if it existed.
    pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.map
            .as_mut()
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.into_any().downcast().ok().map(|boxed| *boxed))
    }

    /// Clear the pile, removing all data.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Returns true if the pile contains no data.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the number of data types in the pile.
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Extends the pile with another pile.
    ///
    /// If a type exists in both piles, the other pile's type will overwrite the current pile's data.
    pub fn extend(&mut self, other: Self) {
        self.map.extend(*other.map);
    }
}

impl fmt::Debug for DataPile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataPile").finish()
    }
}

trait AnyClone: Any {
    fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Clone + Send + Sync + 'static> AnyClone for T {
    fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Clone for Box<dyn AnyClone + Send + Sync> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}
