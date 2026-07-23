# 🎨 Color Alchemist

A multiplayer color puzzle game developed in Rust using Macroquad.

Players try to match a target RGB color by adjusting red, green, and blue values. The goal is to achieve the highest similarity score.

## Features

### Single Player
- RGB color guessing system
- Difficulty levels
- Timer-based challenges
- Score system
- Save and leaderboard system

### Multiplayer
- TCP client/server architecture
- Multiple players support
- Shared target colors
- Real-time guessing
- Accuracy calculation
- Winner detection
- Automatic round reset

### AI Hint System
A simple AI assistant analyzes the difference between the player's color and the target color and provides hints to improve the guess.

## Technologies

- Rust
- Macroquad
- TCP Networking
- Serde JSON
- Threads
- Arc & Mutex
- File I/O

## Controls

- Up/Down: Change Red
- Left/Right: Change Green
- Q/A: Change Blue
- Space: Submit Guess
- H: AI Hint
- Tab: Leaderboard

## Rust Concepts Used

- Structs and Impl
- Enums
- Ownership and Borrowing
- Serialization
- TCP Communication
- Multi-threading
- Synchronization with Arc and Mutex

## Future Improvements

- Better network synchronization
- More multiplayer modes
- Dynamic level downloading
- Advanced AI suggestions

## Developer

Sheyda Rahmanpour
