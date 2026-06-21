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
4. The AgentPay Smart Contract validates the agent's daily limits and executes the USDC payment.
5. The Server verifies the on-chain event and serves the data.

## 🏆 Why We Built This on Stellar
Stellar isn't just an option; it's the optimal environment for machine payments:
* **Native Soroban Auth:** Agents can sign payment payloads *off-chain* without holding native gas tokens (XLM). 
* **Relayer Friendly:** Deep integration with OpenZeppelin Relayer ensures parallel execution without sequence number bottlenecks.
* **Micro-transaction Ready:** Fees at ~$0.0001 make fractional data purchases economically viable.

## 🧠 Smart Contract Innovations (Rust / Soroban)
This isn't a simple token transfer contract. It is highly optimized for high-frequency AI interactions:
* **On-chain Payment History:** Every transaction is recorded directly on-chain. This allows the frontend or any client to verify exactly what resources were purchased and how much was spent without relying on an external database.
* **Automated Daily Limits:** The escrow enforces precise spending limits with automated daily resets based on ledger timestamps, ensuring agents never overspend their daily budget.
* **Granular Control:** Admins can pause agents and adjust per-transaction or daily USDC limits on the fly.

## 💻 Tech Stack
* **Smart Contract:** Rust (Soroban `no_std`)
* **Backend API:** Node.js, Express, TypeScript (Paywall Simulator)
* **Frontend Demo:** Next.js, Tailwind CSS (Featuring a cyberpunk-themed, glassmorphism UI with real-time 3D terminal logs to visualize the HTTP 402 flow)
* **Facilitator:** OpenZeppelin Relayer (Stellar Testnet)
