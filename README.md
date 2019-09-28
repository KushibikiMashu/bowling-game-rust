# bowling-game-rust
coding kata

https://www.slideshare.net/lalitkale/bowling-game-kata-by-robert-c-martin

# Build
```bash
$ docker build . -t rust:bowling-game
$ docker run -ti -v `pwd`:bowling-game rust:bowling-game ash
```

# Run test
```bash
/home/bowling-game # cargo test

running 5 tests
test tests::all_gutter ... ok
test tests::all_one_pin ... ok
test tests::all_strike ... ok
test tests::one_spare ... ok
test tests::one_strike ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc-tests bowling-game
```