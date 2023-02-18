/*
MIT License

Copyright (c) 2023 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::collections::{HashMap, HashSet, VecDeque};

use crate::color::Color;

pub struct Graph {
    // [1, 2, 5, 7, 16, 23]
    vertices: HashSet<usize>,

    // 1->[2, 5, 7], 5->[16, 23]
    adjacent: HashMap<usize, HashSet<usize>>,

    // 1->White, 5->Gray
    color: HashMap<usize, Color>,

    // 1->0, 5->1, 16->2
    distance: HashMap<usize, usize>,

    // 5->1, 2->1, 16->5
    pred: HashMap<usize, usize>,
}

impl Graph {
    /// New empty graph
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            adjacent: HashMap::new(),

            color: HashMap::new(),
            distance: HashMap::new(),
            pred: HashMap::new(),
        }
    }

    /// Reset color and distance for each vertex
    fn bleach(&mut self) {
        for v in &self.vertices {
            self.color.insert(*v, Color::White);
            self.distance.insert(*v, 0);
        }
    }

    /// Add a new Node to the vertices list
    pub fn add_node(&mut self, i: usize) {
        if !self.vertices.contains(&i) {
            self.vertices.insert(i);
        }
    }

    /// Add a new Edge between 2 vertices
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.add_node(to);

        match self.adjacent.get_mut(&from) {
            // add a new neighbor to the list
            Some(l) =>
                l.insert(to),

            // add the first neighbor to the list
            _ =>
                self.adjacent.insert(from, HashSet::from([to]))
                    .is_some(),
        };

        // create an empty neighbors list
        if !self.adjacent.contains_key(&to) {
            self.adjacent.insert(to, HashSet::new());
        }
    }

    /// Breadth-first search (BFS) is an algorithm for searching a tree data structure for a node that satisfies a given property.
    pub fn bfs(&mut self, s: usize) {
        self.bleach();

        // create a Queue
        let mut q = VecDeque::from([s]);

        self.color.insert(s, Color::Gray);
        self.distance.insert(s, 0);

        while let Some(v) = q.pop_front() { // dequeue
            for w in &self.adjacent[&v] {
                match self.color[&w] {
                    Color::White => {
                        self.color.insert(*w, Color::Gray);
                        self.distance.insert(*w, self.distance[&v] + 1);
                        self.pred.insert(*w, v);

                        q.push_back(*w); // enqueue
                    }

                    _ => {}
                }
            }

            self.color.insert(v, Color::Black);
        }
    }
}