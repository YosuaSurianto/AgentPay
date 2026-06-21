# AgentPay — PRD & MVP (v2, Updated)
### Stellar Hackathon | Solo Dev | 1 Day | Soroban Smart Contract Required

---

## APA ITU AGENTPAY?

AgentPay adalah **programmable payment wallet untuk AI agent**, dibangun di atas Soroban (Stellar).

Analoginya simpel:

> Bayangkan kamu punya asisten AI yang bertugas riset pasar setiap hari — dia perlu beli data harga crypto, data cuaca, laporan keuangan dari berbagai API berbayar. Tanpa AgentPay, kamu harus setup API key satu-satu, isi billing, approve setiap transaksi secara manual.
>
> Dengan AgentPay: kamu deploy satu smart contract, deposit USDC, set batas pengeluaran, dan AI agent bisa bayar sendiri — secara otonom, sesuai rules yang kamu tentukan on-chain.

**Satu kalimat:** AgentPay adalah on-chain spending account untuk AI agent dengan programmable rules — kalau agent mau bayar lebih dari batas, kontrak otomatis menolak.

---

## KENAPA INI ADA?

### Masalah nyata yang belum terpecahkan:

AI agent sedang meledak. Tapi ada satu celah fatal:

```
AI Agent → mau beli data dari API berbayar
         → butuh kartu kredit / API key / human approval
         → STUCK. Agent berhenti. Manusia harus intervensi.
```

Ini membunuh autonomy. Agent tidak bisa scale kalau setiap payment butuh manusia.

### Solusi AgentPay:

```
Owner → deploy AgentPay contract
      → deposit USDC ke contract
      → set rules: max $0.01/tx, max $1/hari, boleh pause kapan saja

AI Agent → detect API butuh bayar (HTTP 402)
         → call AgentPay.pay() di Stellar
         → contract enforce rules on-chain
         → kalau valid → USDC langsung transfer ke API provider
         → agent dapat response ✅

Semua terjadi dalam 5 detik. Zero human intervention.
```

---

## KENAPA STELLAR, BUKAN ETH / SOLANA?

Ini wajib kamu hafal untuk juri:

| Faktor | Stellar | Ethereum | Solana |
|--------|---------|----------|--------|
| Fee per tx | ~$0.0001 | ~$0.50–$5 | ~$0.00025 |
| Settlement | ~5 detik | ~15 detik | ~0.5 detik |
| Native USDC | ✅ | ✅ | ✅ |
| Soroban auth native | ✅ | ❌ | ❌ |
| x402 facilitator live | ✅ Maret 2026 | ❌ | ✅ |

**Poin kritis untuk juri:** Agent micropayment tidak feasible di Ethereum karena fee $2 untuk bayar data $0.001 = tidak masuk akal. Di Stellar, fee $0.0001 untuk bayar $0.001 = **masuk akal secara ekonomi**.

---

## KOMPONEN PRODUK

### 1. Soroban Smart Contract (INTI — WAJIB) ⬅ ini yang kamu build di Soroban Studio

**AgentPayContract** — programmable spending wallet

| Function | Siapa yang call | Apa yang dilakukan |
|----------|----------------|-------------------|
| `initialize(owner, token, limit_per_tx, daily_limit)` | Owner (1x) | Setup wallet |
| `deposit(from, amount)` | Owner/funder | Top up USDC ke contract |
| `pay(caller, resource, amount, recipient)` | AI Agent (owner) | Bayar API, enforce rules |
| `set_active(caller, active)` | Owner | Pause/resume agent |
| `update_limits(caller, new_limits)` | Owner | Ubah spending rules |
| `withdraw(caller, amount)` | Owner | Tarik sisa dana |
| `get_balance()` | Anyone | Cek saldo |
| `get_payment(index)` | Anyone | Lihat riwayat pembayaran |
| `get_daily_spent()` | Anyone | Cek pengeluaran hari ini |

**Rules yang di-enforce on-chain:**
- ✅ Per-tx spending limit (reject kalau over)
- ✅ Daily spending limit (auto-reset tiap hari)
- ✅ Agent harus aktif (bisa di-pause owner kapan saja)
- ✅ Balance harus cukup
- ✅ Hanya owner yang bisa trigger payment

### 2. Demo Frontend (Next.js / plain HTML — pilih yang paling cepat)

Tujuan frontend bukan untuk impress dengan UI. Tujuannya: **buat juri bisa lihat alur pembayaran secara visual.**

Yang harus ada di UI:
- [ ] Connect wallet (Freighter)
- [ ] Display: contract balance, spending limits, daily spent
- [ ] Tombol "Run Agent Task" → agent mock call beberapa endpoint berbayar
- [ ] Real-time log: `[Agent] Paying 0.001 USDC for /api/crypto-price...`
- [ ] Payment history table (dari on-chain data)
- [ ] Tombol Pause Agent (demo safety mechanism)

