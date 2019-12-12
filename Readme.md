#### Advent of code 2019 in Rust

Attempt to solve aoc2019 in rust.


##### Learned stuff
[Iterator.step_by](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by)

[Iterator.windows](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)

[successors](https://doc.rust-lang.org/std/iter/fn.successors.html)

[frickin' comparing floats to one another](https://github.com/rust-lang/rust/blob/88fc543866c2c48b3b1a32e9d55a4eb77d1dee66/src/test/run-pass/const-binops.rs#L12-L19)

[radian (I am stupid)](https://en.wikipedia.org/wiki/Radian)

[f64.to_radians](https://doc.rust-lang.org/std/primitive.f64.html#method.to_radians)

[atan2 to find an angle for a given point (day10)](https://stackoverflow.com/questions/21483999/using-atan2-to-find-angle-between-two-vectors/21484228)

[while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)

[match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)

[match guard](https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html)

[permutations of a sequence (language agnostic)](https://rosettacode.org/wiki/Permutations#Iterative)

[measure time (for benchmarking functions)](https://rust-lang-nursery.github.io/rust-cookbook/datetime/duration.html)

[split_at_mut, when borrowing mutliple elements in a vec things get weird (see day 12 "step" fn)](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut)


---
###### Ownership

```

let v = vec![1, 2, 3];

let mut v2 = v;
// accessing v is invalid, because v moved ownership to v2.
// this keeps us from i.e. changing v2 length to 2 and then accessing the v.3 element

// keywords: ownership, binding(pointer)

// # copy types
// uses the Copy trait, which copies the data on assign

let v = 1
let v2 = v
println!("v is: {}"); // this works, because i32 is copied

```

###### References and borrowing
https://doc.rust-lang.org/1.9.0/book/references-and-borrowing.html

```
// #'&' means to borrow a resource, but not being able to change it
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // do stuff with v1 and v2
    42
}
let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];
let answer = foo(&v1, &v2);
// this is possible!


// # Borrowing and mutating (&mut) 
let mut x = 5;
{ // <- borrowing needs a scope smaller than that of the owner!
    let y = &mut x;
    *y += 1; // variable y has to be accessed tith *y, because it is mutable reference
}
println!("{}", x); // <- prints 6 (5+1)


// adding an element to a mutable vector:
fn main() {
    let mut a = vec![1,2,3];
    add(&mut a);

    println!("{:?}",a)
}
fn add(v: &mut Vec<i32>) {
    v.push(99);
}

```

##### Lifetimes

```
// # 
// implicit
fn foo(x: &i32) {
}

// explicit
fn bar<'a>(x: &'a i32) { // reads: lifetime a
}
```