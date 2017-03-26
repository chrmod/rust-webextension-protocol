#!/usr/bin/env bash
cargo build --example echo
cat tests/fixtures/simple.json | target/debug/examples/echo
