use std::collections::HashMap;
use smallvec::SmallVec;

#[derive(Default, Clone, Debug)]
///
/// Stores the neighbours of a subdivided shape.
///
pub struct AdjacentStore {
    pub(crate) subdivisions: usize,
    pub(crate) map: HashMap<usize, SmallVec<[usize; 6]>>,
}

impl AdjacentStore {
    ///
    /// Creates an empty neighbour storage.
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// Optionally returns the neighbours for a vertex.
    ///
    /// In the case of an IcoSphere, this is of length `5` or `6`.
    ///
    pub fn neighbours(&self, id: usize) -> Option<&[usize]> {
        self.map.get(&id).map(|x| &**x)
    }

    ///
    /// Creates the map given the indices of a shape.
    ///
    pub fn from_indices(indices: &[usize]) -> Self {
        let mut this = Self::new();
        this.add_triangle_indices(indices);
        this
    }

    ///
    /// Adds the indices to the map.
    ///
    pub fn add_triangle_indices(&mut self, triangles: &[usize]) {
        assert_eq!(triangles.len() % 3, 0);

        for triangle in triangles.chunks(3) {
            self.add_triangle([triangle[0], triangle[1], triangle[2]]);
        }
    }

    ///
    /// Adds a single subdivided triangle to the storage.
    ///
    fn add_triangle(&mut self, [a, b, c]: [usize; 3]) {
        let mut add_triangle = |a, b, c| {
            let vec = self.map.entry(a).or_insert_with(SmallVec::new);
            if !vec.contains(&b) {
                vec.push(b);
            }
            if !vec.contains(&c) {
                vec.push(c);
            }
        };

        add_triangle(a, b, c);
        add_triangle(b, c, a);
        add_triangle(c, a, b);
    }
}

#[cfg(feature = "adjacency")]
mod tests {
    #[allow(unused_imports)]
    use crate::{AdjacentStore, shapes::IcoSphere};

    #[test]
    fn creation() {
        let sphere = IcoSphere::new(0, |_| ());

        let mut indices = Vec::new();

        for i in 0..20 {
            sphere.get_indices(i, &mut indices);
        }

        let _ = AdjacentStore::from_indices(&indices);
    }

    #[test]
    fn correct_indices() {
        let sphere = IcoSphere::new(0, |_| ());

        let mut indices = Vec::new();

        for i in 0..20 {
            sphere.get_indices(i, &mut indices);
        }

        let store = AdjacentStore::from_indices(&indices);

        const REFERENCE_DATA: [[usize; 5]; 12] = [
            [1, 2, 3, 4, 5],
            [0, 2, 7, 6, 5],
            [0, 1, 3, 8, 7],
            [0, 4, 2, 9, 8],
            [0, 5, 10, 9, 3],
            [0, 1, 6, 10, 4],
            [5, 1, 7, 11, 10],
            [1, 2, 8, 11, 6],
            [2, 3, 9, 11, 7],
            [3, 4, 10, 11, 8],
            [4, 5, 6, 11, 9],
            [6, 7, 8, 9, 10],
        ];

        for i in 0..12 {
            let expected = REFERENCE_DATA[i];
            let actual = store.neighbours(i).unwrap();
            assert_eq!(actual.len(), 5);
            let mut values = [0; 5];
            for (x, i) in actual.iter().enumerate() {
                assert!(expected.contains(i));
                values[x] += 1;
            }
            assert_eq!(values, [1; 5]);
        }
    }
}