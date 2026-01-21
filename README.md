# Solux ⚡

**Solux** is a high-performance, in-memory Credit Matching Engine written in **Rust**.

It is designed to facilitate peer-to-peer lending markets by matching **Lenders** (investors) and **Borrowers** based on **Interest Rates** and **Time Priority** (Price-Time Priority algorithm). It supports advanced features like partial order filling, event sourcing, and state persistence.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![Build](https://img.shields.io/badge/build-passing-green.svg)

## 🚀 Features

- **Price-Time Priority Matching**:
  - **Borrowers (Bids)** are prioritized by *Highest Interest Rate* (willing to pay more).
  - **Lenders (Asks)** are prioritized by *Lowest Interest Rate* (most competitive offer).
  - Orders with the same rate are matched based on **FIFO** (First-In-First-Out).
- **Partial Fills**: A large order can be matched against multiple smaller orders until the full amount is satisfied.
- **Event Sourcing**: The engine emits events (`OrderPlaced`, `OrderMatched`, `OrderFilled`) instead of just mutating state, enabling reactive architectures.
- **Persistence**: Built-in support for saving/loading Orderbook snapshots via JSON.
- **Type-Safe**: Built with Rust to ensure memory safety and concurrency support.

---

## 📂 Project Structure

```text
solux/
├── src/
│   ├── engine.rs       # Core logic orchestrating the matching process
│   ├── orderbook.rs    # Data structures managing Bids and Asks queues
│   ├── types.rs        # Data definitions (Order, Trade, Side)
│   ├── events.rs       # Event definitions (OrderMatched, etc.)
│   ├── persistence.rs  # Snapshot Save/Load functionality
│   ├── error.rs        # Custom error handling
│   └── lib.rs          # Module exports
├── tests/              # Integration tests
└── benches/            # Performance benchmarks (Criterion)

🛠️ Installation
Ensure you have Rust and Cargo installed.

Clone the repository:

Bash

git clone [https://github.com/your-username/solux.git](https://github.com/your-username/solux.git)
cd solux
Build the project:

Bash

cargo build --release
💡 Usage Example
Here is a complete example of how to use the MatchingEngine to match a Lender with a Borrower. You can run this inside your main.rs if you build a binary, or use it as a library reference.

Rust

use solux::engine::MatchingEngine;
use solux::types::{Order, OrderSide};

fn main() {
    // 1. Initialize the Engine
    let mut engine = MatchingEngine::new();
    println!("🚀 Engine Started...");

    // 2. A Lender offers 10,000 at 5.0% (500 bps)
    // The engine stores this in the Orderbook waiting for a match.
    let lender_order = Order::new(OrderSide::Lend, 10_000, 500);
    engine.place_order(lender_order);
    println!("✅ Lender Order Placed: 10,000 @ 5.0%");

    // 3. A Borrower needs 4,000 and is willing to pay 6.0% (600 bps)
    // Since 6.0% >= 5.0%, a match will occur immediately for 4,000.
    // The Lender will still have 6,000 remaining in the order book.
    let borrower_order = Order::new(OrderSide::Borrow, 4_000, 600);
    engine.place_order(borrower_order);
    println!("✅ Borrower Order Placed: 4,000 @ 6.0%");

    // 4. Process Events
    // The engine does not return trades directly but emits events.
    let events = engine.drain_events();
    
    for event in events {
        match event {
            solux::events::EngineEvent::OrderMatched(trade) => {
                println!(
                    "🎉 MATCH FOUND! Trade ID: {}\n   Amount: {}\n   Rate: {} bps", 
                    trade.id, trade.amount, trade.interest_rate
                );
            },
            _ => println!("ℹ️ Event: {:?}", event),
        }
    }
}
💾 Persistence
Solux allows you to save the state of the Orderbook to a JSON file and reload it later. This is useful for restarting the system without losing active orders.

Rust

use solux::persistence::Persistence;
use solux::engine::MatchingEngine;

fn main() {
    let mut engine = MatchingEngine::new();
    let persistence = Persistence::new("snapshot.json");

    // ... perform operations ...

    // Save State
    if let Err(e) = persistence.save(&engine.orderbook) {
        eprintln!("Failed to save snapshot: {}", e);
    } else {
        println!("💾 State saved successfully.");
    }

    // Load State
    match persistence.load() {
        Ok(loaded_book) => {
            engine.orderbook = loaded_book;
            println!("📂 State loaded successfully.");
        },
        Err(e) => eprintln!("Failed to load snapshot: {}", e),
    }
}
🧪 Testing
Solux includes Integration Tests to verify the matching logic, including scenarios for partial fills and complex order queues.

To run the tests:

Bash

cargo test
Expected output:

Plaintext

running 1 test
test test_partial_fill_scenario ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
⚡ Benchmarking
We use Criterion.rs to benchmark the matching engine's performance (latency and throughput).

To run the benchmarks:

Bash

cargo bench
This will execute the scenarios defined in benches/matching_bench.rs and generate a performance report located at target/criterion/report/index.html.

📄 License
This project is licensed under the MIT License.

Plaintext

MIT License

Copyright (c) 2024 Solux Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
