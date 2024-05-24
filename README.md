# vrf-activity-group5
## Solution 

Group 5
Members:
```
1. Abhiraj Mengade
2. Rohit Sarpotdar
3. Oliver
4. Kishan
```

Welcome to the PBA Poker Game! This Rust-based application simulates a simplified poker game where players use cryptographic keys for unique identification and signature generation. The game also incorporates a Verifiable Random Function (VRF) to ensure fair card distribution.

## Gameplay

- Player Initialization: Each player is given a key pair and an initial balance of 1000 SGD.
- Signature Collection: Players sign a message, and a shared VRF seed is generated.
- Card Distribution: Each player receives 2 cards.
- Bidding Rounds: Players place bids, and cards are revealed on the table in several rounds.
- Winner Determination: The player with the highest card sum (including table cards) wins the game.

