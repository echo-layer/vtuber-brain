# Engineering Principles

These principles guide the development and maintenance of `vtuber-brain`.

## 🛠️ Core Architecture
- **Director / Performer split (brain decides what to think; voice decides how to say it):** Our primary architectural guideline to ensure code remains clean and understandable.
- **Tool use over hardcoded skills (skill = registered tool with typed contract from vtuber-contracts):** Secondary principle focusing on the specific performance and safety needs of the Rust, Axum, tonic, Mojo, Python (LLM bridge), Ollama, vLLM, Postgres, pgvector stack.

## ⚖️ Quality Standards
1. **Uncompromising Safety:** Every line of code must prioritize data integrity and memory safety.
2. **Predictable Performance:** Zero-cost abstractions are preferred over convenience if performance is impacted.
3. **Comprehensive Testing:** No feature is complete without an automated test suite runnable via `cargo test && cargo clippy --all-targets -- -D warnings && pytest python/`.

## 🤝 Collaborative Values
- **Explicit over Implicit:** Code should be self-documenting and intent should be clear.
- **Incremental Excellence:** We value small, high-quality PRs over massive, complex changes.
