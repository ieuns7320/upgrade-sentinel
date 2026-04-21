# UpgradeSentinel Spec

## Project Goal
Analyze upgradeable smart contracts and detect security-relevant suspicious patterns using static analysis and on-chain metadata.

## Components
- Rust analyzer: parse AST/build output and run detection rules
- Go collector: collect EIP-1967 slot values and contract metadata via RPC

## Initial Detection Rules
- UPG-001 Initializer exposure
- UPG-002 Unlocked implementation contract
- UPG-003 Missing access control on upgrade path
- UPG-004 Dangerous delegatecall / low-level call usage
- UPG-005 Destructive code path
- UPG-006 EIP-1967 slot inspection

## Outputs
- CLI summary
- JSON report
- Markdown report
