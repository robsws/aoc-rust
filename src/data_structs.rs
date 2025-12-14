use std::fmt::Display;
use std::hash::Hash;
use std::cmp::{self, max, min};

use priority_queue::PriorityQueue;

#[derive(Clone, Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}

pub fn manhattan_dist(a: &Coord, b: &Coord) -> usize {
    (max(a.x, b.x) - min(a.x, b.x)) +
        (max(a.y, b.y) - min(a.y, b.y))
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
pub struct SignedCoord {
    pub x: isize,
    pub y: isize
}

#[derive(Clone)]
pub struct Grid<T: Clone> {
    pub xsize: usize,
    pub ysize: usize,
    elements: Vec<T>
}

impl<'a, T: Clone> Grid<T> {
    /// Create a new grid of size xsize x ysize
    /// filled with the element 'fill_with'.
    pub fn new(
        xsize: usize,
        ysize: usize,
        fill_with: T
    ) -> Grid<T> {
        let elements = vec![fill_with; xsize * ysize];
        Grid{xsize, ysize, elements}
    }

    /// Create a new grid of size xsize x ysize
    /// with the given elements
    pub fn with_elements(
        xsize: usize,
        ysize: usize,
        elements: Vec<T>
    ) -> Grid<T> {
        Grid{xsize, ysize, elements}
    }

    /// Read the element at x,y in the grid
    pub fn get(
        &self,
        x: usize,
        y: usize
    ) -> &T {
        self.check_bounds(x, y);
        let index = self.calc_index(x, y);
        return &self.elements[index];
    }

    /// Read the element at x,y in the grid
    /// and return a mutable reference
    pub fn get_mut(
        &mut self,
        x: usize,
        y: usize
    ) -> &mut T {
        self.check_bounds(x, y);
        let index = self.calc_index(x, y);
        return &mut self.elements[index];
    }

    /// Set the element at x,y to the given value
    pub fn set(
        &mut self,
        x: usize,
        y: usize,
        val: T
    ) {
        self.check_bounds(x, y);
        let index = self.calc_index(x, y);
        self.elements[index] = val;
    }

    /// Make sure that the given x, y coords are
    /// within the bounds of the grid.
    fn check_bounds(
        &self,
        x: usize,
        y: usize
    ) {
        if x >= self.xsize || y >= self.ysize {
            panic!("Index {},{} out of bounds of grid.", x, y);
        }
    }

    /// Calculate the index of the element in the
    /// elements vector given the x and y coords.
    fn calc_index(
        &self,
        x: usize,
        y: usize
    ) -> usize {
        y*self.xsize + x
    }
}

// The lifetime specifier here makes sure that
// the elements reference living inside the iterator
// does not outlive the grid. This iterator returns
// references and hence does not consume the grid.
impl<'a, T: Clone> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = GridIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            elements: &self.elements,
            index: 0
        }
    }
}

pub struct GridIterator<'a, T> {
    elements: &'a Vec<T>,
    index: usize
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.elements.len() {
            let item = &self.elements[self.index];
            self.index += 1;
            return Some(item);
        } else {
            return None;
        }
    }
}

pub struct MinPriorityQueue<T: Hash + Eq> {
    queue: PriorityQueue<T, u32>
}

impl<T: Hash + Eq> MinPriorityQueue<T> {
    pub fn new() -> MinPriorityQueue<T> {
        MinPriorityQueue {
            queue: PriorityQueue::new()
        }
    }

    pub fn pop(&mut self) -> Option<(T, u32)> {
        match self.queue.pop() {
            None => None,
            Some((t, p)) => Some((t, u32::MAX - p))
        }
    }

    pub fn push(&mut self, item: T, priority: u32) {
        self.queue.push(item, u32::MAX - priority);
    }

    pub fn get(&self, item: &T) -> Option<(&T, u32)> {
        match self.queue.get(item) {
            None => None,
            Some((t, p)) => Some((t, u32::MAX - p))
        }
    }

    pub fn change_priority(&mut self, item: &T, new_priority: u32) {
        self.queue.change_priority(item, u32::MAX - new_priority);
    }
}

#[derive(Clone)]
pub struct NumRange {
    pub min: u64,
    pub max: u64
}

impl NumRange {

    pub fn includes(&self, val: u64) -> bool {
        val >= self.min && val <= self.max
    }

    pub fn overlaps_with(&self, other: &NumRange) -> bool {
        self.max >= other.min && self.min <= other.max
    }

    pub fn combine_with(&mut self, other: &NumRange) -> Option<NumRange> {
        if self.overlaps_with(other) {
            Some(NumRange {
                min: (cmp::min(self.min, other.min)),
                max: (cmp::max(self.max, other.max))
            })
        } else {
            None
        }
    }

    pub fn member_count(&self) -> u64 {
        self.max + 1 - self.min
    }
}

impl Display for NumRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumRange [{} to {}]", self.min, self.max)
    }
}
