# Advent of Code 2024 Solutions

## Running

```bash
cat ./inputs/day1p1.txt | cargo run --day 1 --part 1
# OR one of the below (all todo)
cargo run --day 1 --part 1 --input ./inputs/day1p1.txt
cargo run --day 1 --part 1 --directory ./inputs/
cargo run --day 1 --part 1
cargo test
cargo test --day 1 --part 1
```

### TOD
- Add CLI for detemining what test and whether part a/b is run
- Add unit tests for each day and part using example data and result provided
- Option to override piping behaviour if a file is provided
- Use CLI to download input data and automatically use it for test (requires OAuth with github creds)
