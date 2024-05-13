use core::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
};
use libafl::{inputs::HasBytesVec, prelude::MapObserver};
use std::borrow::Cow;

use ahash::RandomState;
use libafl_bolts::{ownedref::OwnedMutSlice, AsSlice, AsSliceMut, HasLen, Named, Truncate};
use num_traits::Bounded;
use serde::{Deserialize, Serialize};

use libafl::{
    inputs::UsesInput,
    observers::{DifferentialObserver, Observer, ObserversTuple},
    Error,
};

/// The Map Observer retrieves the state of a map,
/// that will get updated by the target.
/// A well-known example is the AFL-Style coverage map.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(bound = "T: serde::de::DeserializeOwned")]
#[allow(clippy::unsafe_derive_deserialize)]
pub struct CustomStdMapObserver<'a, T, const DIFFERENTIAL: bool>
where
    T: Default + Copy + 'static + Serialize,
{
    map: OwnedMutSlice<'a, T>,
    initial: T,
    name: Cow<'static, str>,
}

impl<'a, S, T> Observer<S> for CustomStdMapObserver<'a, T, false>
where
    S: UsesInput,
    S::Input: HasBytesVec,
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + Hash
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug
        + ToString,
{
    #[inline]
    fn pre_exec(&mut self, _state: &mut S, _input: &S::Input) -> Result<(), Error> {
        // let map = self.map.to_vec();
        // let mut string = String::new();
        // for el in map {
        //     string.push_str(&el.to_string());
        //     string.push_str(" ");
        // }
        // eprintln!("{:#?}", String::from_utf8_lossy(_input.bytes()));
        // eprintln!("{:#?}", string);
        self.reset_map()
    }
}

impl<'a, S, T> Observer<S> for CustomStdMapObserver<'a, T, true>
where
    S: UsesInput,
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
}

impl<'a, T, const DIFFERENTIAL: bool> Named for CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Default + Copy + 'static + Serialize + serde::de::DeserializeOwned,
{
    #[inline]
    fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
}

impl<'a, T, const DIFFERENTIAL: bool> HasLen for CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Default + Copy + 'static + Serialize + serde::de::DeserializeOwned,
{
    #[inline]
    fn len(&self) -> usize {
        self.map.as_slice().len()
    }
}

impl<'a, 'it, T, const DIFFERENTIAL: bool> IntoIterator
    for &'it CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + Hash
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
    type Item = <Iter<'it, T> as Iterator>::Item;
    type IntoIter = Iter<'it, T>;

    fn into_iter(self) -> Self::IntoIter {
        let cnt = self.usable_count();
        self.as_slice()[..cnt].iter()
    }
}

impl<'a, 'it, T, const DIFFERENTIAL: bool> IntoIterator
    for &'it mut CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + Hash
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
    type Item = <IterMut<'it, T> as Iterator>::Item;
    type IntoIter = IterMut<'it, T>;

    fn into_iter(self) -> Self::IntoIter {
        let cnt = self.usable_count();
        self.as_slice_mut()[..cnt].iter_mut()
    }
}

impl<'a, T, const DIFFERENTIAL: bool> CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + Hash
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
    /// Returns an iterator over the map.
    pub fn iter(&self) -> Iter<'_, T> {
        <&Self as IntoIterator>::into_iter(self)
    }

    /// Returns a mutable iterator over the map.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        <&mut Self as IntoIterator>::into_iter(self)
    }
}

impl<'a, T, const DIFFERENTIAL: bool> Hash for CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + Hash
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_slice().hash(hasher);
    }
}

