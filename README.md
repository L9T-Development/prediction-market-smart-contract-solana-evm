# Solana Prediction Market Smart Contract

A decentralized prediction market platform built on Solana, EVM(BASE, Xlayer, BNB ) blockchain, inspired by Polymarket. This project enables users to create markets, trade positions, and resolve outcomes based on real-world events.

## Features

- **Market Creation**: Create prediction markets for any event
- **Liquidity Provision**: Add and withdraw liquidity to markets
- **Trading**: Trade positions using Yes/No tokens
- **Market Resolution**: Automatic resolution based on final outcomes
- **Fee Structure**: Platform and LP fees for sustainable operations

## Contact

If you wanna build prediction market project like this, plz contact here: [Telegram](https://t.me/shiny0103) | [Twitter](https://x.com/0xTan1319)

## Relation project

- [Frontedn & Backend](https://github.com/0xTan1319/prediction-market-fe-be-solana)

- [EVM Version](https://github.com/0xTan1319/EVM-Prediction-Market-Smart-Contract)

## Architecture

The project is built using:

- Solana Web3.js
- Anchor Framework
- SPL Token Program
- Associated Token Program

## Getting Started

### Prerequisites

- Node.js
- Yarn
- Solana CLI
- Anchor Framework

### Installation

1. Build the program:

```bash
anchor build
```

2. Deploy the program:

```bash
anchor deploy
```

### Configuration

Configure your project settings:

```bash
yarn script config -e devnet -k <your-keypair-path> -r <your-rpc-url>
```

### Usage Examples

1. Create a new market:

```bash
yarn script market -e devnet -k <your-keypair-path> -r <your-rpc-url>
```

2. Add liquidity to a market:

```bash
yarn script addlp -y <yes-token-address> -n <no-token-address> -a <amount> -e devnet -k <your-keypair-path> -r <your-rpc-url>
```

3. Trade positions:

```bash
yarn script swap -y <yes-token-address> -n <no-token-address> -a <amount> -s <style> -t <token-type> -e devnet -k <your-keypair-path> -r <your-rpc-url>
```

4. Withdraw liquidity:

```bash
yarn script withdraw -y <yes-token-address> -n <no-token-address> -a <amount> -e devnet -k <your-keypair-path> -r <your-rpc-url>
```

5. Resolve market:

```bash
yarn script resolution -y <yes-token-address> -n <no-token-address> -e devnet -k <your-keypair-path> -r <your-rpc-url>
```

## Example Transactions

### Configuration

[61hRVTaWHMVFFrmDX8oKKecEJu2unneEZ1ojwRa8cC9i5mjGA9qf6MLkCSJq5k3dsteLcEKGLsXRnXATgnvJ3kn](https://solscan.io/tx/61hRVTaWHMVFFrmDX8oKKecEJu2unneEZ1ojwRa8cC9i5mjGA9qf6MLkCSJq5k3dsteLcEKGLsXRnXATgnvJ3kn?cluster=devnet)

### Market Creation

[4Bu1wf7LxtYNp1SZxuBJQENkpW4H1bLZYW8T4PaTBDkcmj87gGCdK7UxwxKLmo5ZbDiaHyhBGsgxQEQjAZBc8UdE](https://solscan.io/tx/4Bu1wf7LxtYNp1SZxuBJQENkpW4H1bLZYW8T4PaTBDkcmj87gGCdK7UxwxKLmo5ZbDiaHyhBGsgxQEQjAZBc8UdE?cluster=devnet)

### Add Liquidity

[2z9CsyN3pbpXgomzQb61VVm9U2Qhmc2eDsqVEDx6HePLzXnrBjVZtkfbiESbgvicWD4sTqeZ7imnLGMkj1M6NeDA](https://solscan.io/tx/2z9CsyN3pbpXgomzQb61VVm9U2Qhmc2eDsqVEDx6HePLzXnrBjVZtkfbiESbgvicWD4sTqeZ7imnLGMkj1M6NeDA?cluster=devnet)

### Withdraw Liquidity

[4itxyZS1pvXYXDwaxV2qit45BhSWzXNpgQw244xVUvpWYVoS4UctYunLzMkEENKEDoHyL37HSCQ86esv3C684XY7](https://solscan.io/tx/4itxyZS1pvXYXDwaxV2qit45BhSWzXNpgQw244xVUvpWYVoS4UctYunLzMkEENKEDoHyL37HSCQ86esv3C684XY7?cluster=devnet)

### Swap

[f1z3gXdu76gP2HDAAJpPJNC7zWB7HvqS743EtHKPPtwv7P7A2CiEbR1rEnkYwZJQ2eE1XW9ARFxWEGnzugjQCwm](https://solscan.io/tx/f1z3gXdu76gP2HDAAJpPJNC7zWB7HvqS743EtHKPPtwv7P7A2CiEbR1rEnkYwZJQ2eE1XW9ARFxWEGnzugjQCwm?cluster=devnet)

### Resolution

[4DFb3Y8fiZEHYV5KNVUP5w8E79AbMpjmJnV8yqwC9s21x2Z8BREu7UT9N1CLYAEdbatypn8HPVVcAxG675qnryfH](https://solscan.io/tx/4DFb3Y8fiZEHYV5KNVUP5w8E79AbMpjmJnV8yqwC9s21x2Z8BREu7UT9N1CLYAEdbatypn8HPVVcAxG675qnryfH?cluster=devnet)

