# Advent of Code 2024 Solutions

## Running

```bash
cat ./inputs/day1p1.txt | cargo run -- -d1 -p1
# OR one of the below (all todo)
cargo run -- -d1 -p1 --input ./inputs/day1p1.txt
cargo run -- -d1 -p1
cargo run -- -d1 -p1
cargo test
cargo test -- -d1 -p1
```

### TODO
- Add CLI for detemining what test and whether part a/b is run
- Add unit tests for each day and part using example data and result provided
- Option to override piping behaviour if a file is provided
- Use CLI to download input data and automatically use it for test (requires OAuth with github creds)
