# Project Intelligence & Operational Logic

This file is the operational core. Gemini CLI MUST follow these protocols to maintain project integrity.

## 🎯 Architectural Intent
- **Core Mission:** The central reasoning, memory, and tool-use core that every vtuber persona depends on for character consistency across hours of live streaming, with a plug-and-play skill ecosystem.
- **Primary Stack:** Rust, Axum, tonic, Mojo, Python (LLM bridge), Ollama, vLLM, Postgres, pgvector
- **System Nature:** vtuber-brain is the Director half of the Director/Performer split (ADR-001 in repo_plus.yml). It receives context (chat, audience signal, internal state), runs reasoning and memory retrieval, decides which persona to wear and which skill to invoke, then emits a ConversationDirective containing text_prompt and voice_prompt to vtuber-voice via gRPC. Skills (game/sing/policy/strategy) register as typed tools and brain dispatches via tool-use protocol. Long-term memory lives in Postgres with pgvector; character lore loads from vtuber-commons at startup. Mojo handles hot inference kernels (RAG re-rank, intent classification); Python serves the underlying LLM (Ollama or vLLM).

## 🧬 Automated Lifecycle Management
1. **Research Sync:** When `./scripts/update_notebookLM.sh` is executed:
   - You MUST update `DESIGN_DECISIONS.md` with new ADRs found in research.
   - **Constraint:** Maintain a rolling log of the **latest 10 ADRs**.
2. **PR Creation Protocol:** When instructed to create a Pull Request:
   - **Summarize:** Analyze all commit messages since the last merge to `main`.
   - **Template:** Read `.github/PULL_REQUEST_TEMPLATE.md` and populate it with:
     - Detailed description of changes.
     - Linked Issue ID (search for keywords like "fixes #123").
     - Automated Labels (e.g., `feat`, `fix`, `docs`).
   - **Assign:** Automatically set the current developer as the Assignee.
3. **Pre-Commit Action:** Before every commit, you MUST:
   - Run `tree -a -I 'node_modules|.git|target' > STRUCTURE.tree`.
   - Trigger stack-specific formatting (e.g., `cargo fmt`).
   - Run `pre-commit run --all-files` if available.

## 🌐 Ecosystem Interaction Protocol
1. **Multi-Repo Boundaries:** You MUST NOT directly modify code in other `vtuber-*` repositories (especially `vtuber-contracts`).
2. **Issue-Based Communication:** When a change or resource is needed from another repository, you MUST:
   - Draft the requirements locally in `docs/specs/ecosystem/`.
   - Create a GitHub Issue in the target repository using `gh issue create`.
   - Reference the Issue URL in your local progress reports.
3. **Dependency Sync:** Only implement features depending on external changes (like new Schemas) after the corresponding Issue is resolved and released.

## 🛠️ Tooling & Standards
- **Translation:** All technical specifications are English. `locales/` MUST be kept in sync and translated for users documentation.
- **Workflow Mastery:** Use `/superpower:executing-plans` for feature work.
- **Automation:** Refer to `.github/workflows/pr_automation.yml` for server-side PR handling.

## 📂 Template Inventory
You manage: ARCHITECTURE.md, ROADMAP.md, CONTRIBUTING.md, DESIGN_DECISIONS.md, STRUCTURE.tree, SECURITY.md, LICENSE.md, FAQ.md, GOVERNANCE.md, SUPPORT.md, TROUBLESHOOTING.md, PHILOSOPHY.md, MANIFESTO.md, and `locales/README.{th,ja,zh}.md`.
