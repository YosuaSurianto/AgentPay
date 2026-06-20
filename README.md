# AgentPay — Autonomous Payment Rails for the AI Economy 🤖💸

![Stellar](https://img.shields.io/badge/Network-Stellar_Testnet-black?style=for-the-badge&logo=stellar)
![Soroban](https://img.shields.io/badge/Smart_Contract-Soroban_Rust-orange?style=for-the-badge&logo=rust)
![x402](https://img.shields.io/badge/Protocol-x402-blue?style=for-the-badge)

## ⚡ What is AgentPay?
**AgentPay** is a production-grade infrastructure layer that enables AI agents to autonomously pay for API resources on the Stellar network using the **x402 protocol** and **Soroban Smart Contracts**. 

No API keys. No human approval. No subscriptions. Just autonomous machine-to-machine microtransactions settled in ~5 seconds.

## 🚨 The Problem
AI agents are evolving, but their economic models are broken. When an autonomous agent hits a paywalled API (e.g., premium on-chain data, ZK-proof generation), it halts. It requires a human to configure billing, manage API keys, and manually approve payments. This friction destroys the core concept of true machine autonomy at scale.

## 💡 The Solution
AgentPay implements the x402 payment protocol directly on Stellar. It acts as an **On-Chain Escrow & Authorization layer** for AI.
1. The AI Agent requests a premium resource.
2. The Server responds with HTTP `402 Payment Required`.
3. The Agent seamlessly signs a Soroban auth entry off-chain and submits it via a Relayer.
4. The AgentPay Smart Contract validates the agent's 24-hour rolling limits and executes the USDC payment.
5. The Server verifies the on-chain event and serves the data.

## 🏆 Why We Built This on Stellar
Stellar isn't just an option; it's the optimal environment for machine payments:
* **Native Soroban Auth:** Agents can sign payment payloads *off-chain* without holding native gas tokens (XLM). 
* **Relayer Friendly:** Deep integration with OpenZeppelin Relayer ensures parallel execution without sequence number bottlenecks.
* **Micro-transaction Ready:** Fees at ~$0.0001 make fractional data purchases economically viable.

## 🧠 Smart Contract Innovations (Rust / Soroban)
This isn't a simple token transfer contract. It is highly optimized for high-frequency AI interactions:
* **Zero-Cost Storage via Events:** Instead of bloating state storage with payment histories, the contract emits lightweight Soroban Events. Relayers and indexers read these for free, dropping contract execution costs significantly.
* **24-Hour Rolling Window Algorithm:** Instead of naive UTC resets, the escrow enforces precision spending limits using a dynamic 24-hour rolling window based on ledger timestamps.
* **Granular Control:** Admins can pause agents and adjust per-transaction or daily USDC limits on the fly.

## 💻 Tech Stack
* **Smart Contract:** Rust (Soroban `no_std`)
* **Backend API:** Node.js, Express, TypeScript (Paywall Simulator)
* **Frontend Demo:** Next.js, Tailwind CSS (Featuring a cyberpunk-themed, glassmorphism UI with real-time 3D terminal logs to visualize the HTTP 402 flow)
* **Facilitator:** OpenZeppelin Relayer (Stellar Testnet)

## 🚀 Quick Start (Local Setup)

### 1. Smart Contract Deployment
```bash
cd contracts
soroban contract build
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/agentpay.wasm --source <YOUR_SECRET_KEY> --network testnet