impl<'a, T, const DIFFERENTIAL: bool> AsRef<Self> for CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Default + Copy + 'static + Serialize,
{
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'a, T, const DIFFERENTIAL: bool> AsMut<Self> for CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Default + Copy + 'static + Serialize,
{
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'a, T, const DIFFERENTIAL: bool> MapObserver for CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + Hash
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
    type Entry = T;

    #[inline]
    fn get(&self, pos: usize) -> T {
        self.as_slice()[pos]
    }

    fn set(&mut self, pos: usize, val: T) {
        self.map.as_slice_mut()[pos] = val;
    }

    /// Count the set bytes in the map
    fn count_bytes(&self) -> u64 {
        let initial = self.initial();
        let cnt = self.usable_count();
        let map = self.as_slice();
        let mut res = 0;
        for x in &map[0..cnt] {
            if *x != initial {
                res += 1;
            }
        }
        res
    }

    #[inline]
    fn usable_count(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    fn hash_simple(&self) -> u64 {
        RandomState::with_seeds(0, 0, 0, 0).hash_one(self)
    }

    #[inline]
    fn initial(&self) -> T {
        self.initial
    }

    fn to_vec(&self) -> Vec<T> {
        self.as_slice().to_vec()
    }

    /// Reset the map
    #[inline]
    fn reset_map(&mut self) -> Result<(), Error> {
        // Normal memset, see https://rust.godbolt.org/z/Trs5hv
        let initial = self.initial();
        let cnt = self.usable_count();
        let map = self.as_slice_mut();
        for x in &mut map[0..cnt] {
            *x = initial;
        }
        Ok(())
    }

    fn how_many_set(&self, indexes: &[usize]) -> usize {
        let initial = self.initial();
        let cnt = self.usable_count();
        let map = self.as_slice();
        let mut res = 0;
        for i in indexes {
            if *i < cnt && map[*i] != initial {
                res += 1;
            }
        }
        res
    }
}

impl<'a, T, const DIFFERENTIAL: bool> Truncate for CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
    fn truncate(&mut self, new_len: usize) {
        self.map.truncate(new_len);
    }
}

impl<'a, T, const DIFFERENTIAL: bool> Deref for CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Default + Copy + 'static + Serialize + serde::de::DeserializeOwned + Debug,
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.map
    }
}

impl<'a, T, const DIFFERENTIAL: bool> DerefMut for CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Default + Copy + 'static + Serialize + serde::de::DeserializeOwned + Debug,
{
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.map
    }
}

impl<'a, T, const DIFFERENTIAL: bool> CustomStdMapObserver<'a, T, DIFFERENTIAL>
where
    T: Default + Copy + 'static + Serialize + serde::de::DeserializeOwned,
{
    /// Creates a new [`MapObserver`]
    ///
    /// # Safety
    /// Will get a pointer to the map and dereference it at any point in time.
    /// The map must not move in memory!
    #[must_use]
    unsafe fn maybe_differential<S>(name: S, map: &'a mut [T]) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        let len = map.len();
        let ptr = map.as_mut_ptr();
        Self::maybe_differential_from_mut_ptr(name, ptr, len)
    }

    /// Creates a new [`MapObserver`] from an [`OwnedMutSlice`]
    #[must_use]
    fn maybe_differential_from_mut_slice<S>(name: S, map: OwnedMutSlice<'a, T>) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        CustomStdMapObserver {
            name: name.into(),
            map,
            initial: T::default(),
        }
    }

    /// Creates a new [`MapObserver`] with an owned map
    #[must_use]
    fn maybe_differential_owned<S>(name: S, map: Vec<T>) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            map: OwnedMutSlice::from(map),
            name: name.into(),
            initial: T::default(),
        }
    }

    /// Creates a new [`MapObserver`] from an [`OwnedMutSlice`] map.
    ///
    /// # Safety
    /// Will dereference the owned slice with up to len elements.
    #[must_use]
    fn maybe_differential_from_ownedref<S>(name: S, map: OwnedMutSlice<'a, T>) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            map,
            name: name.into(),
            initial: T::default(),
        }
    }

    /// Creates a new [`MapObserver`] from a raw pointer
    ///
    /// # Safety
    /// Will dereference the `map_ptr` with up to len elements.
    unsafe fn maybe_differential_from_mut_ptr<S>(name: S, map_ptr: *mut T, len: usize) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential_from_mut_slice(
            name,
            OwnedMutSlice::from_raw_parts_mut(map_ptr, len),
        )
    }

    /// Gets the initial value for this map, mutably
    pub fn initial_mut(&mut self) -> &mut T {
        &mut self.initial
    }

    /// Gets the backing for this map
    pub fn map(&self) -> &OwnedMutSlice<'a, T> {
        &self.map
    }

    /// Gets the backing for this map mutably
    pub fn map_mut(&mut self) -> &mut OwnedMutSlice<'a, T> {
        &mut self.map
    }
}

