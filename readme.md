# Orbiter Checker
![Orbiter Finance](src/logo.png)
- This is a simple Rust program that checks the Orbiter API for a list of wallets and calculates the potential value of the tokens in USD.
- If your wallet is eligible, it will print the potential value of the tokens in USD.
- The program is built with Rust and uses the `reqwest` crate to make HTTP requests.
- The script is run asynchronously using the `tokio` crate.
by [0xKoiner](https://github.com/0xkoiner)


## Usage
1. Insert your wallets in the `wallets.txt` file.
```solidity
├──[-] src
│   ├── wallets.txt

Example:

0x1234567890123456789012345678901234567890
0x9876543210987654321098765432109876543210
```
2. Build the program
```bash
cargo build
```
3. Run the program
```bash
cargo run
```
## Connect with Me

<p>
    <a href="https://www.linkedin.com/in/alex-gray-0xff/" target="_blank">
        <img src="https://cdn-icons-png.flaticon.com/512/174/174857.png" width="30" alt="LinkedIn" style="margin-right: 5px;">
    </a>
    <a href="https://x.com/0xKoiner" target="_blank">
        <img src="https://cdn-icons-png.flaticon.com/512/733/733579.png" width="30" alt="Twitter">
    </a>
</p>
