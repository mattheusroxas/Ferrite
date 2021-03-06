## About
A CLI program for YouTube written in Rust.

### Why though
I was inspired by youtube-viewer, and it's many forks, to adapt the idea of a YouTube CLI. Since I use `pipe-viewer` so regularly I thought it would be nice to make my own spin on it and even improve on it in some areas.

## Supported platforms
This program will literally run on anything with a terminal.

## Dependencies
* cargo
* mpv
* youtube-dl

## To all you Windonk users out there
Install WSL using the following link. After you've done that just follow all the commands normally.
https://docs.microsoft.com/en-us/windows/wsl/install-win10

## How to get cargo
First install `rustup`
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then actually get `cargo` (If you're a cool kid you'll swap out `stable` with `nightly`)
```
rustup install stable
```

## Installation
```
cargo install
```
