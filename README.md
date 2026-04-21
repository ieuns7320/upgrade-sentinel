# UpgradeSentinel

UpgradeSentinel is a hybrid Rust/Go security analysis tool for upgradeable smart contracts.

It combines:
- Rust-based static analysis
- Go-based on-chain metadata collection

## Goal
Detect risky patterns in upgradeable smart contracts, especially around:
- initializer exposure
- unlocked implementation contracts
- missing access control in upgrade paths
- dangerous delegatecall / low-level call usage
- destructive code paths
- EIP-1967 slot inspection

## Structure
- `rust-analyzer/`: static analysis core
- `go-collector/`: on-chain RPC collector
- `samples/`: safe and vulnerable sample contracts
- `docs/`: specs and notes
- `schema/`: output schema
- `reports/`: generated reports

## Status
Project bootstrap phase.
