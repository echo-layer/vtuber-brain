# Troubleshooting Guide

## 🔍 Common Issues

### Issue: Installation Fails
- **Check:** Ensure your `Rust, Axum, tonic, Mojo, Python (LLM bridge), Ollama, vLLM, Postgres, pgvector` version matches the requirements.
- **Fix:** Run `cargo build && pip install -r python/requirements.txt` with administrative privileges if necessary.

### Issue: Tests are failing
- **Check:** Verify your environment variables.
- **Run:** `cargo test && cargo clippy --all-targets -- -D warnings && pytest python/` with verbose logging enabled.

## 🛠️ Debugging Tools
Use the built-in logging and diagnostic flags to trace the execution flow.
