# 📚 Stellar Flashcard Smart Contract

## 📖 Description

This project is a **decentralized flashcard application** built using **Stellar Soroban Smart Contracts**.
It allows users to create, store, search, and manage flashcards directly on the blockchain.

Each flashcard consists of:

* **ID** → unique identifier
* **Question** → the front side of the flashcard
* **Answer** → the back side of the flashcard

This application is designed to help students learn and memorize concepts in a simple and decentralized way.

---

## ✨ Features

* ➕ Create new flashcards (question & answer)
* 📄 Retrieve all flashcards
* ❌ Delete flashcards by ID
* 🔎 Search flashcards (exact match by question)
* 🎲 Get a random flashcard
* 💡 Get a hint (simple guidance)

---

## 🧠 How It Works

* All flashcards are stored on-chain using Soroban storage.
* Each flashcard is assigned a unique ID using a pseudo-random generator.
* Users can interact with the contract to manage their learning data securely and transparently.

---

## 🏗️ Smart Contract Structure

### Data Structure

```rust
pub struct Flashcard {
    id: u64,
    question: String,
    answer: String,
}
```

### Storage

* Uses key: `FLASHCARD`
* Data is stored as `Vec<Flashcard>`

---

## ⚙️ Available Functions

### 1. Get All Flashcards

```rust
get_flashcards(env: Env) -> Vec<Flashcard>
```

Returns all stored flashcards.

---

### 2. Create Flashcard

```rust
create_flashcard(env: Env, question: String, answer: String) -> String
```

Creates a new flashcard and stores it on-chain.

---

### 3. Delete Flashcard

```rust
delete_flashcard(env: Env, id: u64) -> String
```

Deletes a flashcard by its ID.

---

### 4. Search Flashcard

```rust
search_flashcard(env: Env, keyword: String) -> Vec<Flashcard>
```

Searches flashcards using exact match on the question.

---

### 5. Get Random Flashcard

```rust
get_random_flashcard(env: Env) -> Option<Flashcard>
```

Returns a random flashcard from storage.

---

### 6. Get Hint

```rust
get_hint(env: Env, id: u64) -> String
```

Provides a simple hint related to the flashcard.

---

## 🚀 Deployment

### 🌐 Network

* Stellar Soroban Testnet

### 🆔 Contract ID

```
CB3RY6FADUTZWLZ6UASBRH2CXOZJYAMQCOSGXVH7A5CS7LHM6OBVQYQX
```

---

## 🛠️ Build Instructions

### 1. Build Project

```bash
cargo build
```

### 2. Compile to WASM

```bash
soroban contract build
```

---

## 📸 Testnet Interaction (Screenshots Required)

Include screenshots of:

* Contract deployment
* Creating flashcard
* Retrieving flashcards
* Random flashcard result


---

## 🎯 Use Case

This application can be used by:

* Students for studying and memorization
* Learners preparing for exams
* Anyone who wants a simple decentralized learning tool

---

## 📂 Repository Name

```
stellar-flashcard-dapp
```

---

## 🧩 Future Improvements

* Add difficulty level
* Add scoring system
* Add spaced repetition logic
* Integrate with frontend UI

---

## 👨‍💻 Author

* Name: Dirga
* Role: Student Developer

---

## 📜 Notes

This project is created for educational purposes as part of a Stellar Soroban workshop.
