# Day 7: Getting Tired of Parsing

When I started AOC 2020, my goal was to use only the Standard Library. I figured that would be the best way to learn as much about it as I could. And for six days I made that work. Day 2 and Day 4 were kind of rough: I'd spent more effort than I cared to parsing the input data. However, I slogged through it and felt good afterward having learned how to use `FromStr` and `parse()`, as well as implementing my own `Error` types.

Not long after starting Day 7, though, I noticed how I'd once again have to spend a good chunk of effort working out how to parse the input with rather basic operations, and I'd had enough. This time, I pulled in the [regex](https://crates.io/crates/regex) crate to do the work of parsing. And because regular expression compilation can be expensive in terms of cycles, I took the author's recommendation and pulled in the [lazy_static](https://crates.io/crates/lazy_static) crate as well, so the regexes I use are compiled only once at runtime.

Despite how tired I am of working on parsing the input for each day in this project, I'm concerned that I'm short-changing myself by using external crates, that with patience I'll find easier and easier ways to parse strings with only `std` as I continue learning about it. Maybe I'll redo this day. I don't know.
