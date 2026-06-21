"use client";

import { useEffect, useState } from "react";
import { isConnected, requestAccess, getAddress } from "@stellar/freighter-api";

export default function Home() {
  const [logs, setLogs] = useState<string[]>([]);
  const [walletAddress, setWalletAddress] = useState<string>("");
  const [agentRunning, setAgentRunning] = useState<boolean>(false);
  const [agentPaused, setAgentPaused] = useState<boolean>(false);

  const connectWallet = async () => {
    try {
      if (await isConnected()) {
        const access = await requestAccess();
        if (access) {
          const address = await getAddress();
          setWalletAddress(address);
        }
      } else {
        // Mock fallback for demo if Freighter is missing
        setWalletAddress("GBBD...FLA5");
        console.warn("Freighter not installed, using mock address for demo.");
      }
    } catch (e) {
      console.error("Wallet connection failed:", e);
      // Fallback for hackathon demo
      setWalletAddress("GBBD...FLA5");
    }
  };
  
  const initialLogs = [
    '<span class="log-time">[14:32:01]</span> <span class="log-info">Initializing AgentPay Core v2.0...</span>',
    '<span class="log-time">[14:32:02]</span> Connecting to Stellar Soroban Network...',
    '<span class="log-time">[14:32:03]</span> <span class="log-info">Agent #9042: Detected HTTP 402 Payment Required</span>',
    '<span class="log-time">[14:32:04]</span> Generating off-chain signature payload...',
    '<span class="log-time">[14:32:05]</span> <span class="log-warning">Validating daily spending limit (0.005/1.000 USDC used)</span>',
    '<span class="log-time">[14:32:06]</span> Submitting to Relayer...',
    '<span class="log-time">[14:32:08]</span> <span class="log-success">Payment Confirmed | Tx: 0x8a...4b2f</span>',
    '<span class="log-time">[14:32:08]</span> <span class="log-info">Resource Unlocked. Agent resuming task.</span>'
  ];

  const runAgent = () => {
    if (agentPaused) {
      setLogs(prev => [...prev, '<span class="log-time">[' + new Date().toLocaleTimeString('en-US', { hour12: false }) + ']</span> <span style="color: #ef4444">❌ Payment rejected: agent is paused</span>']);
      return;
    }
    if (agentRunning) return;
    
    setAgentRunning(true);
    setLogs([]); 
    let currentLog = 0;
    const interval = setInterval(() => {
      if (currentLog < initialLogs.length) {
        setLogs(prev => [...prev, initialLogs[currentLog]]);
        currentLog++;
      } else {
        clearInterval(interval);
        setAgentRunning(false);
      }
    }, 800);
  };

  const togglePause = () => {
    setAgentPaused(!agentPaused);
  };

  return (
    <main>
      <div className="bg-grid"></div>
      
      <div className="container">
        {/* Navbar */}
        <nav className="navbar">
          <div className="logo">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M13 2L3 14H12L11 22L21 10H12L13 2Z" fill="#00C6FF" stroke="#00C6FF" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
            AgentPay
          </div>
          <div className="nav-links">
            <a href="#dashboard" onClick={(e) => { e.preventDefault(); document.querySelector('.dashboard-panel')?.scrollIntoView({ behavior: 'smooth' }); }}>Dashboard</a>
            <a href="#features" onClick={(e) => { e.preventDefault(); document.querySelector('.features-grid')?.scrollIntoView({ behavior: 'smooth' }); }}>Features</a>
            <a href="#history" onClick={(e) => { e.preventDefault(); document.querySelector('.history-section')?.scrollIntoView({ behavior: 'smooth' }); }}>History</a>
          </div>
          <button 
            className="btn btn-secondary" 
            style={{ padding: "0.5rem 1.5rem", fontSize: "0.875rem" }}
            onClick={connectWallet}
          >
            {walletAddress ? `${walletAddress.slice(0, 4)}...${walletAddress.slice(-4)}` : "Connect Wallet"}
          </button>
        </nav>

        {/* Hero Section */}
        <div className="flex-col flex-center" style={{ marginTop: "4rem" }}>
          <h1>Autonomous Payments<br/>for the AI Economy.</h1>
          <p className="subtitle">
            The ultimate Web3 payment gateway powered by intelligent agents. Secure, fast, and fully decentralized via Stellar Soroban.
          </p>
          
          <div className="dashboard-panel" style={{ background: "rgba(0,0,0,0.5)", border: "1px solid #333", padding: "1.5rem", borderRadius: "12px", width: "100%", maxWidth: "800px", marginBottom: "2rem", display: "flex", justifyContent: "space-between", alignItems: "center", flexWrap: "wrap", gap: "1rem" }}>
            <div style={{ display: "flex", gap: "2rem" }}>
              <div>
                <div style={{ fontSize: "0.8rem", color: "#888", textTransform: "uppercase", letterSpacing: "1px" }}>Contract Balance</div>
                <div style={{ fontSize: "1.5rem", fontWeight: "bold", color: "#00C6FF" }}>10.00 USDC</div>
              </div>
              <div>
                <div style={{ fontSize: "0.8rem", color: "#888", textTransform: "uppercase", letterSpacing: "1px" }}>Daily Spent</div>
                <div style={{ fontSize: "1.5rem", fontWeight: "bold", color: "#10B981" }}>0.005 / 1.0 USDC</div>
              </div>
            </div>
            <div className="btn-group" style={{ margin: 0 }}>
              <button className="btn btn-primary" onClick={runAgent} disabled={agentRunning}>
                {agentRunning ? "Running..." : "Run Agent Task"}
              </button>
              <button 
                className="btn btn-secondary" 
                style={{ borderColor: agentPaused ? "#10B981" : "rgba(239, 68, 68, 0.5)", color: agentPaused ? "#10B981" : "#ef4444" }}
                onClick={togglePause}
              >
                {agentPaused ? "Resume Agent" : "Pause Agent"}
              </button>
            </div>
          </div>

          {/* Terminal View */}
          <div className="terminal-wrapper">
            <div className="terminal-glass">
              <div className="terminal-header">
                <div className="window-controls">
                  <div className="control-dot dot-red"></div>
                  <div className="control-dot dot-yellow"></div>
                  <div className="control-dot dot-green"></div>
                </div>
                <div className="terminal-title">AgentPay Terminal</div>
                <div style={{ width: "44px" }}></div> {/* Spacer for symmetry */}
              </div>
              
              <div className="terminal-body">
                {logs.map((log, index) => (
                  <div 
                    key={index} 
                    className="log-line" 
                    dangerouslySetInnerHTML={{ __html: log }} 
                  />
                ))}
                {agentRunning && (
                  <div className="log-line">
                    <span className="log-time">[{new Date().toLocaleTimeString('en-US', { hour12: false, hour: "numeric", minute: "numeric", second: "numeric" })}]</span> <span style={{ animation: "pulse 1s infinite" }}>_</span>
                  </div>
                )}
              </div>
            </div>
          </div>

        </div>

        {/* Features Grid */}
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon" style={{ color: '#00C6FF', background: '#E0F7FF' }}>
              <svg width="24" height="24" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
            </div>
            <h3 className="feature-title">x402 Protocol Native</h3>
            <p className="feature-desc">Seamless integration with the standard x402 HTTP protocol for truly autonomous paywalls.</p>
          </div>
          
          <div className="feature-card">
            <div className="feature-icon" style={{ color: '#8B5CF6', background: '#F3E8FF' }}>
              <svg width="24" height="24" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
            </div>
            <h3 className="feature-title">Automated Daily Limits</h3>
            <p className="feature-desc">Precision spending caps ensure your agents never exceed your budget during autonomous operations.</p>
          </div>

          <div className="feature-card">
            <div className="feature-icon" style={{ color: '#10B981', background: '#D1FAE5' }}>
              <svg width="24" height="24" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 002-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg>
            </div>
            <h3 className="feature-title">On-chain History</h3>
            <p className="feature-desc">Every transaction is recorded directly on-chain, allowing the frontend to verify purchases natively.</p>
          </div>
        </div>

        {/* Payment History Table */}
        <div className="history-section" style={{ paddingBottom: "4rem", width: "100%", maxWidth: "800px", margin: "4rem auto 0 auto" }}>
          <h2 style={{ marginBottom: "1.5rem", fontSize: "1.5rem" }}>On-Chain Payment History</h2>
          <div style={{ background: "rgba(0,0,0,0.5)", border: "1px solid #333", borderRadius: "12px", overflow: "hidden" }}>
            <table style={{ width: "100%", textAlign: "left", borderCollapse: "collapse" }}>
              <thead>
                <tr style={{ background: "rgba(255,255,255,0.05)", borderBottom: "1px solid #333" }}>
                  <th style={{ padding: "1rem", color: "#888", fontWeight: "normal" }}>Resource</th>
                  <th style={{ padding: "1rem", color: "#888", fontWeight: "normal" }}>Amount</th>
                  <th style={{ padding: "1rem", color: "#888", fontWeight: "normal" }}>Tx Hash</th>
                  <th style={{ padding: "1rem", color: "#888", fontWeight: "normal" }}>Time</th>
                </tr>
              </thead>
              <tbody>
                <tr style={{ borderBottom: "1px solid #222" }}>
                  <td style={{ padding: "1rem", color: "#e2e8f0" }}>/api/crypto-price</td>
                  <td style={{ padding: "1rem", color: "#10B981" }}>0.001 USDC</td>
                  <td style={{ padding: "1rem", color: "#00C6FF" }}>0x8a...4b2f</td>
                  <td style={{ padding: "1rem", color: "#888" }}>14:32:08</td>
                </tr>
                <tr style={{ borderBottom: "1px solid #222" }}>
                  <td style={{ padding: "1rem", color: "#e2e8f0" }}>/api/weather</td>
                  <td style={{ padding: "1rem", color: "#10B981" }}>0.0005 USDC</td>
                  <td style={{ padding: "1rem", color: "#00C6FF" }}>0x91...1c4a</td>
                  <td style={{ padding: "1rem", color: "#888" }}>12:15:22</td>
                </tr>
                <tr>
                  <td style={{ padding: "1rem", color: "#e2e8f0" }}>/api/ai-summary</td>
                  <td style={{ padding: "1rem", color: "#10B981" }}>0.002 USDC</td>
                  <td style={{ padding: "1rem", color: "#00C6FF" }}>0xf3...8e9d</td>
                  <td style={{ padding: "1rem", color: "#888" }}>09:05:11</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

      </div>
    </main>
  );
}
