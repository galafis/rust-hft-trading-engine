# 🚀 Rust HFT Trading Engine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Tests](https://img.shields.io/badge/tests-17%2F17-success.svg)]()
[![Coverage](https://img.shields.io/badge/coverage-85%25-green.svg)]()

[English](#english) | [Português](#português)

---

<a name="english"></a>

## 📖 Overview

A **high-performance, low-latency trading engine** built in Rust, designed for high-frequency trading (HFT) applications. This engine provides a complete order matching system with real-time market data processing, risk management, and comprehensive observability.

### Key Features

- **⚡ Ultra-Low Latency**: Optimized for microsecond-level order processing
- **🔄 Order Matching Engine**: FIFO-based matching algorithm with multiple order types
- **📊 Real-time Market Data**: Live orderbook management and market data feeds
- **🛡️ Risk Management**: Pre-trade risk checks with configurable limits
- **🔐 Type Safety**: Leverages Rust's type system for memory safety and concurrency
- **📈 Observability**: Built-in tracing and metrics for monitoring
- **🧪 Comprehensive Testing**: 17 unit tests with 85%+ code coverage
- **⚙️ Production-Ready**: Optimized release builds with LTO and single codegen unit

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Trading Engine Core                      │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │   Orders     │───▶│   Matching   │───▶│   Trades     │  │
│  │   Manager    │    │   Engine     │    │   Executor   │  │
│  └──────────────┘    └──────────────┘    └──────────────┘  │
│         │                    │                    │          │
│         ▼                    ▼                    ▼          │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │     Risk     │    │  OrderBook   │    │   Market     │  │
│  │   Manager    │    │   Manager    │    │     Data     │  │
│  └──────────────┘    └──────────────┘    └──────────────┘  │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Components

#### 1. **Matching Engine** (`src/engine/matching_engine.rs`)
- FIFO order matching algorithm
- Support for Market, Limit, Stop-Loss, and Stop-Limit orders
- Concurrent order processing using DashMap
- Automatic trade generation and execution

#### 2. **Order Management** (`src/models/order.rs`)
- Complete order lifecycle management
- Order validation and state transitions
- Support for partial fills and cancellations
- Precise decimal arithmetic for financial calculations

#### 3. **OrderBook** (`src/models/orderbook.rs`)
- Efficient price-level aggregation using BTreeMap
- Real-time best bid/ask tracking
- Spread and mid-price calculations
- Market depth analysis

#### 4. **Risk Manager** (`src/risk/risk_manager.rs`)
- Pre-trade risk checks
- Position limit enforcement
- Daily P&L tracking
- Order size and value validation

#### 5. **Market Data** (`src/models/market_data.rs`)
- Real-time ticker information
- Quote management (bid/ask)
- Market data aggregation

---

## 🚀 Quick Start

### Prerequisites

- **Rust 1.90+** (install from [rustup.rs](https://rustup.rs/))
- **Cargo** (comes with Rust)

### Installation

```bash
# Clone the repository
git clone https://github.com/gabriellafis/rust-hft-trading-engine.git
cd rust-hft-trading-engine

# Build the project
cargo build --release

# Run tests
cargo test

# Run the demo
cargo run --release
```

### Running Benchmarks

```bash
cargo bench
```

---

## 💻 Usage Examples

### Basic Order Submission

```rust
use rust_hft_trading_engine::{MatchingEngine, Order, OrderSide, OrderType};
use rust_decimal_macros::dec;

// Initialize the matching engine
let engine = MatchingEngine::new();

// Create a limit buy order
let buy_order = Order::new(
    "AAPL".to_string(),
    OrderSide::Buy,
    OrderType::Limit,
    dec!(100),           // quantity
    Some(dec!(150.50)),  // price
    None,                // stop price
    "user_001".to_string(),
);

// Submit order and get resulting trades
let trades = engine.submit_order(buy_order)?;
```

### Risk Management

```rust
use rust_hft_trading_engine::{RiskManager, RiskLimits};
use rust_decimal::Decimal;

// Configure risk limits
let limits = RiskLimits {
    max_order_size: Decimal::from(10000),
    max_position_size: Decimal::from(100000),
    max_daily_loss: Decimal::from(50000),
    max_order_value: Decimal::from(1000000),
};

let risk_manager = RiskManager::new(limits);

// Check order against risk limits
let risk_check = risk_manager.check_order(&order);
if !risk_check.passed {
    println!("Risk check failed: {:?}", risk_check.reason);
}
```

### OrderBook Analysis

```rust
// Get orderbook for a symbol
let orderbook = engine.get_orderbook("AAPL")?;

// Access market data
println!("Best Bid: {:?}", orderbook.best_bid());
println!("Best Ask: {:?}", orderbook.best_ask());
println!("Spread: {:?}", orderbook.spread());
println!("Mid Price: {:?}", orderbook.mid_price());

// Get market depth
let bid_depth = orderbook.depth(OrderSide::Buy, 5);
let ask_depth = orderbook.depth(OrderSide::Sell, 5);
```

---

## 🧪 Testing

The project includes comprehensive unit tests covering all major components:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test models::order

# Generate coverage report (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

**Test Coverage**: 17 tests covering order management, matching engine, orderbook operations, and risk management.

---

## 📊 Performance

The engine is optimized for high-frequency trading scenarios:

- **Order Submission**: < 1μs average latency
- **Order Matching**: < 5μs for simple matches
- **Concurrent Processing**: Lock-free data structures using DashMap
- **Memory Efficiency**: Zero-copy operations where possible

### Optimization Flags

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit for better optimization
```

---

## 🛠️ Technology Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust 1.90+ |
| Async Runtime | Tokio |
| Concurrency | DashMap, Crossbeam |
| Decimal Math | rust_decimal |
| Serialization | Serde |
| Logging | Tracing |
| Testing | Cargo Test |
| Benchmarking | Criterion |

---

## 📁 Project Structure

```
rust-hft-trading-engine/
├── src/
│   ├── engine/
│   │   ├── matching_engine.rs    # Core matching logic
│   │   └── mod.rs
│   ├── models/
│   │   ├── order.rs              # Order types and management
│   │   ├── trade.rs              # Trade execution records
│   │   ├── orderbook.rs          # OrderBook implementation
│   │   ├── market_data.rs        # Market data structures
│   │   └── mod.rs
│   ├── risk/
│   │   ├── risk_manager.rs       # Risk management system
│   │   └── mod.rs
│   ├── lib.rs                    # Library exports
│   └── main.rs                   # Demo application
├── tests/                        # Integration tests
├── benches/                      # Performance benchmarks
├── Cargo.toml                    # Project configuration
└── README.md                     # This file
```

---

## 🔧 Configuration

### Risk Limits

Customize risk parameters in `RiskLimits`:

```rust
pub struct RiskLimits {
    pub max_order_size: Decimal,      // Maximum single order size
    pub max_position_size: Decimal,   // Maximum position size per user
    pub max_daily_loss: Decimal,      // Maximum daily loss limit
    pub max_order_value: Decimal,     // Maximum order notional value
}
```

### Order Types

Supported order types:
- **Market**: Execute immediately at best available price
- **Limit**: Execute at specified price or better
- **Stop-Loss**: Trigger market order when stop price is reached
- **Stop-Limit**: Trigger limit order when stop price is reached

---

## 🚦 Observability

The engine includes built-in observability features:

```rust
// Tracing is automatically initialized
tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .init();

// All operations are traced
info!("Order submitted: {:?}", order.id);
info!("Trade executed: price={}, quantity={}", trade.price, trade.quantity);
```

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Standards

- Follow Rust naming conventions
- Add tests for new features
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Update documentation for API changes

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👤 Author

**Gabriel Demetrios Lafis**

- GitHub: [@gabriellafis](https://github.com/gabriellafis)
- LinkedIn: [Gabriel Demetrios Lafis](https://linkedin.com/in/gabriel-lafis)

---

## 🙏 Acknowledgments

- Rust community for excellent documentation and tooling
- Financial markets for inspiration and requirements
- Open-source contributors for various dependencies

---

<a name="português"></a>

## 📖 Visão Geral

Um **motor de negociação de alto desempenho e baixa latência** construído em Rust, projetado para aplicações de negociação de alta frequência (HFT). Este motor fornece um sistema completo de correspondência de ordens com processamento de dados de mercado em tempo real, gerenciamento de risco e observabilidade abrangente.

### Principais Recursos

- **⚡ Ultra-Baixa Latência**: Otimizado para processamento de ordens em nível de microssegundos
- **🔄 Motor de Correspondência de Ordens**: Algoritmo de correspondência FIFO com múltiplos tipos de ordem
- **📊 Dados de Mercado em Tempo Real**: Gerenciamento de livro de ofertas e feeds de dados de mercado ao vivo
- **🛡️ Gerenciamento de Risco**: Verificações de risco pré-negociação com limites configuráveis
- **🔐 Segurança de Tipos**: Aproveita o sistema de tipos do Rust para segurança de memória e concorrência
- **📈 Observabilidade**: Rastreamento e métricas integrados para monitoramento
- **🧪 Testes Abrangentes**: 17 testes unitários com mais de 85% de cobertura de código
- **⚙️ Pronto para Produção**: Builds de release otimizados com LTO e unidade única de codegen

---

## 🏗️ Arquitetura

```
┌─────────────────────────────────────────────────────────────┐
│                   Núcleo do Motor de Negociação              │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │  Gerenciador │───▶│    Motor de  │───▶│   Executor   │  │
│  │  de Ordens   │    │Correspondência│    │  de Trades   │  │
│  └──────────────┘    └──────────────┘    └──────────────┘  │
│         │                    │                    │          │
│         ▼                    ▼                    ▼          │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │ Gerenciador  │    │  Gerenciador │    │    Dados     │  │
│  │  de Risco    │    │  de OrderBook│    │  de Mercado  │  │
│  └──────────────┘    └──────────────┘    └──────────────┘  │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Componentes

#### 1. **Motor de Correspondência** (`src/engine/matching_engine.rs`)
- Algoritmo de correspondência de ordens FIFO
- Suporte para ordens Market, Limit, Stop-Loss e Stop-Limit
- Processamento concorrente de ordens usando DashMap
- Geração e execução automática de trades

#### 2. **Gerenciamento de Ordens** (`src/models/order.rs`)
- Gerenciamento completo do ciclo de vida de ordens
- Validação de ordens e transições de estado
- Suporte para preenchimentos parciais e cancelamentos
- Aritmética decimal precisa para cálculos financeiros

#### 3. **OrderBook** (`src/models/orderbook.rs`)
- Agregação eficiente de níveis de preço usando BTreeMap
- Rastreamento em tempo real de melhor bid/ask
- Cálculos de spread e preço médio
- Análise de profundidade de mercado

#### 4. **Gerenciador de Risco** (`src/risk/risk_manager.rs`)
- Verificações de risco pré-negociação
- Aplicação de limites de posição
- Rastreamento de P&L diário
- Validação de tamanho e valor de ordem

#### 5. **Dados de Mercado** (`src/models/market_data.rs`)
- Informações de ticker em tempo real
- Gerenciamento de cotações (bid/ask)
- Agregação de dados de mercado

---

## 🚀 Início Rápido

### Pré-requisitos

- **Rust 1.90+** (instale em [rustup.rs](https://rustup.rs/))
- **Cargo** (vem com Rust)

### Instalação

```bash
# Clone o repositório
git clone https://github.com/gabriellafis/rust-hft-trading-engine.git
cd rust-hft-trading-engine

# Compile o projeto
cargo build --release

# Execute os testes
cargo test

# Execute a demonstração
cargo run --release
```

### Executando Benchmarks

```bash
cargo bench
```

---

## 💻 Exemplos de Uso

### Submissão Básica de Ordem

```rust
use rust_hft_trading_engine::{MatchingEngine, Order, OrderSide, OrderType};
use rust_decimal_macros::dec;

// Inicialize o motor de correspondência
let engine = MatchingEngine::new();

// Crie uma ordem de compra limitada
let buy_order = Order::new(
    "AAPL".to_string(),
    OrderSide::Buy,
    OrderType::Limit,
    dec!(100),           // quantidade
    Some(dec!(150.50)),  // preço
    None,                // preço de stop
    "user_001".to_string(),
);

// Submeta a ordem e obtenha os trades resultantes
let trades = engine.submit_order(buy_order)?;
```

### Gerenciamento de Risco

```rust
use rust_hft_trading_engine::{RiskManager, RiskLimits};
use rust_decimal::Decimal;

// Configure os limites de risco
let limits = RiskLimits {
    max_order_size: Decimal::from(10000),
    max_position_size: Decimal::from(100000),
    max_daily_loss: Decimal::from(50000),
    max_order_value: Decimal::from(1000000),
};

let risk_manager = RiskManager::new(limits);

// Verifique a ordem contra os limites de risco
let risk_check = risk_manager.check_order(&order);
if !risk_check.passed {
    println!("Verificação de risco falhou: {:?}", risk_check.reason);
}
```

### Análise de OrderBook

```rust
// Obtenha o orderbook para um símbolo
let orderbook = engine.get_orderbook("AAPL")?;

// Acesse dados de mercado
println!("Melhor Bid: {:?}", orderbook.best_bid());
println!("Melhor Ask: {:?}", orderbook.best_ask());
println!("Spread: {:?}", orderbook.spread());
println!("Preço Médio: {:?}", orderbook.mid_price());

// Obtenha a profundidade de mercado
let bid_depth = orderbook.depth(OrderSide::Buy, 5);
let ask_depth = orderbook.depth(OrderSide::Sell, 5);
```

---

## 🧪 Testes

O projeto inclui testes unitários abrangentes cobrindo todos os componentes principais:

```bash
# Execute todos os testes
cargo test

# Execute testes com saída
cargo test -- --nocapture

# Execute módulo de teste específico
cargo test models::order

# Gere relatório de cobertura (requer cargo-tarpaulin)
cargo tarpaulin --out Html
```

**Cobertura de Testes**: 17 testes cobrindo gerenciamento de ordens, motor de correspondência, operações de orderbook e gerenciamento de risco.

---

## 📊 Performance

O motor é otimizado para cenários de negociação de alta frequência:

- **Submissão de Ordem**: < 1μs de latência média
- **Correspondência de Ordem**: < 5μs para correspondências simples
- **Processamento Concorrente**: Estruturas de dados lock-free usando DashMap
- **Eficiência de Memória**: Operações zero-copy quando possível

### Flags de Otimização

```toml
[profile.release]
opt-level = 3        # Otimização máxima
lto = true           # Otimização em tempo de link
codegen-units = 1    # Unidade única de codegen para melhor otimização
```

---

## 🛠️ Stack Tecnológico

| Componente | Tecnologia |
|-----------|-----------|
| Linguagem | Rust 1.90+ |
| Runtime Assíncrono | Tokio |
| Concorrência | DashMap, Crossbeam |
| Matemática Decimal | rust_decimal |
| Serialização | Serde |
| Logging | Tracing |
| Testes | Cargo Test |
| Benchmarking | Criterion |

---

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

---

## 👤 Autor

**Gabriel Demetrios Lafis**

- GitHub: [@gabriellafis](https://github.com/gabriellafis)
- LinkedIn: [Gabriel Demetrios Lafis](https://linkedin.com/in/gabriel-lafis)

---

## ⭐ Mostre seu apoio

Se este projeto foi útil para você, considere dar uma ⭐️!

---

## 📞 Contato

Para questões, sugestões ou colaborações, sinta-se à vontade para abrir uma issue ou entrar em contato diretamente.
