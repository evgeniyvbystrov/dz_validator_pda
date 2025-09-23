# DZ Validator PDA - Solana Revenue Distribution

A comprehensive utility for managing Program Derived Addresses (PDAs) for validator deposits in the DoubleZero revenue distribution system. This tool provides secure and efficient management of validator deposit accounts on the Solana blockchain.

## Features

- **PDA Generation**: Generate deterministic PDAs for validator deposits
- **Balance Checking**: Query PDA account balances from the Solana network
- **Funding Operations**: Transfer SOL to validator PDAs from keypairs
- **Address Validation**: Validate base58 encoded Solana addresses
- **Gossip Network Validation**: Verify validator presence in Solana gossip network with automatic funding cancellation for inactive validators
- **CLI Interface**: Three main operations: `pda-address`, `pda-balance`, and `pda-fund-address`
- **Error Handling**: Comprehensive error handling with detailed messages
- **Network Support**: Works with Solana mainnet
- **Safety Features**: Automatic validation checks to prevent funding inactive validators

## Installation

### Prerequisites
- Rust 1.70+ (recommended: latest stable)
- Solana CLI tools (optional, for keypair management)
- Internet connection for RPC calls
- Valid Solana keypair file for funding operations

### Building from Source
```bash
# Clone the repository
git clone <repository-url>
cd dz_validator_pda

# Build the project
cargo build --release

# The executable will be available at target/release/dz_validator_pda
```

### Development Build
```bash
# For development and testing
cargo build
```

## Project Structure

```
dz_validator_pda/
├── src/
│   └── main.rs              # Main application code with CLI interface
├── tests/
│   ├── integration_tests.rs # Integration tests for network operations
│   └── unit_tests.rs        # Unit tests for core functions
├── Cargo.toml              # Project configuration and dependencies
├── README.md               # This documentation
├── API.md                  # Detailed API documentation
└── TESTING.md              # Testing guidelines and procedures
```

## Usage

The application provides three main operations for managing validator PDAs:

### 1. Generate PDA Address
Generate a Program Derived Address for a specific validator.

```bash
cargo run -- pda-address <validator_address>
```

**Example:**
```bash
cargo run -- pda-address FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL
```

**Expected Output:**
```
Checking if validator is in Solana gossip network...
✓ Validator FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL is present in Solana gossip network
Validator pubkey FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL
PDA Address: [generated_pda_address]
```

**Use Cases:**
- Setting up validator deposit accounts
- Verifying PDA generation for specific validators
- Integration with other Solana applications

### 2. Check PDA Balance
Query the current balance of a validator's PDA account.

```bash
cargo run -- pda-balance <validator_address>
```

**Example:**
```bash
cargo run -- pda-balance FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL
```

**Expected Output:**
```
Checking if validator is in Solana gossip network...
✓ Validator FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL is present in Solana gossip network
Validator pubkey FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL
PDA Address: [generated_pda_address]
PDA Balance: 0 lamports (0.0 SOL)
```

**Use Cases:**
- Monitoring validator deposit balances
- Verifying funding transactions
- Account reconciliation

### 3. Fund PDA Address
Transfer SOL from a keypair to a validator's PDA account.

```bash
cargo run -- pda-fund-address <validator_address> <keypair_path> <amount_sol>
```

**Example:**
```bash
cargo run -- pda-fund-address FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL /path/to/keypair.json 1.5
```

**Expected Output:**
```
Validator pubkey: FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL
PDA Address: [generated_pda_address]
Funding PDA with 1.5 SOL (1500000000 lamports) from keypair: /path/to/keypair.json
Checking validator gossip status before funding...
✓ Validator FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL is present in Solana gossip network - proceeding with funding
Transaction successful!
Transaction signature: [transaction_signature]
Transferred 1.5 SOL (1500000000 lamports) to PDA
```

**Parameters:**
- `validator_address`: Valid Solana public key of the validator
- `keypair_path`: Path to JSON keypair file with sufficient SOL balance
- `amount_sol`: Amount to transfer in SOL (e.g., 1.5 for 1.5 SOL)

**Use Cases:**
- Initial funding of validator deposit accounts
- Regular deposit operations
- Automated funding workflows

