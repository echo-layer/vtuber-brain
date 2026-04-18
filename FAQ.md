# Frequently Asked Questions (FAQ)

## ❓ General
**Q: What is vtuber-brain?**
A: Director for the vtuber-* program — long-term memory, skill router, tool use, and per-turn persona prompt generation. Receives context, chooses persona, emits text_prompt and voice_prompt to vtuber-voice (PersonaPlex Performer) via ConversationDirective.

## 🛠️ Technical
**Q: How do I run tests?**
A: Use the command: `cargo test && cargo clippy --all-targets -- -D warnings && pytest python/`.

**Q: Which languages are supported?**
A: Primarily Rust, Axum, tonic, Mojo, Python (LLM bridge), Ollama, vLLM, Postgres, pgvector.

## 🤝 Contributing
**Q: How can I help?**
A: Check out [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to get involved!
