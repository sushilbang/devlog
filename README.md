# devlog

A simple CLI tool to log your development progress. Keep track of what you're working on without leaving your terminal.

## Installation

```bash
cargo install devlogr
```

## Usage

**Add a log entry:**

```bash
devlog add "fixed the authentication bug"
devlog add "learned how async works in Rust"
devlog add "finally got tests passing"
```

**View all entries:**

```bash
devlog list
```

Output:

```
[2026-01-12 14:30:22] fixed the authentication bug
[2026-01-12 15:45:10] learned how async works in Rust
[2026-01-12 16:20:05] finally got tests passing
```

## How it works

Each time you run `devlog add`, your message is saved with a timestamp to a `.devlog.json` file in your current directory. This means each project gets its own log.

The JSON file looks like this:

```json
[
  {
    "timestamp": "2026-01-12 14:30:22",
    "message": "fixed the authentication bug"
  }
]
```

## Why?

When you want to post progress on Twitter or write a standup update, you often forget what you actually did. This tool keeps a running log so you never have to remember.

## License

MIT
