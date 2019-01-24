# Benchmark between different methods to know a `f64` value is also an integer

Use the following command to bench:

```bash
% rustup override set nightly
% cargo bench
```

```command line
% cargo bench
running 2 tests
test tests::bench_cast  ... bench:     175,998 ns/iter (+/- 79,679)
test tests::bench_fract ... bench:     157,540 ns/iter (+/- 26,664)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out

% cargo bench
running 2 tests
test tests::bench_cast  ... bench:     173,530 ns/iter (+/- 44,183)
test tests::bench_fract ... bench:     172,032 ns/iter (+/- 47,314)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out

% cargo bench
running 2 tests
test tests::bench_cast  ... bench:     172,118 ns/iter (+/- 63,072)
test tests::bench_fract ... bench:     181,475 ns/iter (+/- 55,781)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out

```
