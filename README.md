# 🦀 Rust GitHub Actions Workflow 🚀

[![Rust](https://img.shields.io/badge/Language-Rust-blue?style=flat-square)](https://www.rust-lang.org)
[![Build Status](https://github.com/tailcallhq/rust-gh-workflows/actions/workflows/ci.yml/badge.svg?style=flat-square)](https://github.com/tailcallhq/rust-gh-workflows/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?style=flat-square)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/rust-gh-workflows?style=flat-square)](https://crates.io/crates/rust-gh-workflows)
[![Contributors](https://img.shields.io/github/contributors/tailcallhq/rust-gh-workflows?style=flat-square)](https://github.com/tailcallhq/rust-gh-workflows/graphs/contributors)
[![GitHub forks](https://img.shields.io/github/forks/tailcallhq/rust-gh-workflows?style=flat-square)](https://github.com/tailcallhq/rust-gh-workflows/network/members)
[![Stars](https://img.shields.io/github/stars/tailcallhq/rust-gh-workflows?style=flat-square)](https://github.com/tailcallhq/rust-gh-workflows/stargazers)
[![Issues](https://img.shields.io/github/issues/tailcallhq/rust-gh-workflows?style=flat-square)](https://github.com/tailcallhq/rust-gh-workflows/issues)

## 🧑‍💻 What is Rust GitHub Workflows?

**Rust GitHub Workflows** is a library that allows developers to write GitHub Actions in Rust. It empowers you to automate, manage, and improve your CI/CD pipelines in a type-safe manner.

GitHub Actions is powerful, but writing workflows can sometimes feel repetitive, tricky and frustrating with no strict type-checking. That's where **Rust GitHub Workflows** steps in! 🦾

- 🔥 **Rust-Powered**: Leverage the type-safety of Rust for writing workflows.
- 📦 **Crate-friendly**: Build reusable workflows and publish them as crates.

## 📦 Installation

To use **Rust GitHub Workflows** in your project, add it to your `Cargo.toml`:

```toml
[build-dependencies]
gh-workflow = "0.1"
```

or via CLI

```bash
cargo add --build gh-workflow
```

Then you can start creating GitHub Actions in your [build.rs](https://github.com/tailcallhq/rust-gh-workflows/blob/main/workspace/gh-workflow-gen/build.rs).

## 💡 Why Rust?

Rust provides the perfect combination of speed, safety, and flexibility, making it an ideal choice for writing GitHub Actions. With Rust, you get strong typing, memory safety, and the ability to reuse existing code, which can make your automation scripts more robust and maintainable.

## 📄 License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.

## 🙌 Contributors

A big thank you to all the contributors who helped make this project a success! 🙏

[![Contributors](https://contrib.rocks/image?repo=tailcallhq/rust-gh-workflows)](https://github.com/tailcallhq/rust-gh-workflows/graphs/contributors)

## 🌟 Show Your Support

If you like this project, please consider giving it a ⭐ on [GitHub](https://github.com/tailcallhq/rust-gh-workflows) and share it with the community!

## 🔗 Inspiration

This project was inspired by the following repositories:

- [sbt/sbt-github-actions](https://github.com/sbt/sbt-github-actions)
- [emmanuelnk/github-actions-workflow-ts](https://github.com/emmanuelnk/github-actions-workflow-ts)

---

Happy automating with Rust! 🦀❤️
