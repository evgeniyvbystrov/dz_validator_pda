# Testing

The project contains a comprehensive testing system:

## Running all tests
```bash
cargo test
```

## Running only unit tests
```bash
cargo test --lib
```

## Running integration tests
```bash
cargo test --test integration_tests
```

## Running additional unit tests
```bash
cargo test --test unit_tests
```

## Running tests with verbose output
```bash
cargo test -- --nocapture
```

## Test Types

### 1. Unit tests (in src/main.rs)
- `test_generate_deposit_pda` - Testing PDA generation
- `test_generate_deposit_pda_different_validators` - Checking PDA uniqueness for different validators
- `test_parse_pubkey_valid` - Testing parsing of valid addresses
- `test_parse_pubkey_invalid` - Testing handling of invalid addresses
- `test_parse_pubkey_empty` - Testing handling of empty strings
- `test_revenue_distribution_program_id` - Checking program constant
- `test_deposit_pda_seed` - Checking PDA seed structure

### 2. Integration tests (tests/integration_tests.rs)
- `test_cli_with_valid_validator_id` - Testing CLI with valid address (pda-address operation)
- `test_cli_with_invalid_validator_id` - Testing CLI with invalid address
- `test_cli_with_empty_input` - Testing CLI with empty input
- `test_cli_with_whitespace_only_input` - Testing CLI with whitespace-only input
- `test_cli_without_arguments` - Testing CLI without arguments
- `test_cli_with_multiple_arguments` - Testing CLI with multiple arguments
- `test_cli_deterministic_output` - Checking output determinism

### 3. Additional unit tests (tests/unit_tests.rs)
- `test_generate_deposit_pda_edge_cases` - Testing edge cases
- `test_parse_pubkey_edge_cases` - Testing parsing edge cases
- `test_deposit_pda_uniqueness` - Checking PDA uniqueness
- `test_program_id_consistency` - Checking program ID consistency
- `test_deposit_pda_seed_structure` - Detailed seed structure verification
- `test_parse_pubkey_whitespace_handling` - Testing whitespace handling
- `test_deposit_pda_deterministic_across_runs` - Checking determinism
- `test_deposit_pda_with_different_program_ids` - Testing with different program IDs
- `test_get_account_balance_with_custom_rpc` - Testing balance retrieval with custom RPC

## Test Coverage

The project has high test coverage:
- ✅ All public functions are covered by unit tests
- ✅ CLI interface is covered by integration tests
- ✅ Edge cases and error handling are tested
- ✅ PDA determinism and uniqueness are verified

## Development

### Adding new tests

1. **Unit tests** - add to the `tests` module in `src/main.rs`
2. **Integration tests** - add to `tests/integration_tests.rs`
3. **Additional unit tests** - add to `tests/unit_tests.rs`

### Running tests in development mode

```bash
# Quick test run
cargo test --lib

# Run with println! output
cargo test -- --nocapture

# Run specific test
cargo test test_generate_deposit_pda
```
