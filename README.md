# Hackernews Frontpage Crawler

This simple rust program will get the frontpage of hackernews, parse the link URLs out of the content, and iterate over those sending an HTTP GET to each. Then it'll wait 30 seconds before trying again. This is a useful traffic generator that changes over time.

## Using
Install rust

Build and run the program
```
cargo run
```