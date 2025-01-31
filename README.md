# 🎯 Rust Guessing Game
A simple command-line number guessing game written in **Rust**.

## 📌 Features
- Random number generation (1-100)
- User input handling
- Looping until the correct guess
- Input validation with error handling
- Simple CLI interface

## 🚀 How to Run
### 1️⃣ Clone the Repository (Optional)
If you have a GitHub repo, clone it:
```sh
git clone https://github.com/yourusername/guessing-game.git
cd guessing-game
```
Or, navigate to the project folder if created locally:
```sh
cd guessing-game
```

### 2️⃣ Install Rust
Make sure Rust is installed:
```sh
rustc --version
cargo --version
```
If not installed, install Rust with:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3️⃣ Build and Run
```sh
cargo run
```

---

## 🎮 Gameplay Example
```
🎯 Welcome to the Number Guessing Game!
🔢 Guess a number between 1 and 100!

Enter your guess: 50
📉 Too low! Try again.

Enter your guess: 75
📈 Too high! Try again.

Enter your guess: 62
🎉 You guessed it! The number was 62
```

---

## 🛠 How It Works
1. The program **randomly generates** a secret number between `1-100`.
2. The user **enters a guess**.
3. The program compares:
   - 📉 **Too low** → Try again
   - 📈 **Too high** → Try again
   - 🎉 **Correct!** → Game over
4. The game **loops until the user guesses correctly**.

---

## 📝 Code Overview
- `rand::Rng` → Generates a random number.
- `io::stdin().read_line()` → Reads user input.
- `parse()` → Converts user input (string) to a number.
- `loop` → Runs the game continuously.
- `match` → Compares user input to the secret number.
- `std::cmp::Ordering` → Determines if the guess is low, high, or correct.

---

## 🔹 Future Improvements
✅ Add difficulty modes (Easy, Medium, Hard)  
✅ Track number of attempts  
✅ Allow custom number range  

---

## 📜 License
This project is open-source under the **MIT License**.

---

## 👨‍💻 Author
Created by **Your Name**  
GitHub: [v4nilla1ce](https://github.com/v4nilla1ce)
