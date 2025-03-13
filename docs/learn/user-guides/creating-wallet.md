# Creating a Selendra Wallet

This guide will walk you through the process of creating a wallet to interact with the Selendra network. A wallet is essential for holding SEL tokens, participating in staking, and interacting with dApps built on Selendra.

## Wallet Options

Selendra supports multiple wallet types:

1. **Web Wallets**: Browser-based interfaces
2. **Mobile Wallets**: Applications for smartphones
3. **Browser Extensions**: Add-ons for web browsers
4. **Hardware Wallets**: Physical devices for maximum security

## Option 1: Selendra Web Wallet (Recommended for Beginners)

The official Selendra web wallet provides a user-friendly interface for all basic operations.

### Steps to Create a Web Wallet

1. **Visit the Selendra Wallet Website**
   - Go to [https://wallet.selendra.org](https://wallet.selendra.org)

2. **Create a New Account**
   - Click on "Create Account"
   - A seed phrase will be generated (12-24 random words)
   - **IMPORTANT**: Write down this seed phrase and store it securely offline. Never share it with anyone!

3. **Set a Wallet Name and Password**
   - Choose a memorable name for your wallet
   - Create a strong password (this protects access to your wallet, but is not your recovery phrase)

4. **Verify Your Seed Phrase**
   - You'll be asked to verify your seed phrase by entering specific words
   - This ensures you've properly saved your seed phrase

5. **Backup Your Account**
   - Download your account backup file and store it securely
   - This provides an additional recovery method

## Option 2: Polkadot.js Browser Extension

The Polkadot.js extension is ideal for users who frequently interact with various dApps.

### Steps to Create a Wallet with Polkadot.js Extension

1. **Install the Extension**
   - Visit [Chrome Web Store](https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd) (for Chrome/Brave) or [Firefox Add-ons](https://addons.mozilla.org/en-US/firefox/addon/polkadot-js-extension/) (for Firefox)
   - Click "Add to Browser" and follow installation prompts

2. **Create a New Account**
   - Click on the extension icon in your browser
   - Select the "+" button and then "Create new account"
   - Save the generated seed phrase securely

3. **Set Account Details**
   - Enter a descriptive name for your account
   - Create a strong password

4. **Configure Network**
   - In the extension settings, under "Manage Networks"
   - Add Selendra network with the following details:
     - Network Name: Selendra
     - RPC URL: https://mainnet.selendra.org
     - Chain ID: 1994 (for EVM compatibility)

## Option 3: Mobile Wallets

### Selendra Mobile Wallet

1. **Download the App**
   - Visit [App Store](https://apps.apple.com/app/selendrawallet) (iOS) or [Google Play](https://play.google.com/store/apps/details?id=org.selendra.wallet) (Android)
   - Install the Selendra Wallet app

2. **Create a New Wallet**
   - Open the app and select "Create New Wallet"
   - Follow the prompts to generate and secure your seed phrase
   - Set a wallet name and PIN code

### Polkadot.js Mobile

Another option is the Polkadot.js mobile app:

1. **Download the App**
   - Visit [App Store](https://apps.apple.com/us/app/polkadot-js/id1633050386) (iOS) or [Google Play](https://play.google.com/store/apps/details?id=com.polkadot.javascript) (Android)

2. **Create a New Account**
   - Open the app and follow the account creation process
   - Save your seed phrase and set a password
   - Add Selendra network in the settings

## Option 4: Hardware Wallet (Most Secure)

For maximum security, you can use a hardware wallet like Ledger.

### Using Ledger with Selendra

1. **Set Up Your Ledger Device**
   - Initialize your Ledger device following manufacturer instructions
   - Update to the latest firmware

2. **Install Required Apps**
   - Install the Selendra app on your Ledger device via Ledger Live

3. **Connect to Selendra Web Wallet**
   - Visit [https://wallet.selendra.org](https://wallet.selendra.org)
   - Click "Connect Hardware Wallet"
   - Follow the prompts to connect your Ledger

## Wallet Security Best Practices

Regardless of which wallet you choose, follow these security practices:

1. **Never Share Your Seed Phrase**
   - Your seed phrase provides complete access to your funds
   - Never share it with anyone, including support staff
   - Don't store it digitally or take screenshots of it

2. **Use Multiple Backups**
   - Write down your seed phrase on paper and store in a secure location
   - Consider using a metal backup for fire and water resistance
   - Keep backups in different physical locations

3. **Enable Additional Security Features**
   - Use biometric authentication if available
   - Enable 2FA where supported
   - Consider a dedicated device for high-value accounts

4. **Regular Updates**
   - Keep your wallet software updated to the latest version
   - Update your device's operating system regularly

5. **Use Multiple Wallets**
   - Consider a separate "hot wallet" with small amounts for daily use
   - Keep larger holdings in more secure wallets

## Account Address Formats

Selendra uses different address formats for different purposes:

1. **Substrate Format**: Begins with characters like "5" (e.g., 5FHneW46xGXgs...)
   - Used for native Substrate functionality like staking
   - Derived from your public key with SS58 encoding

2. **EVM Format**: Begins with "0x" (e.g., 0x71C7656EC7ab88b098defB751B7401B5f6d8976F)
   - Used for EVM compatibility and smart contracts
   - Same format as Ethereum addresses

Your wallet will display both formats, as they access the same underlying account.

## Next Steps

After creating your wallet:

- [Fund your wallet](./making-transfers.md) with SEL tokens
- [Set up staking](./staking-basics.md) to earn rewards
- [Explore dApps](https://selendra.org/ecosystem) built on Selendra 