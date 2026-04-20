# HustleTracker: Decentralized Freelance Gig Ledger

## 🚀 Overview
**HustleTracker** is a decentralized application (dApp) built on the **Stellar Network** using **Soroban Smart Contracts**. 

In the world of freelance and side-hustles, tracking multiple projects, clients, and payments can be messy. HustleTracker solves this by providing a transparent, immutable, and secure ledger on the blockchain. Every task you add is recorded permanently, ensuring a "single source of truth" for your professional engagements.

## ✨ Key Features
- [cite_start]**Immutable Logging**: Once a task is added, the record (client name, description, and fee) is secured by the Stellar blockchain[cite: 179].
- **CRUD Operations**: Specifically tailored for task management:
    - `add_task`: Records a new gig with specific IDs and fees.
    - [cite_start]`get_tasks`: Retrieves all your active engagements directly from the blockchain storage[cite: 362].
    - [cite_start]`delete_task`: Cleans up the ledger once a project is completed or canceled[cite: 352].
- [cite_start]**Cost-Efficient**: Leverages Stellar's low gas fees for high-frequency task updates[cite: 193].

## 🛠️ Technologies Used
- [cite_start]**Smart Contract Language**: Rust [cite: 218]
- [cite_start]**Platform**: Soroban (Stellar Smart Contracts) [cite: 213]
- [cite_start]**Network**: Stellar Testnet [cite: 386]
- [cite_start]**Development Tool**: Stellar CLI & Soroban SDK [cite: 239, 426]

## 📋 Smart Contract Details
- **Network**: Stellar Testnet
- **Contract ID**: `CCDXMBRPAVJEKN7A7WQRUQXIGJX2KX72ISSEVVCETZPCDCFDVQ7OJ4HC`

## 🚀 How to Run (Development)
If you want to invoke the contract via terminal:

1. **Install Stellar CLI**
2. **Add Task**:
   ```bash
   stellar contract invoke --id CCDXMBRPAVJEKN7A7WQRUQXIGJX2KX72ISSEVVCETZPCDCFDVQ7OJ4HC --source-account <YOUR_ACCOUNT> --network testnet -- add_task --id 1 --client_name "Acme Corp" --task_desc "UI Design" --fee 500

![Screenshot Terminal Aplikasi HustleTracker](contracts/HustleTracker.png)
