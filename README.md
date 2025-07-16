# Solana Prediction Market Smart Contract

A decentralized prediction market platform built on Solana blockchain, inspired by Polymarket. This project enables users to create markets, trade positions, and resolve outcomes based on real-world events.

## Features

- **Market Creation**: Create prediction markets for any event
- **Liquidity Provision**: Add and withdraw liquidity to markets
- **Trading**: Trade positions using Yes/No tokens
- **Market Resolution**: Automatic resolution based on final outcomes
- **Fee Structure**: Platform and LP fees for sustainable operations

## Contact

If you wanna build prediction market project like this, plz contact here: [Telegram](https://t.me/shiny0103) | [Twitter](https://x.com/0xTan1319)

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

[3PsfbvzAPyhwNPQm2aCTf3XUaJwwrZTCoVVw41uJMzs3DXvx948JUrtW2KQRH1UkoTNFxMSbpN32KF5aFQuZ7mCc](https://solscan.io/tx/3PsfbvzAPyhwNPQm2aCTf3XUaJwwrZTCoVVw41uJMzs3DXvx948JUrtW2KQRH1UkoTNFxMSbpN32KF5aFQuZ7mCc?cluster=custom&customUrl=https://api.devnet.solana.com)

### Market Creation

[4xnzHarhppyWKJccQh27TUCTyeR2da8JGSXWsKAYw5YvVywKKWJzSz9JxRzNwhuj7fjmrCAhtM2drWc29a8J3i2C](https://solscan.io/tx/4xnzHarhppyWKJccQh27TUCTyeR2da8JGSXWsKAYw5YvVywKKWJzSz9JxRzNwhuj7fjmrCAhtM2drWc29a8J3i2C?cluster=custom&customUrl=https://api.devnet.solana.com)

### Add Liquidity

[3kfdmxfq1U6JKwo7p5aAHQ2QeaF6pxp41ycN2wDTBToujEWQMewSfNpwCSBTLuxFsy1MAUojqfkWkAqAMo63b94k](https://solscan.io/tx/3kfdmxfq1U6JKwo7p5aAHQ2QeaF6pxp41ycN2wDTBToujEWQMewSfNpwCSBTLuxFsy1MAUojqfkWkAqAMo63b94k?cluster=custom&customUrl=https://api.devnet.solana.com)

### Withdraw Liquidity

[5QDonNfWURYyGrQQwTQWjccsnmRuWSNDb66WuuXo12cWhwXo2SwbvSrRpimXPkAbsSmMzD8iYaoEdf5CgMFdtEk3](https://solscan.io/tx/5QDonNfWURYyGrQQwTQWjccsnmRuWSNDb66WuuXo12cWhwXo2SwbvSrRpimXPkAbsSmMzD8iYaoEdf5CgMFdtEk3?cluster=custom&customUrl=https://api.devnet.solana.com)

### Swap

[5s9xy2no9YANBCsp8Zr5fBq6whHK65t7eBU3AxLp6xFNcagvvzhpknWrJHrk8BYLfnf4jF6kzeW4QyuZi4UUKQKX](https://solscan.io/tx/5s9xy2no9YANBCsp8Zr5fBq6whHK65t7eBU3AxLp6xFNcagvvzhpknWrJHrk8BYLfnf4jF6kzeW4QyuZi4UUKQKX?cluster=custom&customUrl=https://api.devnet.solana.com)

### Resolution

[TryUfcHXKTVWTY3vM1bUCckbtBCxnHBrbB92LFCpRf67J5po2HsqvPT5wWNYSQJVWBcvcvWbS42KfcX6vufdUup](https://solscan.io/tx/TryUfcHXKTVWTY3vM1bUCckbtBCxnHBrbB92LFCpRf67J5po2HsqvPT5wWNYSQJVWBcvcvWbS42KfcX6vufdUup?cluster=custom&customUrl=https://api.devnet.solana.com)

