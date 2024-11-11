use glam::IVec2;
use hashbrown::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Default)]
pub struct Plane<T: Default> {
    map: HashMap<IVec2, T>,
    default_value: T,
}

impl<T: Default> Plane<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn inner_hash_map(&self) -> &HashMap<IVec2, T> {
        &self.map
    }
    #[inline]
    pub fn inner_hash_map_mut(&mut self) -> &mut HashMap<IVec2, T> {
        &mut self.map
    }

    #[inline]
    pub fn into_hash_map(self) -> HashMap<IVec2, T> {
        self.map
    }

    pub fn from_hash_map(map: HashMap<IVec2, T>) -> Self {
        Self {
            map,
            ..Default::default()
        }
    }

    /// Access a certain element on the plane-2d.
    /// Returns [`Default`] value if uninitialized element is being accessed.
    pub fn get(&self, pos: IVec2) -> &T {
        let val = self.map.get(&pos);
        val.unwrap_or(&self.default_value)
    }

    /// Mutable access to a certain element on the plane-2d.
    /// Returns [`Default`] value if uninitialized element is being accessed.
    pub fn get_mut(&mut self, pos: IVec2) -> &mut T {
        self.map.entry(pos).or_default()
    }

    /// Insert element at the coordinate.
    /// Returns [`Default`] value if uninitialized element is being accessed.
    pub fn insert(&mut self, pos: IVec2, value: T) -> T {
        self.map.insert(pos, value).unwrap_or_default()
    }

    /// Iterates over all the items within the rectangle area inclusively.
    /// Returns [`Default`] value if uninitialized element is being accessed.
    /// Order of iteration is deterministic, but can change in future versions.
    pub fn iter_rect(&self, min: IVec2, max: IVec2) -> impl Iterator<Item = (IVec2, &T)> {
        (min.x..=max.x).flat_map(move |x| {
            (min.y..=max.y).map(move |y| (IVec2::new(x, y), self.get(IVec2::new(x, y))))
        })
    }

    /// Mutably iterates over all the items within the rectangle area inclusively.
    /// Returns [`Default`] value if uninitialized element is being accessed.
    /// Order of iteration is deterministic, but can change in future versions .
    pub fn foreach_rect_mut(
        &mut self,
        min: IVec2,
        max: IVec2,
        mut func: impl FnMut(IVec2, &mut T),
    ) {
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let pos = IVec2::new(x, y);
                func(pos, self.get_mut(pos))
            }
        }
    }

    /// Iterate over all the elements stored inside the grid and hashmap. May return value from HashMap even if it is overlapping with Grid   
    pub fn iter_all(&self) -> impl Iterator<Item = (IVec2, &T)> {
        self.map.iter().map(|(pos, elem)| (*pos, elem))
    }

    /// Mutably iterate over all the elements stored inside the grid and hashmap. May return value from HashMap even if it is overlapping with Grid   
    pub fn iter_all_mut(&mut self) -> impl Iterator<Item = (IVec2, &mut T)> {
        self.map.iter_mut().map(|(pos, elem)| (*pos, elem))
    }

    /// Iterate over all the elements stored inside the grid and hashmap. May return value from HashMap even if it is overlapping with Grid   
    pub fn into_iter_all(self) -> impl Iterator<Item = (IVec2, T)> {
        self.map.into_iter()
    }
}

impl<T> Plane<Option<T>> {
    /// Iterate over all the initialized elements
    pub fn iter(&self) -> impl Iterator<Item = (IVec2, &T)> {
        self.map
            .iter()
            .filter_map(|(pos, elem)| elem.as_ref().map(|el| (*pos, el)))
    }

    /// Mutably iterate over all the initialized elements
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (IVec2, &mut T)> {
        self.map
            .iter_mut()
            .filter_map(|(pos, elem)| elem.as_mut().map(|el| (*pos, el)))
    }

    /// Consume plane-2d to get all the initialized elements
    pub fn into_iter(self) -> impl Iterator<Item = (IVec2, T)> {
        self.map
            .into_iter()
            .filter_map(|(pos, elem)| elem.map(|el| (pos, el)))
    }
}

impl<T: Default> From<HashMap<IVec2, T>> for Plane<T> {
    #[inline]
    fn from(map: HashMap<IVec2, T>) -> Self {
        Self::from_hash_map(map)
    }
}

impl<T: Default> Index<IVec2> for Plane<T> {
    type Output = T;
    #[inline]
    fn index(&self, pos: IVec2) -> &Self::Output {
        self.get(pos)
    }
}

impl<T: Default> IndexMut<IVec2> for Plane<T> {
    #[inline]
    fn index_mut(&mut self, pos: IVec2) -> &mut Self::Output {
        self.get_mut(pos)
    }
}
