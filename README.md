## stack-rs

stack-rs is a free-lock library implemented using rust.


### quick start

```rust
let s = Stack::new();
s.push(1);
assert_eq!(s.pop(), Some(1));
assert_eq!(s.pop(), None);
```


### benchmark

Lock-Free Stack VS std::sync::Mutex\<std::collections::LinkedList\>

main.rs benchmark output:

|   Benchmark   | Total time spent | Average time spent |
| ------------- | ----- | ------- |
| stack_loop_n(100000) | 56.523467ms | 565ns |
| list_loop_n(100000)  | 67.573497ms | 675ns |
| stack_thread_n_m(2, 100000) | 115.590207ms | 577ns |
| list_thread_n_m(2, 100000)  | 161.359683ms | 806ns |
| stack_thread_n_m(4, 100000) | 440.585874ms | 1.101µs |
| list_thread_n_m(4, 100000)  | 562.439723ms | 1.406µs |
| stack_thread_n_m(8, 100000) | 1.886768172s | 2.358µs |
| list_thread_n_m(8, 100000)  | 2.120945074s | 2.651µs |


### reference

[Implementing Lock-Free Queues (1994)](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.53.8674)

