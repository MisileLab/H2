# $^{2}h$
![Rust](https://img.shields.io/badge/language-rust-1976d2?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-misilelab-green?style=for-the-badge)

## Download & Configuration 

1. [Download this project](https://github.com/MisileLab/H2/actions/workflows/test.yml)  
2. Register application in [site](https://dev.twitch.tv/console/apps)
3. Get client (id, secret) in application information.
4. Rename example.json to config.json and put client (id, secret), streamers.  

## What is problem for twitch notification?

### Mobile

You can't view on desktop (without low performance program lol)

### Web

Web does not give notification. also slow (load GUI)

## How this project resolved this problem?

Low Performance resolve -> Rust  
Ban posibility with web crawling -> Twitch API

## How to contributing(Issue)?

### Bug, crash

- Set name to `[bug] example`
- Requirements: OS Information (version and name)
- Crash requirements: run program with RUST_BACKTRACE=1, write log

### Feature request

- Set name to `[request] example`
- Requirements: Feature description, why this feature need.

## How to contributing(PR)?

Contributing = Thank you!

### translation

- Set name to `[feature] lang`
- Add translation string to translation.json
- Add lang name to struct TranslationHandler, impl TranslationTrait

### Fix, add feature

- Set name to `[feature, fix] (choose one) example`
