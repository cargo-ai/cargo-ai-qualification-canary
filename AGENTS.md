# Agent Working Rules

- Keep this repository a minimal public, credential-free qualification fixture.
- Do not add provider keys, account tokens, private endpoints, local absolute paths, or generated Cargo AI Home state.
- Keep `.cargo-ai/project.toml` as the package contract and `cargo-ai-qualification.toml` as the bounded CI declaration.
- Use the Cargo AI-generated Rust tool layout and keep protocol/bridge files separate from author-owned behavior in `src/tool.rs`.
- Track source and lockfiles only. Do not commit `target/`, built tool artifacts, package output, generated guidance, or runtime data.
- Pin reusable workflows and actions to reviewed immutable commit SHAs.