### 3. Mock API Server (Express.js — minimal, bisa localhost)

3 endpoint yang return HTTP 402 kalau tidak ada payment header:

```
GET /api/crypto-price  → $0.001 USDC
GET /api/weather       → $0.0005 USDC
GET /api/ai-summary    → $0.002 USDC
```

---

## MVP SCOPE — YANG WAJIB ADA vs NICE TO HAVE

### ✅ WAJIB (tanpa ini tidak bisa submit):
1. Soroban contract deployed di testnet
2. `initialize`, `deposit`, `pay` function berjalan
3. Spending limit enforcement bekerja
4. Payment history tersimpan on-chain
5. GitHub repo + README English
6. Frontend bisa connect ke contract (walau sederhana)

### 🎯 NICE TO HAVE (kalau ada waktu, tambahkan):
- Daily limit reset otomatis
- Pause/resume agent dari UI
- Real-time payment log
- Mock API server yang beneran return 402

### ❌ JANGAN DIKERJAKAN (buang waktu):
- Mobile responsive UI
- Multi-agent management
- Custom token / stablecoin baru
- Mainnet deployment
- Authentication system

---

## TIMELINE — JAM PER JAM

> Asumsi mulai sekarang. Total ~7–8 jam coding.

```
JAM 1 (Setup & Contract)
├── 0:00–0:20  Setup Soroban Studio, paste lib.rs yang sudah ada
├── 0:20–0:45  Baca dan pahami contract, sesuaikan jika perlu
└── 0:45–1:00  Run tests di Soroban Studio → semua harus green

JAM 2 (Deploy Contract)
├── 1:00–1:30  Deploy ke Stellar testnet via Soroban Studio
├── 1:30–1:45  Catat contract address, test invoke fungsi di explorer
└── 1:45–2:00  Fund wallet testnet via Friendbot, test deposit USDC

JAM 3–4 (Frontend Core)
├── 2:00–2:45  Setup Next.js + install @stellar/stellar-sdk + Freighter adapter
├── 2:45–3:30  Connect wallet + display balance dari contract
└── 3:30–4:00  Implement tombol deposit + tampilkan di UI

JAM 5–6 (Agent Demo Flow)
├── 4:00–4:45  Buat mock API server (3 endpoint return 402)
├── 4:45–5:30  Buat agent client yang auto-call pay() di contract
└── 5:30–6:00  Connect agent ke UI, tampilkan real-time log

JAM 7 (Polish + README)
├── 6:00–6:30  Fix bugs, test end-to-end full flow
├── 6:30–7:00  Tulis README (template ada di bawah)
└── 7:00–7:15  Push ke GitHub, submit

JAM 7.5 (Buffer / Darurat)
└── 7:15–7:30  Fix kalau ada yang masih broken
```

---

## TECH STACK FINAL

| Layer | Tech | Catatan |
|-------|------|---------|
| Smart Contract | Rust + Soroban SDK | Di Soroban Studio |
| Frontend | Next.js 14 + TailwindCSS | Atau plain HTML kalau mepet waktu |
| Stellar integration | @stellar/stellar-sdk (JS) | Official SDK |
| Wallet | Freighter | Browser extension, testnet support |
| Mock API | Express.js + TypeScript | Localhost OK untuk demo |
| Deploy frontend | Vercel atau localhost | Vercel lebih clean untuk demo |
| Testnet funding | Friendbot (stellar.org/lab) | Gratis |

---

## ARCHITECTURE DIAGRAM

```
┌──────────────────────────────────────────────┐
│              OWNER (kamu / juri lihat)        │
│         Connect Freighter Wallet              │
└─────────────────┬────────────────────────────┘
                  │ deposit USDC + set limits
                  ▼
┌──────────────────────────────────────────────┐
│        AGENTPAY SMART CONTRACT               │
│           (Soroban Testnet)                  │
│                                              │
│  Storage:                                    │
│  - balance: 10 USDC                         │
│  - spending_limit: 0.01 USDC/tx             │
│  - daily_limit: 1 USDC/day                  │
│  - payment_history: [...]                    │
│  - is_active: true                           │
└──────────────┬───────────────────────────────┘
               │ pay(resource, amount, recipient)
               │ ← enforces rules on-chain
               ▼
┌──────────────────────────────────────────────┐
│            AI AGENT (mock client)            │
│                                              │
│  1. GET /api/crypto-price                    │
│     ← 402: "pay 0.001 USDC"                 │
│  2. call AgentPay.pay()                      │
│     ← contract validate + transfer USDC     │
│  3. GET /api/crypto-price + payment proof    │
│     ← 200: { bitcoin: $104,250 } ✅         │
└──────────────────────────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────────┐
│          MOCK API SERVER                     │
│          (Express.js, localhost)             │
│                                              │
│  GET /api/crypto-price → 402 / 200           │
│  GET /api/weather      → 402 / 200           │
│  GET /api/ai-summary   → 402 / 200           │
└──────────────────────────────────────────────┘
```

