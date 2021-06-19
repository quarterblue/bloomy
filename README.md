<br />
<p align="center">
  <a href="https://raw.githubusercontent.com/quarterblue/bloomy/main/public/bloomy.png">
    <img width="50%" src="https://raw.githubusercontent.com/quarterblue/bloomy/main/public/bloomy.png" alt="bloomy logo">
  </a>
  <p align="center">
    <a href="https://github.com/quarterblue/bloomy/actions/workflows/rust.yml" target="_blank">
        <img src="https://github.com/quarterblue/bloomy/actions/workflows/rust.yml/badge.svg" alt="GitHub Passing">
    </a>
    <a href="https://github.com/quarterblue/bloomy/blob/main/LICENSE" target="_blank">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
    </a>   
    <a href="https://github.com/quarterblue/bloomy/graphs/commit-activity" target="_blank">
        <img src="https://img.shields.io/github/last-commit/quarterblue/bloomy" alt="Last Commit">
    </a>
</p>
  <h3 align="center">Bloomberg terminal alternative.</h3>

  <p align="center">
    Minimalistic cli app for fetching and analyzing equity data.
    <br />
    <a href="https://github.com/quarterblue/bloomy"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/quarterblue/bloomy">View Demo</a>
    ·
    <a href="https://github.com/quarterblue/bloomy/issues">Report Bug</a>
    ·
    <a href="https://github.com/quarterblue/bloomy/issuess">Request Feature</a>
  </p>
</p>

## About

`Bloomy` is a minimalistic terminal application for fetching, analyzing, and backesting equity data.

## Key Features

- [x] Multiple portfolio management
- [x] Real time price fetching and rendering
- [x] Equity overview analysis
- [ ] Historical price charting
- [x] Discounted cash flow analysis
- [x] Comparables analysis
- [ ] Backtesting
- [ ] Customizable configuration
- [x] Alpha Vantage integration
- [ ] IEX Cloud integration


## Installation

To run, clone the repo and run
```
cargo run --release [bloomy]
```

## Usage
**Equity Commands:**

equity price `[TICKER]`

_Fetch and render price of `[TICKER]`_
```bash
$ bloomy cmd> equity price tsla
```

```bash
ticker: TSLA
open: 613.37
high: 628.35
low: 611.80
price: 623.31
volume: 24,560,905
ltd: 2021-06-18
```

**Portfolio Commands:**

port make `[PORTFOLIO]`
```bash
$ bloomy cmd> port make tech2021
```