## Command Reference

### Operation Parameters

**Basic Operations (`pda-address`, `pda-balance`):**
- `operation`: Operation type (`pda-address` or `pda-balance`)
- `validator_address`: Valid Solana public key

**Funding Operation (`pda-fund-address`):**
- `operation`: `pda-fund-address`
- `validator_address`: Valid Solana public key
- `keypair_path`: Path to JSON keypair file
- `amount_sol`: Amount in SOL (e.g., 1.5 for 1.5 SOL)

### Error Handling

The application provides comprehensive error handling for various scenarios:

**Input Validation:**
- ❌ Invalid validator addresses (malformed base58)
- ❌ Empty or whitespace-only parameters
- ❌ Unknown operations
- ❌ Missing required arguments

**Network Operations:**
- ❌ Invalid keypair files or corrupted JSON
- ❌ Invalid amount values (non-numeric, negative)
- ❌ Insufficient balance in source keypair
- ⚠️ Validators not found in gossip network (warning, continues operation)
- ❌ Network connectivity issues
- ❌ Transaction failures

**Example Error Messages:**
```
Error: Invalid validator address format: Invalid base58 character '0' found in address
Error: pda-fund-address requires keypair path and amount parameters
Error: Invalid amount: abc123
Error: Amount must be a valid number (e.g., 1.5 for 1.5 SOL)
Error: Failed to read keypair from /path/to/keypair.json: No such file or directory
Error: Funding cancelled: Validator is not in Solana gossip network
```

## Testing

The project includes comprehensive testing coverage:

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests
cargo test --test integration_tests

# Run with verbose output
cargo test -- --nocapture
```

For detailed testing information, see [TESTING.md](TESTING.md).

## Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| `solana-sdk` | `3.0.0` | Core Solana SDK for addresses, PDAs, and transactions |
| `solana-client` | `3.0.2` | RPC client for network operations |
| `tokio` | `1.0` | Async runtime for network operations |
| `anyhow` | `1.0` | Error handling and result types |
| `bs58` | `0.5` | Base58 encoding/decoding for address validation |
| `solana-system-interface` | `2.0.0` | System program interface for transfer instructions |

### Key Features of Dependencies

- **solana-sdk**: Provides core Solana primitives including `Pubkey`, `Keypair`, and PDA generation
- **solana-client**: Handles RPC communication with Solana network
- **tokio**: Enables async/await for efficient network operations
- **anyhow**: Simplifies error handling with `Result<T, anyhow::Error>`
- **bs58**: Validates and processes base58 encoded addresses

## Advanced Features

### Gossip Network Validation
The application automatically verifies validator presence in the Solana gossip network before operations, ensuring validators are active and properly configured. This feature:

- **Prevents funding inactive validators**: Automatically cancels funding if validator is not found in gossip network
- **Provides warnings**: Shows clear warnings when validators are not active
- **Ensures safety**: Prevents accidental funding of inactive or misconfigured validators
- **Network verification**: Uses Solana cluster nodes API to verify validator status

### Base58 Address Validation
Comprehensive validation of Solana addresses with detailed error messages for invalid formats.

### Enhanced Error Handling
- Detailed error messages for debugging
- Graceful handling of network issues
- Input validation with helpful suggestions

## Configuration

### Program Constants
- **Revenue Distribution Program ID**: `dzrevZC94tBLwuHw1dyynZxaXTWyp7yocsinyEVPtt4`
- **PDA Seed**: `"solana_validator_deposit"`
- **Default RPC**: `https://api.mainnet-beta.solana.com`
- **Rust Edition**: `2024`

### Environment Variables
```bash
# Optional: Custom RPC endpoint
export SOLANA_RPC_URL="https://your-custom-rpc.com"

# Optional: Custom keypair location
export SOLANA_KEYPAIR_PATH="/path/to/default/keypair.json"
```

## API Documentation

For detailed API documentation including function signatures, parameters, and return types, see [API.md](API.md).

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is distributed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Support

For questions, issues, or contributions:
- Create an issue on GitHub
- Review the [API documentation](API.md)
- Check the [testing guidelines](TESTING.md)
