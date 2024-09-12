# `libc` checks
[![Continuous Integration](https://github.com/SergioGasquez/libc-checks/actions/workflows/rust_ci.yml/badge.svg)](https://github.com/SergioGasquez/libc-checks/actions/workflows/rust_ci.yml)

This repository contains code for an app that compares `libc` constants and structures. ESP-IDF source is taken as source of truth, and its accessed via `esp_idf_svc::sys`, and then its compared with the [`newlib` values](https://github.com/rust-lang/libc/tree/main/src/unix/newlib)

