    cargo new fibonacci --bin --vcs none

`--bin` to generate the `main.rs` and `--vcs none` to not conflict with the current git repository.

---
# Fibonacci using iterators

$$\begin{aligned}
F_0 &= 0 \\
F_1 &= 1 \\
F_n &= F_{n-2} + F_{n-1}
\end{aligned}$$

$$
F = \{0,1,1,2,3,5,8,13,\dots\}
$$

Instead of implement it as a recursive functions we can rewrite the rule as

$$\begin{pmatrix} F_n \\
F_{n-1} \end{pmatrix} = \begin{pmatrix} 1 & 1 \\
1 & 0 \end{pmatrix} \begin{pmatrix} F_{n-1} \\
F_{n-2} \end{pmatrix}$$

In that case we store the state of the sequece in a tuple with the last two elements.

So first we create a structu to store the state of the Fibonacci sequence.

```rust
struct Fib {
    state: (usize, usize)
}
```

Next we implement our struct as an interator type. The `Iterator` trait requires a `Item` type which is the type of the elements in the collection (`usize` in this case) and a function `next` that returns... the next element of the collection? Not exactly! I returns an optional with `Some` element of the collection or `None` in case of finite collections.

Since, in theory, the Fibonacci sequence is infity, we don't need to inform a stop criteria. However would be better to use one since here we are assuming fixed size integer which in practice has a termination number. But let's ignore that for now.

```rust
impl Iterator for Fib {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    self.x = (self.x.1, self.x.0 + self.x.1);
    Some(self.x.0)
  }
}
```

So `fn next(&mut self)` is a mutating function which is let clear from the type notation. We know the object will change after we call this method.

Now let's create a method that generate a list with the first $n$ numbers of the Fibonacci sequence.

```rust
impl Fib {
  fn take_first(n: usize) -> Vec<usize> {
    let fib = Fib {x: (1,0)};
    fib.take(n).collect()
  }
}
```

The `Iterator` trait provide a `take` method that calls the `next` method a fixed number of times. And here is a trick.

Since the `next` will be called right for the first element, the initial values of the sequence must be `(1,0)` instead of `(0,1)`. Otherwise, the `next` method will skip the `0` value.

Last we call the `collect` method to wrap the result into a vector.

Despite the fact the `take` calls `next` and `next` causes a mutation the in the state to the variable `fib`, we don't need to declare it as `let mut fib` because all the mutations occur in "a controlled situation." We are not passing the variable to a function or changing by ourselves.

Finally, the `main` function is quite trivial.

```rust
fn main () {
    println!("{:?}", Fib::take_first(10));
}
```

    % cargo run   
    Compiling fibonacci v0.1.0 (.../fibonacci)
     Finished dev [unoptimized + debuginfo] target(s) in 0.14s
      Running `target/debug/fibonacci`
    [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
