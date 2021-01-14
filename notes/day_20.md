# Day 20: Whew!
This day was rough. Here's why.

## `HashMap` Uses a Random Seed
Rust's developers had security in mind when they implemented `HashMap`. To prevent HashDoS attacks, the collection randomly seeds its hashing algorithm, so two instances of `HashMap` produce different hashes for the same key. This adds a layer of difficulty to unit testing. When the code under test uses a `HashMap`, it produces a collection whose iterators do not traverse its elements in a predictable order.

Unfortunately, the random hashing caused a problem for my tile-graphing algorithm because the tile parser stored the ungraphed tiles in a `HashMap`. As a result, the tile-graphing algorithm could not process the tiles in the same order as it did during a previous test execution.

It's clear to me now I could have used a `Vec` to store the ungraphed tiles. The graphing algorithm would then process the tiles in a predictable order. The tiles would then have had predictable orientations relative to each other, which would have been easy to compare in a unit test.

## A Little Detail Got Me
I spent most of my time trying to get my code to correctly link the tiles to each other and orient them relative to each other. The greatest pain was caused by a flaw in my algorithm.

Each tile had eight "edge values" grouped into four pairs. The first pair was for the top edge of the tile, the second pair was for the right edge, and so on for the bottom and left edges. The first value in each pair was the numeric value after treating the hash marks and periods along the edge as ones and zeroes, respectively, of a binary number. The second value was the numeric value of those ones and zeroes in reverse order.

My algorithm considered the "normal" direction of the top and bottom edges to be left-to-right, and it was top-to-bottom for the left and right edges. However, this caused a problem when deciding whether a neighbor tile needed to be flipped. This decision depended on whether the neighbor tile's edge value was calculated in the "normal" or the reverse direction compared to the originating tile. This determination was correct when the neighbor tile was the top- or right-hand neighbor, but it was wrong for bottom- and left-hand neighbors. The "normal" direction needed to be right-to-left for the bottom edge and bottom-to-top for the left edge.

## Iterators vs. Rust's Borrowing Rules
Probably the second greatest pain point was trying to mutate a collection in place while iterating over it. You can't in Rust. At least, not as far as I know.

While attempting to collect the tiles into a graph, I needed to update each tile with neighbor information as I discovered which tiles were its neighbors. At first, I tried to do this with a mutable `HashMap`, and I thought I'd hit upon the solution with the collection's `Entry` API. However, you can't iterate over the map's values and also modify them inside the iteration loop. The borrowing rules won't allow you to create a mutable reference to a value in the collection when the iterator has given you an immutable reference to it.

My solution to this was to clone the map of ungraphed tiles into a mutable variable, iterate over the original map and modify the values in the clone. I'm not sure cloning the entire collection is the best solution. I could have made a mutable clone of each entry and stored that in a fresh map instead. And I feel like might be a way to do this without cloning anything.

I said earlier that I would have been better off with a `Vec` of ungraphed tiles to get around the randomness of `HashMap` hashes. That would have saved me from myself. In that case, I would have started with an empty map, iterated over the list of tiles, creating a mutable copy of each tile, and iterated over the list of tiles to find that tile's neighbors.

## Something I Didn't Learn
There's one thing I encountered while working on this day that I still don't understand. Inside a function, I shadowed one of the function's parameters, a `&mut Vec<_>` I think it was, after a few lines of code. Something along these lines:

```rust
fn foo(param: &mut Vec<u64>) {
    // some code
    let mut param = some_other_value;
    // code that uses new `param`
}
```

I'm fuzzy on the details because it was least a week ago. Whatever it was, it caused the function to behave in a way I didn't understand while debugging it.

Since then I've learned about a few debugger nuances, such as it refusing to enter a single-line closure when you tell it to step into the closure. Perhaps I'm remembering the issue wrong and it was a debugger thing.

Regardless, I can't shake the feeling I missed learning about some subtlety of variable shadowing.
