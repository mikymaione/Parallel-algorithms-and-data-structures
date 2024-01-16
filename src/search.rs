/*
MIT License

Copyright (c) 2023 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread::{available_parallelism, spawn};

pub fn find_occurrence(content: String, word: String) -> usize {
    let processor_count = available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);

    let words = content
        .split_whitespace()
        .map(|s| s.into())
        .collect::<Vec<String>>();

    let words_count = words.len();
    let words_for_processor = words_count.div_ceil(processor_count);
    let processor_count = words_count.div_ceil(words_for_processor);

    let word = Arc::new(word);
    let words = Arc::new(words);
    let occurrence = Arc::new(Mutex::new(0usize));

    let mut threads = Vec::new();

    for processor in 0..processor_count {
        let from = processor * words_for_processor;
        let to = min(from + words_for_processor, words_count);

        let word = Arc::clone(&word);
        let words = Arc::clone(&words);
        let occurrence = Arc::clone(&occurrence);

        let thread = spawn(move || {
            for i in from..to {
                if words[i].eq(&*word) {
                    let mut occurrence = occurrence
                        .lock()
                        .unwrap_or_else(|e| panic!("Can not Lock 'occurrence': {e}"));

                    *occurrence += 1;
                }
            }
        });

        threads.push(thread);
    }

    for thread in threads {
        thread
            .join()
            .unwrap_or_else(|_| panic!("Thread panicked!"));
    }

    let occurrence = *occurrence
        .lock()
        .unwrap_or_else(|e| panic!("Can not Lock 'occurrence': {e}"));

    return occurrence;
}