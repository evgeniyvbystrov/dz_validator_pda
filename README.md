# DZ Validator PDA - Solana Revenue Distribution

This project contains a utility for generating Program Derived Address (PDA) for validator deposits for DoubleZero payments

## Functionality

- Generate PDA for validator deposit
- Parse and validate Solana addresses
- Check PDA account balance from Solana network
- CLI interface with two operations: `pda-address` and `pda-balance`

## Project Structure

```
dz_validator_pda/
├── src/
│   └── main.rs          # Main application code
├── tests/
│   ├── integration_tests.rs  # Integration tests
│   └── unit_tests.rs         # Additional unit tests
├── Cargo.toml           # Project configuration
└── README.md           # This file
```

## Running

### Building the project
```bash
cargo build
```

### Running with operations
The application supports two operations:

#### 1. Generate PDA Address
```bash
cargo run -- pda-address <validator_address>
```

Example:
```bash
cargo run -- pda-address FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL
```

#### 2. Check PDA Balance
```bash
cargo run -- pda-balance <validator_address>
```

Example:
```bash
cargo run -- pda-balance FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL
```

### Command Structure
The application requires exactly 2 arguments:
1. **Operation**: Either `pda-address` or `pda-balance`
2. **Validator Address**: A valid Solana public key

### Error Handling
- Invalid validator addresses will result in an error
- Empty or whitespace-only parameters will be rejected
- Unknown operations will be rejected
- Missing arguments will show usage information

## Testing

For comprehensive testing information, see [TESTING.md](TESTING.md).

## Dependencies

- `solana-sdk = "3.0.0"` - Solana SDK for working with addresses and PDA
- `solana-client = "3.0.2"` - Solana client for RPC operations
- `tokio = "1.0"` - Async runtime for network operations

## Constants

- `REVENUE_DISTRIBUTION_PROGRAM_ID` - Revenue distribution program ID: `dzrevZC94tBLwuHw1dyynZxaXTWyp7yocsinyEVPtt4`

## API

### Functions

#### `generate_deposit_pda(validator_id: &Pubkey) -> Pubkey`
Generates a Program Derived Address for validator deposit.

**Parameters:**
- `validator_id` - Validator's public key

**Returns:**
- `Pubkey` - Generated PDA for deposit

#### `parse_pubkey(address_str: &str) -> Result<Pubkey, String>`
Parses a string into Pubkey with error handling.

**Parameters:**
- `address_str` - Address string

**Returns:**
- `Result<Pubkey, String>` - Parsing result

#### `get_account_balance(address: &Pubkey, rpc_url: Option<&str>) -> Result<u64, String>`
Gets the balance of a given account from the Solana network.

**Parameters:**
- `address` - The account address to check balance for
- `rpc_url` - The RPC endpoint URL (optional, defaults to mainnet)

**Returns:**
- `Result<u64, String>` - Balance in lamports or error message

## License

This project is distributed under the MIT license.
