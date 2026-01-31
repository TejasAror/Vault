# Vault Program (Solana + Anchor)

This repository contains a simple Vault smart contract built using Solana Anchor (Rust).

The purpose of this program is to demonstrate basic state management on Solana, where a vault account stores balances and allows deposit and withdrawal operations.

This implementation focuses on program structure and correctness, not real SOL custody.

---

## Features

- Initialize a vault account
- Deposit (increase stored balance)
- Withdraw (decrease stored balance with validation)
- Custom error handling


## Instructions

### `deposit`
Adds the specified amount to the vault balance.

### `withdraw`
Removes the specified amount from the vault balance.
Fails if the balance is insufficient.



## Build Instructions

```bash
     anchor build
