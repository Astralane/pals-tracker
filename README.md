# Paladin Validator Tracker 🛡️⚔️

A tiny server that helps you retrieve all palidators and their upcoming leader slots.

## Getting Started 🏁

use the sample .env file as a reference, you will require a good rpc for this.

```bash
# Clone the repository
git clone https://github.com/your-username/paladin-server.git
cd paladin-server

# Build with Rust nightly (requires Rust installed)
cargo build --release
```

## Endpoints

### 🛡️ `GET /api/palidators`
**Fetch all palidator public keys for current epoch**  

```json
[
    "Ss...Z77",
    "ACv...mi",
    "7Z...Z84",
]
```

### ⚔️ `GET /api/next_palidator`
**Get next leader palidator**
```json
{
  "pubkey": "Csd...def",
  "leader_slot": 42424242,
  "context_slot": 42424242
}
```

### ⚔️ `GET /api/next_palidator/{slot}`
**Get next leader palidator on or after given slot**
```json
{
  "pubkey": "Csd...def",
  "leader_slot": 42424242,
  "context_slot": 42424242
}
```

