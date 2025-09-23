# API Documentation

## Functions

### `generate_deposit_pda(validator_id: &Pubkey) -> Pubkey`
Generates a Program Derived Address for validator deposit.

**Parameters:**
- `validator_id` - Validator's public key

**Returns:**
- `Pubkey` - Generated PDA for deposit

### `parse_pubkey(address_str: &str) -> Result<Pubkey, String>`
Parses a string into Pubkey with error handling.

**Parameters:**
- `address_str` - Address string

**Returns:**
- `Result<Pubkey, String>` - Parsing result

### `get_account_balance(address: &Pubkey, rpc_url: Option<&str>) -> Result<u64, String>`
Gets the balance of a given account from the Solana network.

**Parameters:**
- `address` - The account address to check balance for
- `rpc_url` - The RPC endpoint URL (optional, defaults to mainnet)

**Returns:**
- `Result<u64, String>` - Balance in lamports or error message
