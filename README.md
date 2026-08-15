IRONVAULT

Local. Encrypted. Yours.

IRONVAULT is a local-first encrypted file vault built for people who want their files stored on their machine, not floating around in some random cloud.

No accounts. No required servers. No fake security screens. Just encrypted storage.

What It Does
Local encrypted file storage
Master-password protected vaults
Folder and file organization
Fast file search
File import and export
Safe previews for supported files
Automatic vault locking
Local activity logging
Command palette and keyboard shortcuts
Configurable security settings
Persistent local database
Path-traversal and input protection
Architecture
IRONVAULT
│
├── UI
├── Vault Engine
├── Cryptography
├── Storage
├── Database
├── Search
├── Security
├── Configuration
└── Logging

The UI doesn't handle everything. The vault engine doesn't depend on the UI. Security-critical operations stay isolated.

Security

IRONVAULT is designed around one rule:

Your password unlocks the vault. It doesn't become the vault.

The master password is never stored as plaintext.

Vault data is encrypted locally using modern authenticated encryption and a password-based key derivation process.

IRONVAULT also includes protection against:

Incorrect passwords
Corrupted vault data
Unsafe file paths
Interrupted file operations
Permission failures
Invalid input
Automatic locking
Important

IRONVAULT is an open-source project, not a guarantee of perfect security.

Do not use it for highly sensitive data without independently reviewing the implementation and threat model.

Getting Started

Clone the repository:

git clone https://github.com/YOUR_USERNAME/ironvault.git
cd ironvault

Install dependencies:

# See project-specific instructions

Start the application:

# See project-specific instructions

Create a vault, choose a strong password, and start importing files.

Testing

Run the test suite:

# Project test command

Tests cover:

Encryption and decryption
Vault creation
Vault unlocking
File import and export
Database operations
Incorrect passwords
Corrupted data
Path-traversal protection
Design Philosophy

IRONVAULT isn't trying to be another bloated cloud storage platform.

It's built around three things:

LOCAL
Your vault lives on your machine.

PRIVATE
Files are encrypted before being stored.

SIMPLE
Open it. Unlock it. Use it. Lock it.

No unnecessary ecosystem.

Project Status

Active Development

IRONVAULT is being built as a serious open-source project.

Features may change as the security model, storage engine, and interface evolve.

Roadmap
 Vault creation
 Vault unlocking
 Encrypted storage
 File management
 Search
 Auto-lock
 Activity logging
 Vault backup system
 Vault integrity verification
 Advanced file indexing
 Performance benchmarks
 Security audit
 Cross-platform release
License

MIT License.

Use it, study it, modify it, improve it.

IRONVAULT

Local. Encrypted. Yours.
