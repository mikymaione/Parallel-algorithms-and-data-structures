/*
MIT License

Copyright (c) 2023 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use crate::graph::Graph;
use crate::search::find_occurrence;

mod graph;
mod color;
mod search;

fn main_graph() {
    let mut g = Graph::new();

    g.add_edge(1, 5);
    g.add_edge(1, 6);
    g.add_edge(2, 3);

    g.bfs(1);
    g.print();
}

fn main_search() {
    let sono = find_occurrence(
        String::from("Ciao mi chiamo Michele Maione e sono un informatico e sono nato a Napoli e sono bello"),
        String::from("sono"),
    );
    println!("sono: {sono}");

    let napoli = find_occurrence(
        String::from("Ciao mi chiamo Michele Maione e sono un informatico e sono nato a Napoli e sono bello"),
        String::from("Napoli"),
    );
    println!("Napoli: {napoli}");
}

fn main() {
    main_graph();
    main_search();
}