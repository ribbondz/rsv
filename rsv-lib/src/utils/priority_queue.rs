use std::collections::BinaryHeap;
use std::hash::Hash;

pub struct Item<T> {
    pub line_n: usize,
    pub priority: f64,
    pub item: T,
}

impl<T> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Eq for Item<T> {}

impl<T> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.partial_cmp(&other.priority).unwrap()
    }
}

impl<T> Item<T> {
    pub fn line_n_as_string(&self) -> String {
        self.line_n.to_string()
    }
}
pub struct PriorityQueue<T: Hash + Eq> {
    pub max_priority: f64,
    pub capacity: usize,
    pub count: usize,
    pub v: BinaryHeap<Item<T>>,
}

impl<T> PriorityQueue<T>
where
    T: Hash + Eq + Clone,
{
    pub fn with_capacity(n: usize) -> Self {
        PriorityQueue {
            max_priority: 0.0,
            capacity: n,
            count: 0,
            v: BinaryHeap::new(),
        }
    }

    pub fn can_insert(&self, priority: f64) -> bool {
        // println!("{},{}", self.capacity, self.count);
        priority <= self.max_priority || self.count < self.capacity
    }

    pub fn push(&mut self, line_n: usize, priority: f64, item: T) {
        self.v.push(Item {
            line_n,
            priority,
            item,
        });

        if self.max_priority < priority {
            self.max_priority = priority;
        }

        if self.count >= self.capacity {
            self.v.pop();
        } else {
            self.count += 1;
        }
    }

    pub fn into_sorted_items(self) -> Vec<Item<T>> {
        let mut v = self.v.into_vec();
        v.sort_by_key(|a| a.line_n);
        v
    }
}
