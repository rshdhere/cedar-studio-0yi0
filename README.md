# Cedar Chat

Real-time chat app built with **Rust**, **Tokio**, and **WebSockets** (Axum).

## Features

- Live chat room with join/leave notifications
- Message history for newly connected clients
- Health check endpoint for orchestration
- Responsive dark UI served at `/`

## Endpoints

| Route | Description |
|-------|-------------|
| `GET /` | Chat web UI |
| `GET /health` | JSON health check: `{ "ok": true }` |
| `GET /ws` | WebSocket chat channel |

## Getting started

```bash
cargo run
```

Open [http://localhost:8080](http://localhost:8080) in your browser.

Set a custom port with:

```bash
PORT=3000 cargo run
```

## WebSocket protocol

Clients send JSON messages:

```json
{ "type": "join", "user": "Alice" }
{ "type": "chat", "text": "Hello everyone!" }
```

The server broadcasts events such as `message`, `join`, `leave`, and `history`.
