# Design Spec: Phase 1 Foundation & Hybrid gRPC Core Loop

- **Status:** Draft
- **Date:** 2026-04-30
- **Topic:** Foundation Setup for vtuber-brain

## 1. Architectural Overview
vtuber-brain acts as the **Director** (Reasoning Engine) in the vtuber-* ecosystem. It implements a Hybrid gRPC model to handle both inbound context and outbound directives.

### Component Responsibilities:
- **Server Role:** Listens for `PushContext` requests from `vtuber-api`.
- **Client Role:** Emits `ConversationDirective` to `vtuber-voice` and calls tool-based skills (game, sing, etc.).
- **Core Loop:** Orchestrates the flow: Receive Context -> Retrieve Memory -> LLM Reasoning -> Emit Directive.

## 2. Technical Stack
- **Language:** Rust (Latest Stable)
- **Web/gRPC Framework:** Axum + Tonic
- **Serialization:** Protobuf (via `prost`)
- **Database:** Postgres + pgvector
- **Inference Bridge:** Python (Ollama/vLLM) + Mojo (Hot kernels)

## 3. Implementation Plan (Phase 1)

### 3.1 Rust Workspace Structure
Initialize a Cargo workspace to maintain clear boundaries:
- `crates/brain-core`: Main reasoning logic and state management.
- `crates/brain-grpc`: gRPC server and client implementations.
- `crates/brain-proto`: Compiled protobuf definitions.
- `crates/brain-shared`: Shared utilities and types.

### 3.2 gRPC Foundation
- Define `brain.proto` for the primary interface.
- Implement a mock `vtuber-voice` client for local testing.
- Implement the `PushContext` service for `vtuber-api` integration.

### 3.3 Ecosystem Interaction (Protocol Compliance)
- **contracts:** We will NOT modify `vtuber-contracts` directly.
- **external:** Any required changes in `voice` or `api` will be requested via GitHub Issues as per the **Ecosystem Interaction Protocol**.
- **specs:** Requirements for other repos will be drafted in `docs/specs/ecosystem/`.

## 4. Data Flow (Phase 1)
1. `vtuber-api` sends `Context` (JSON/Protobuf) -> `vtuber-brain` (Server).
2. `vtuber-brain` logs the request and runs a "Pass-through" reasoning (Mock).
3. `vtuber-brain` (Client) sends `ConversationDirective` -> `vtuber-voice` (Mock/Actual).

## 5. Next Steps
- [ ] Initialize Cargo workspace.
- [ ] Draft Protobuf definitions.
- [ ] Implement the basic Director loop with mock components.
- [ ] Verify CI/CD pipeline compatibility.
