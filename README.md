## Setup & Building
```bash
cargo install cargo-watch
cargo build
cargo test
```

You can also run commands for one service at a time:
```bash
cargo test -p app-service
cargo test -p auth-service
cargo run -p app-service
cargo run -p auth-service
```

## Run servers locally (Manually)
#### App service
```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service
```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

## Run servers locally (Docker)
```bash
docker compose build
docker compose up
```

visit http://localhost:8000 and http://localhost:3000

## Mermaid diagrams
Install the renderer once:
```bash
npm install -g @mermaid-js/mermaid-cli
```

Store diagrams as `.mmd` files in `docs/diagrams/`, then render all of them:
```bash
./scripts/render-mermaid.sh
```

Auto-render on change:
```bash
./scripts/watch-mermaid.sh
```

Preview an image in kitty:
```bash
kitty +kitten icat docs/diagrams/rendered/your-diagram.png
```
