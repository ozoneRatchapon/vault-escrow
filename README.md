# Solana Vault Program

This is a Solana program built using the Anchor framework. It allows users to create a vault, deposit funds, withdraw funds, and close the vault.

## Features
- **Initialize Vault**: Create a new vault and its state.
- **Deposit Funds**: Deposit funds into the vault.
- **Withdraw Funds**: Withdraw funds from the vault.
- **Close Vault**: Close the vault and reclaim rent.
- **Escrow**: Create an escrow account for token swaps.

## How It Works
The program uses **Program Derived Addresses (PDAs)** to create and manage vault accounts. It also uses **Cross-Program Invocation (CPI)** to interact with the Solana System Program for transferring funds.

### Accounts
- **`vault_state`**: Stores the vault's state (bump seeds).
- **`vault`**: Holds the funds in the vault.
- **`escrow`**: Stores the escrow's state (bump seeds).
- **`vault`**: Holds the funds in the escrow.

### Instructions
1. **Initialize**: Creates a new vault and its state.
2. **Deposit**: Transfers funds from the user to the vault.
3. **Withdraw**: Transfers funds from the vault to the user.
4. **Close**: Closes the vault and reclaims rent.
5. **Make**: Creates a new escrow account and deposits funds.
6. **Take**: Completes the escrow by swapping tokens.
7. **Refund**: Refunds the tokens to the maker if the escrow is not completed.