---

## DEMO SCRIPT (hafal ini — 2 menit)

### Opening (20 detik):
> "AI agent hari ini masih stuck setiap kali ketemu API berbayar. Mereka butuh manusia untuk isi billing, setup API key, approve tiap transaksi. AgentPay solve ini dengan satu Soroban contract — agent bisa bayar sendiri, on-chain, sesuai rules yang owner set."

### Live Demo (80 detik):
1. Tunjuk contract di Stellar testnet explorer — sudah deployed
2. Tunjuk UI: balance 10 USDC, limit $0.01/tx, agent active
3. Klik **"Run Agent Task"**
4. Tunjuk log muncul:
   - `[Agent] Requesting crypto prices...`
   - `[Agent] Hit paywall → $0.001 USDC required`
   - `[Agent] Calling AgentPay.pay() on Stellar...`
   - `[Agent] ✅ Payment confirmed — tx: GABCD...`
   - `[Agent] Got data: BTC $104,250`
5. Tunjuk payment history table — tercatat on-chain
6. Klik **"Pause Agent"** — tunjuk kalau agent langsung tidak bisa bayar lagi
   - `[Agent] ❌ Payment rejected: agent is paused`

### Closing (20 detik):
> "Ini bukan sekedar payment app. Ini infrastructure untuk agent economy — di mana mesin bayar mesin, tanpa manusia di tengah. Dan Stellar adalah satu-satunya chain di mana micropayment $0.001 ini economically feasible karena fee-nya $0.0001."

---

## README TEMPLATE (English — copy paste, edit bagian [...])

```markdown
# AgentPay — Programmable Payment Wallet for AI Agents on Stellar

## Overview
AgentPay is a Soroban smart contract that acts as an autonomous payment 
wallet for AI agents on Stellar. It enables AI agents to pay for API 
resources on-chain — with programmable spending rules enforced by the 
contract, no human approval required per transaction.

## Problem
AI agents break when they encounter paywalled APIs. They need humans to 
configure billing, manage API keys, and approve payments — killing 
autonomy at scale.

## Solution
AgentPay provides:
- **On-chain spending rules**: per-transaction limit, daily limit
- **Autonomous payments**: agent calls pay() and USDC transfers instantly  
- **Owner controls**: pause agent, update limits, withdraw funds anytime
- **Payment history**: every purchase recorded on-chain, queryable

## How It Works
1. Owner deploys AgentPay contract and deposits USDC
2. Owner sets spending limits (e.g. max $0.01/tx, max $1/day)
3. AI agent detects paywalled API (HTTP 402 response)
4. Agent calls `AgentPay.pay()` on Stellar
5. Contract enforces rules → transfers USDC to API provider
6. Agent receives resource — ~5 second settlement, $0.0001 fee

## Why Stellar
- Fee ~$0.0001/tx: micropayments are economically viable
- Native USDC: no wrapping needed
- Soroban authorization: secure on-chain rule enforcement
- ~5 second settlement: fast enough for real-time agent workflows

## Contract Functions
| Function | Description |
|----------|-------------|
| `initialize` | Deploy and configure the agent wallet |
| `deposit` | Fund the wallet with USDC |
| `pay` | Execute a payment (enforces all rules) |
| `set_active` | Pause or resume the agent |
| `update_limits` | Change spending limits |
| `withdraw` | Pull remaining funds |
| `get_balance` | Check wallet balance |
| `get_payment` | Query payment history |

## Setup
```bash
git clone https://github.com/[your-username]/agentpay
cd agentpay
npm install
cp .env.example .env
# Fill in your Stellar testnet keypair
npm run dev
```

## Network
- **Network**: Stellar Testnet
- **Contract Address**: [paste after deploy]
- **Explorer**: https://stellar.expert/explorer/testnet/contract/[address]

## Demo
[your deployed URL]
```

---

## RISIKO & MITIGASI

| Risiko | Kemungkinan | Mitigasi |
|--------|-------------|----------|
| USDC testnet address salah | Medium | Gunakan USDC testnet: `GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5` |
| Soroban Studio error compile | Medium | Test function satu-satu, mulai dari initialize dulu |
| Freighter connect gagal | Low | Fallback: hardcode keypair untuk demo (testnet only) |
| Frontend tidak selesai | Medium | Prioritas backend + contract. UI minimal = tabel + 1 tombol sudah cukup |
| Demo lag saat presentasi | Low | Rekam screen recording sebagai backup |

---

*v2 — Updated Juni 2026 | Contract: Soroban Protocol 26 Testnet*