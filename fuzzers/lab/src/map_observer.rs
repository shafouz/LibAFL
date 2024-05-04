use ahash::RandomState;
use libafl::inputs::UsesInput;
use libafl::observers::MapObserver;
use libafl::observers::Observer;
use libafl_bolts::AsSlice;
use libafl_bolts::AsSliceMut;
use libafl_bolts::HasLen;
use libafl_bolts::Named;
use num_traits::Bounded;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(bound = "T: serde::de::DeserializeOwned")]
pub struct CustomMapObserver<T>
where
    T: Default + Copy + Serialize + 'static,
{
    name: Cow<'static, str>,
    initial: T,
    map: Vec<T>,
}

impl<T> CustomMapObserver<T>
where
    T: Default + Copy + Serialize + serde::de::DeserializeOwned,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name: Cow::from(name),
            map: Vec::default(),
            initial: T::default(),
        }
    }
}

impl<T> AsRef<Self> for CustomMapObserver<T>
where
    T: Default + Copy + Serialize + serde::de::DeserializeOwned,
{
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsMut<Self> for CustomMapObserver<T>
where
    T: Default + Copy + Serialize + serde::de::DeserializeOwned,
{
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T> HasLen for CustomMapObserver<T>
where
    T: Default + Copy + Serialize + serde::de::DeserializeOwned,
{
    fn len(&self) -> usize {
        self.map.as_slice().len()
    }
}

impl<T> Deref for CustomMapObserver<T>
where
    T: Default + Copy + Serialize + serde::de::DeserializeOwned,
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.map
    }
}

impl<T> DerefMut for CustomMapObserver<T>
where
    T: Default + Copy + Serialize + serde::de::DeserializeOwned,
{
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.map
    }
}

impl<T> Hash for CustomMapObserver<T>
where
    T: 'static + Hash + Default + Copy + Serialize + serde::de::DeserializeOwned + Debug,
{
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_slice().hash(hasher);
    }
}

impl<T> MapObserver for CustomMapObserver<T>
where
    T: Serialize
        + serde::de::DeserializeOwned
        + Hash
        + PartialEq
        + Bounded
        + Default
        + Copy
        + Debug
        + 'static,
{
    type Entry = T;

    fn get(&self, idx: usize) -> Self::Entry {
        self.as_slice()[idx]
    }

    fn set(&mut self, idx: usize, val: Self::Entry) {
        self.as_slice_mut()[idx] = val;
    }

    fn usable_count(&self) -> usize {
        self.as_slice().len()
    }

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

    fn hash_simple(&self) -> u64 {
        RandomState::with_seeds(1, 0, 0, 0).hash_one(self)
    }

    fn initial(&self) -> Self::Entry {
        self.initial
    }

    fn reset_map(&mut self) -> Result<(), libafl_bolts::prelude::Error> {
        let initial = self.initial();
        let cnt = self.usable_count();
        let map = self.as_slice_mut();
        for x in &mut map[0..cnt] {
            *x = initial;
        }
        Ok(())
    }

    fn to_vec(&self) -> Vec<Self::Entry> {
        self.as_slice().to_vec()
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

impl<S, T> Observer<S> for CustomMapObserver<T>
where
    S: UsesInput,
    T: Default + Copy + Serialize + serde::de::DeserializeOwned,
    Self: MapObserver,
{
    fn pre_exec(
        &mut self,
        _state: &mut S,
        _input: &<S as UsesInput>::Input,
    ) -> Result<(), libafl_bolts::prelude::Error> {
        self.reset_map()
    }
}

impl<T> Named for CustomMapObserver<T>
where
    T: Default + Copy + Serialize + serde::de::DeserializeOwned,
{
    fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
}
