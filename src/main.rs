use solana_sdk::pubkey::Pubkey;
use solana_client::nonblocking::rpc_client::RpcClient;
use std::env;

pub const REVENUE_DISTRIBUTION_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("dzrevZC94tBLwuHw1dyynZxaXTWyp7yocsinyEVPtt4");

/// Generates a Program Derived Address (PDA) for validator deposit
/// 
/// # Arguments
/// * `validator_id` - The validator's public key
/// 
/// # Returns
/// * `Pubkey` - The generated PDA for the deposit
pub fn generate_deposit_pda(validator_id: &Pubkey) -> Pubkey {
    let (deposit_key, _) = Pubkey::find_program_address(
        &[b"solana_validator_deposit", validator_id.as_ref()],
        &REVENUE_DISTRIBUTION_PROGRAM_ID
    );
    deposit_key
}

/// Parses a string into a Pubkey
/// 
/// # Arguments
/// * `address_str` - String containing the address
/// 
/// # Returns
/// * `Result<Pubkey, String>` - Parsing result
pub fn parse_pubkey(address_str: &str) -> Result<Pubkey, String> {
    address_str.parse::<Pubkey>()
        .map_err(|e| format!("Invalid pubkey format: {}", e))
}

/// Gets the balance of a given account
/// 
/// # Arguments
/// * `address` - The account address to check balance for
/// * `rpc_url` - The RPC endpoint URL (optional, defaults to mainnet)
/// 
/// # Returns
/// * `Result<u64, String>` - Balance in lamports or error message
pub async fn get_account_balance(address: &Pubkey, rpc_url: Option<&str>) -> Result<u64, String> {
    let url = rpc_url.unwrap_or("https://api.mainnet-beta.solana.com");
    let client = RpcClient::new(url.to_string());
    
    client.get_balance(address).await
        .map_err(|e| format!("Failed to get balance: {}", e))
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Error: Please provide operation name and validator address as parameters");
        eprintln!("Usage: {} <operation> <validator_address>", args[0]);
        eprintln!("Operations:");
        eprintln!("  pda-address  - Generate PDA address for validator");
        eprintln!("  pda-balance  - Show balance of PDA address for validator");
        eprintln!("Example: {} pda-address FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL", args[0]);
        eprintln!("Example: {} pda-balance FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL", args[0]);
        std::process::exit(1);
    }
    
    let operation = args[1].as_str();
    let address = args[2].as_str();
    
    // Проверка на заполненность параметров
    if operation.trim().is_empty() {
        eprintln!("Error: Operation parameter cannot be empty");
        std::process::exit(1);
    }
    
    if address.trim().is_empty() {
        eprintln!("Error: Validator address parameter cannot be empty");
        std::process::exit(1);
    }
    
    // Проверка операции
    if operation != "pda-address" && operation != "pda-balance" {
        eprintln!("Error: Unknown operation '{}'. Supported operations: pda-address, pda-balance", operation);
        std::process::exit(1);
    }
    
    match parse_pubkey(address) {
        Ok(validator_id) => {
            let deposit_key = generate_deposit_pda(&validator_id);
            
            if operation == "pda-address" {
                println!("Validator pubkey {}", address);
                println!("PDA Address: {}", deposit_key);
            } else if operation == "pda-balance" {
                match get_account_balance(&deposit_key, None).await {
                    Ok(balance) => {
                        let sol_balance = balance as f64 / 1_000_000_000.0; // Convert lamports to SOL
                        println!("Validator pubkey {}", address);
                        println!("PDA Address: {}", deposit_key);
                        println!("PDA Balance: {} lamports ({} SOL)", balance, sol_balance);
                    }
                    Err(e) => {
                        eprintln!("Error getting balance: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_generate_deposit_pda() {
        // Test validator ID
        let validator_id = Pubkey::from_str("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .expect("Failed to parse test validator ID");
        
        let deposit_pda = generate_deposit_pda(&validator_id);
        
        // Check that PDA is not equal to the default key
        assert_ne!(deposit_pda, Pubkey::default());
        
        // Check that PDA is deterministic (same result for same input)
        let deposit_pda2 = generate_deposit_pda(&validator_id);
        assert_eq!(deposit_pda, deposit_pda2);
    }

    #[test]
    fn test_generate_deposit_pda_different_validators() {
        let validator1 = Pubkey::from_str("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .expect("Failed to parse validator1");
        let validator2 = Pubkey::from_str("11111111111111111111111111111112")
            .expect("Failed to parse validator2");
        
        let deposit_pda1 = generate_deposit_pda(&validator1);
        let deposit_pda2 = generate_deposit_pda(&validator2);
        
        // Different validators should generate different PDAs
        assert_ne!(deposit_pda1, deposit_pda2);
    }

    #[test]
    fn test_parse_pubkey_valid() {
        let valid_address = "FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL";
        let result = parse_pubkey(valid_address);
        
        assert!(result.is_ok());
        let pubkey = result.unwrap();
        assert_eq!(pubkey.to_string(), valid_address);
    }

    #[test]
    fn test_parse_pubkey_invalid() {
        let invalid_address = "invalid_address";
        let result = parse_pubkey(invalid_address);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Invalid pubkey format"));
    }

    #[test]
    fn test_parse_pubkey_empty() {
        let empty_address = "";
        let result = parse_pubkey(empty_address);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_pubkey_whitespace_only() {
        let whitespace_address = "   ";
        let result = parse_pubkey(whitespace_address);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_revenue_distribution_program_id() {
        // Check that the program constant is correctly defined
        let expected_program_id = "dzrevZC94tBLwuHw1dyynZxaXTWyp7yocsinyEVPtt4";
        assert_eq!(REVENUE_DISTRIBUTION_PROGRAM_ID.to_string(), expected_program_id);
    }

    #[test]
    fn test_deposit_pda_seed() {
        let validator_id = Pubkey::from_str("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .expect("Failed to parse test validator ID");
        
        let deposit_pda = generate_deposit_pda(&validator_id);
        
        // Check that PDA is actually created with correct seeds
        let (expected_pda, bump_seed) = Pubkey::find_program_address(
            &[b"solana_validator_deposit", validator_id.as_ref()],
            &REVENUE_DISTRIBUTION_PROGRAM_ID
        );
        
        assert_eq!(deposit_pda, expected_pda);
        assert!(bump_seed > 0); // bump seed should be greater than 0
    }


    #[tokio::test]
    async fn test_get_account_balance_with_custom_rpc() {
        let test_address = Pubkey::from_str("11111111111111111111111111111112")
            .expect("Failed to parse test address");
        
        // Test with a custom RPC URL (this might fail if the URL is invalid, but we're testing the function)
        let result = get_account_balance(&test_address, Some("https://api.mainnet-beta.solana.com")).await;
        assert!(result.is_ok());
    }
}
