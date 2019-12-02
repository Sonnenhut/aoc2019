#### Advent of code 2019 in Rust

Attempt to solve aoc2019 in rust.


##### Learned stuff
[Iterator.step_by](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by)
[successors](https://doc.rust-lang.org/std/iter/fn.successors.html)

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