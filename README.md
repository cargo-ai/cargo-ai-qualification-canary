# Cargo AI Qualification Canary

This repository is the public, credential-free compatibility fixture for
[Cargo AI](https://github.com/cargo-ai/cargo-ai). It is intentionally small and
is not an official product package.

The canary proves that a candidate Cargo AI CLI can:

- assemble this source package;
- install and inspect it in an isolated Cargo AI Home;
- materialize and validate its Rust tool;
- run the exported agent and its installed tool through a deterministic loopback provider;
- compile-check the installed hatch path without re-auditing the interpreted runtime tool;
- execute the package-owned bounded checks on Windows, macOS, and Ubuntu.

The package requests no network, filesystem, environment, subprocess,
credential, provider-account, or Cargo AI account access. The qualification
runner owns temporary state and supplies the loopback provider.

## Local checks

Use the Cargo AI candidate being qualified rather than an unrelated globally
installed binary:

```text
cargo test --locked --manifest-path tools/qualification_probe/Cargo.toml
cargo ai tools lint qualification_probe
cargo ai tools build qualification_probe
cargo ai tools check qualification_probe
cargo ai hatch qualification_smoke --config qualification_smoke.json --check
```

The shared package runner uses `hatch --check --ignore-tools` for the installed
entrypoint because hatch audits project source tools, while the installed run
proves the package's version-bound runtime tool. Those are separate checks.

The reusable qualification workflow is pinned to an immutable Cargo AI commit.
Its workflow revision, tested Cargo AI revision, and this package revision are
reported independently.
