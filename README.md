## competition rust

<p>
    <a href="https://online.visualstudio.com/environments/new?name=Learn%20Rust&repo=HikaruEgashira/competition-rust">
        <img src="https://img.shields.io/endpoint?style=social&url=https%3A%2F%2Faka.ms%2Fvso-badge">
    </a>
</p>

### AtCoder のコンテストに Rust で参加するためのガイドブック

https://doc.rust-jp.rs/atcoder-rust-resources/

```
cargo generate --git https://github.com/rust-lang-ja/atcoder-rust-base --branch ja
```

### tanakh/cargo-atcoder

https://github.com/tanakh/cargo-atcoder

```
cargo atcoder new <contest-name>

cargo atcoder test <problem-id>

cargo atcoder submit <problem-id> # a, b, c...
```

```
SUBCOMMANDS:
    clear-session    Clear session data (cookie store in HTTP client)
    gen-binary       Generate rustified binary
    help             Prints this message or the help of the given subcommand(s)
    info             Show session information
    login            Login to atcoder
    new              Create a new project for specified contest
    result           Show submission result detail
    status           Show submission status
    submit           Submit solution
    test             Test sample cases
    warmup           Warmup (pre-compile dependencies)
    watch            [WIP] Watch filesystem for automatic submission
```
