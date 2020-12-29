# Day 14: Fewer Strings; Investigating Macros and Attributes
This note has nothing to do with the day's puzzle. Okay, hold on. I'll jot down that I learned how to convert integers to/from their binary representation in strings. There.

No, today I went through and replaced all of the `&[String]` function parameters and replaced them with `&[&str]` because I felt like I was doing far too much copying of strings. I had been using `&[String]` all over the place because the `DayRunner` struct I created back on [Day 9][1] kept a `Vec<String>` for the input data. Now, `main()` reads the input from the file and converts the resulting `Vec<String>` to a `Vec<&str>`, and passes that to the `days` API.

After replacing all of the function parameters, I ran into a lifetime specifier requirement for the struct's `data` field. That's when I asked whether the struct really needed to keep a copy of the data. The answer was "no". I replaced the `DayRunner` struct with a tuple that holds pointers to `part_one` and `part_two` functions. Now, instead of calling `run()` on the struct, `main()` calls `days::run()`.

The result of all this:
* No more copying `Strings` throughout the program. I think the program runs faster, though I have no benchmark to compare against.
* Less boilerplate to use when starting a new day.

## Macros and Attributes
I do not know how to create my own macros and attributes. I'm taking the time to learn because I think they'll help simplify even further the process of starting a new day.

I cheated for the first time on Day 13, Part 2. Stumped, I looked up how other programmers answered it. During my search, I came across a Rust programmer who used an `aoc` attribute. It was applied to a function to identify which day and part it solved. I'm curious to find out how useful that can be.


[1]: notes/day_09.md
