# Foundation & Core Loop Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Initialize the Rust workspace and set up the Hybrid gRPC foundation for vtuber-brain.

**Architecture:** Cargo workspace with crates for core logic, gRPC, and shared types. Implements a Director loop that acts as both gRPC server and client.

**Tech Stack:** Rust, Tonic, Prost, Axum.

---

## File Structure

- `Cargo.toml`: Root workspace configuration.
- `crates/brain-proto/`: Protobuf definitions and generated code.
- `crates/brain-core/`: Core reasoning and state logic.
- `crates/brain-grpc/`: gRPC server and client implementations.

---

### Task 1: Initialize Cargo Workspace

**Files:**
- Create: `Cargo.toml` (root)

- [x] **Step 1: Create root Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "crates/brain-core",
    "crates/brain-proto",
    "crates/brain-grpc",
]

[workspace.dependencies]
tonic = "0.12"
prost = "0.13"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
```

- [x] **Step 2: Commit**

```bash
git add Cargo.toml
git commit -m "chore: initialize cargo workspace"
```

### Task 2: Define Protobuf contracts

**Files:**
- Create: `crates/brain-proto/proto/brain.proto`

- [x] **Step 1: Define the Brain service contract**

```proto
syntax = "proto3";

package brain;

// vtuber-api -> vtuber-brain
service BrainService {
  rpc PushContext (ContextRequest) returns (ContextResponse);
}

// vtuber-brain -> vtuber-voice
service VoiceService {
  rpc EmitDirective (DirectiveRequest) returns (DirectiveResponse);
}

message ContextRequest {
  string session_id = 1;
  string user_id = 2;
  string message = 3;
  map<string, string> metadata = 4;
}

message ContextResponse {
  bool accepted = 1;
  string request_id = 2;
}

message DirectiveRequest {
  string text_prompt = 1;
  string voice_prompt = 2;
  map<string, string> commands = 3;
}

message DirectiveResponse {
  bool success = 1;
}
```

- [x] **Step 2: Commit**

```bash
git add crates/brain-proto/proto/brain.proto
git commit -m "feat: define brain and voice gRPC contracts"
```

### Task 3: Set up brain-proto crate

**Files:**
- Create: `crates/brain-proto/Cargo.toml`
- Create: `crates/brain-proto/src/lib.rs`
- Create: `crates/brain-proto/build.rs`

- [ ] **Step 1: Create brain-proto Cargo.toml**

```toml
[package]
name = "brain-proto"
version = "0.1.0"
edition = "2021"

[dependencies]
tonic = { workspace = true }
prost = { workspace = true }

[build-dependencies]
tonic-build = "0.12"
```

- [ ] **Step 2: Create build.rs to compile proto**

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/brain.proto"], &["proto"])?;
    Ok(())
}
```

- [ ] **Step 3: Export generated code in lib.rs**

```rust
pub mod brain {
    tonic::include_proto!("brain");
}
```

- [ ] **Step 4: Verify build**

Run: `cargo build -p brain-proto`
Expected: Success

- [ ] **Step 5: Commit**

```bash
git add crates/brain-proto/
git commit -m "feat: setup brain-proto crate with build.rs"
```

### Task 4: Implement gRPC Server (brain-grpc)

**Files:**
- Create: `crates/brain-grpc/Cargo.toml`
- Create: `crates/brain-grpc/src/lib.rs`
- Create: `crates/brain-grpc/src/server.rs`

- [x] **Step 1: Setup brain-grpc Cargo.toml**

```toml
[package]
name = "brain-grpc"
version = "0.1.0"
edition = "2021"

[dependencies]
brain-proto = { path = "../brain-proto" }
tonic = { workspace = true }
tokio = { workspace = true }
tracing = { workspace = true }
```

- [x] **Step 2: Implement BrainService server**

```rust
use brain_proto::brain::brain_service_server::BrainService;
use brain_proto::brain::{ContextRequest, ContextResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MyBrainService {}

#[tonic::async_trait]
impl BrainService for MyBrainService {
    async fn push_context(
        &self,
        request: Request<ContextRequest>,
    ) -> Result<Result<Response<ContextResponse>, Status>> {
        let r = request.into_inner();
        println!("Received context from {}: {}", r.user_id, r.message);
        
        Ok(Response::new(ContextResponse {
            accepted: true,
            request_id: "mock-uuid".to_string(),
        }))
    }
}
```

- [x] **Step 3: Commit**

```bash
git add crates/brain-grpc/
git commit -m "feat: implement basic gRPC server for PushContext"
```

### Task 5: Core Loop Integration Mock

- [x] **Step 1: Create brain-grpc server binary**

Create: `crates/brain-grpc/src/bin/server.rs`

```rust
use brain_grpc::server::MyBrainService;
use brain_proto::brain::brain_service_server::BrainServiceServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let brain_service = MyBrainService::default();

    println!("Brain server listening on {}", addr);

    Server::builder()
        .add_service(BrainServiceServer::new(brain_service))
        .serve(addr)
        .await?;

    Ok(())
}
```

- [x] **Step 2: Verify binary build**

Run: `cargo build --bin server`
Expected: Success

- [x] **Step 3: Commit**

```bash
git add crates/brain-grpc/src/bin/server.rs
git commit -m "feat: add gRPC server binary"
```