impl<'a, T> CustomStdMapObserver<'a, T, false>
where
    T: Default + Copy + 'static + Serialize + serde::de::DeserializeOwned,
{
    /// Creates a new [`MapObserver`]
    ///
    /// # Safety
    /// The observer will keep a pointer to the map.
    /// Hence, the map may never move in memory.
    #[must_use]
    pub unsafe fn new<S>(name: S, map: &'a mut [T]) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential(name, map)
    }

    /// Creates a new [`MapObserver`] from an [`OwnedMutSlice`]
    pub fn from_mut_slice<S>(name: S, map: OwnedMutSlice<'a, T>) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential_from_mut_slice(name, map)
    }

    /// Creates a new [`MapObserver`] with an owned map
    #[must_use]
    pub fn owned<S>(name: S, map: Vec<T>) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential_owned(name, map)
    }

    /// Creates a new [`MapObserver`] from an [`OwnedMutSlice`] map.
    ///
    /// # Note
    /// Will dereference the owned slice with up to len elements.
    #[must_use]
    pub fn from_ownedref<S>(name: S, map: OwnedMutSlice<'a, T>) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential_from_ownedref(name, map)
    }

    /// Creates a new [`MapObserver`] from a raw pointer
    ///
    /// # Safety
    /// Will dereference the `map_ptr` with up to len elements.
    pub unsafe fn from_mut_ptr<S>(name: S, map_ptr: *mut T, len: usize) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential_from_mut_ptr(name, map_ptr, len)
    }
}

impl<'a, T> CustomStdMapObserver<'a, T, true>
where
    T: Default + Copy + 'static + Serialize + serde::de::DeserializeOwned,
{
    /// Creates a new [`MapObserver`] in differential mode
    ///
    /// # Safety
    /// Will get a pointer to the map and dereference it at any point in time.
    /// The map must not move in memory!
    #[must_use]
    pub unsafe fn differential<S>(name: S, map: &'a mut [T]) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential(name, map)
    }

    /// Creates a new [`MapObserver`] with an owned map in differential mode
    #[must_use]
    pub fn differential_owned<S>(name: S, map: Vec<T>) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential_owned(name, map)
    }

    /// Creates a new [`MapObserver`] from an [`OwnedMutSlice`] map in differential mode.
    ///
    /// # Note
    /// Will dereference the owned slice with up to len elements.
    #[must_use]
    pub fn differential_from_ownedref<S>(name: S, map: OwnedMutSlice<'a, T>) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential_from_ownedref(name, map)
    }

    /// Creates a new [`MapObserver`] from a raw pointer in differential mode
    ///
    /// # Safety
    /// Will dereference the `map_ptr` with up to len elements.
    pub unsafe fn differential_from_mut_ptr<S>(name: S, map_ptr: *mut T, len: usize) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::maybe_differential_from_mut_ptr(name, map_ptr, len)
    }
}

impl<'a, OTA, OTB, S, T> DifferentialObserver<OTA, OTB, S> for CustomStdMapObserver<'a, T, true>
where
    OTA: ObserversTuple<S>,
    OTB: ObserversTuple<S>,
    S: UsesInput,
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
}
