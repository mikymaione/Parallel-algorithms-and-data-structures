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
    vertices: HashSet<i32>,
    adjacent: HashMap<i32, HashSet<i32>>,

    color: HashMap<i32, Color>,
    distance: HashMap<i32, i32>,
    pred: HashMap<i32, i32>,
}

impl Graph {
    pub fn build_graph() -> Self {
        Self {
            vertices: HashSet::new(),
            adjacent: HashMap::new(),

            color: HashMap::new(),
            distance: HashMap::new(),
            pred: HashMap::new(),
        }
    }

    fn bleach(&mut self) {
        for v in &self.vertices {
            self.color.insert(*v, Color::White);
            self.distance.insert(*v, 0);
        }
    }

    fn add_node(&mut self, i: i32) {
        if !self.vertices.contains(&i) {
            self.vertices.insert(i);
        }
    }

    pub fn add_edge(&mut self, from: i32, to: i32) {
        self.add_node(to);

        match self.adjacent.get_mut(&from) {
            Some(l) =>
                l.insert(to),

            None =>
                self.adjacent.insert(from, HashSet::from([to]))
                    .is_some(),
        };

        if !self.adjacent.contains_key(&to) {
            self.adjacent.insert(to, HashSet::new());
        }
    }

    pub fn bfs(&mut self, s: i32) {
        self.bleach();

        let mut q = VecDeque::from([s]);

        self.color.insert(s, Color::Gray);
        self.distance.insert(s, 0);

        while let Some(v) = q.pop_front() {
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