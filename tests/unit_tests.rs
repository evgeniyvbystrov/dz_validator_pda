use dz_validator_pda::{generate_deposit_pda, parse_pubkey, REVENUE_DISTRIBUTION_PROGRAM_ID};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Дополнительные unit тесты для более детального тестирования
#[cfg(test)]
mod additional_unit_tests {
    use super::*;

    #[test]
    fn test_generate_deposit_pda_edge_cases() {
        // Тест с нулевым ключом
        let zero_pubkey = Pubkey::default();
        let deposit_pda = generate_deposit_pda(&zero_pubkey);
        assert_ne!(deposit_pda, Pubkey::default());
        
        // Тест с максимальным ключом
        let max_pubkey = Pubkey::new_from_array([255u8; 32]);
        let deposit_pda_max = generate_deposit_pda(&max_pubkey);
        assert_ne!(deposit_pda_max, Pubkey::default());
        assert_ne!(deposit_pda, deposit_pda_max);
    }

    #[test]
    fn test_parse_pubkey_edge_cases() {
        // Тест с очень длинной строкой
        let long_string = "a".repeat(1000);
        let result = parse_pubkey(&long_string);
        assert!(result.is_err());
        
        // Тест с строкой из спецсимволов
        let special_chars = "!@#$%^&*()";
        let result = parse_pubkey(special_chars);
        assert!(result.is_err());
        
        // Тест с валидным адресом
        let valid_address = "FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL";
        let result = parse_pubkey(valid_address);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deposit_pda_uniqueness() {
        // Генерируем несколько разных валидаторов и проверяем уникальность PDA
        let validator1 = Pubkey::from_str("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .expect("Failed to parse validator1");
        let validator2 = Pubkey::from_str("11111111111111111111111111111112")
            .expect("Failed to parse validator2");
        
        let pda1 = generate_deposit_pda(&validator1);
        let pda2 = generate_deposit_pda(&validator2);
        
        // PDA должны быть разными для разных валидаторов
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn test_program_id_consistency() {
        // Проверяем, что константа программы не изменилась
        let expected = "dzrevZC94tBLwuHw1dyynZxaXTWyp7yocsinyEVPtt4";
        assert_eq!(REVENUE_DISTRIBUTION_PROGRAM_ID.to_string(), expected);
        
        // Проверяем, что это валидный Pubkey
        let parsed = expected.parse::<Pubkey>();
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), REVENUE_DISTRIBUTION_PROGRAM_ID);
    }

    #[test]
    fn test_deposit_pda_seed_structure() {
        let validator_id = Pubkey::from_str("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .expect("Failed to parse test validator ID");
        
        // Проверяем, что PDA создается с правильными сидами
        let (pda, bump_seed) = Pubkey::find_program_address(
            &[b"solana_validator_deposit", validator_id.as_ref()],
            &REVENUE_DISTRIBUTION_PROGRAM_ID
        );
        
        let generated_pda = generate_deposit_pda(&validator_id);
        
        assert_eq!(generated_pda, pda);
        assert!(bump_seed > 0 && bump_seed <= 255, "Bump seed should be in valid range");
        
        // Проверяем, что сид действительно "solana_validator_deposit"
        let expected_seed = b"solana_validator_deposit";
        assert_eq!(expected_seed, b"solana_validator_deposit");
    }

    #[test]
    fn test_parse_pubkey_whitespace_handling() {
        // Тест с пробелами в начале и конце
        let address_with_spaces = "  FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL  ";
        let result = parse_pubkey(address_with_spaces);
        
        // Solana SDK обычно не принимает пробелы, поэтому это должно быть ошибкой
        assert!(result.is_err());
        
        // Тест с табуляцией
        let address_with_tabs = "\tFjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL\t";
        let result = parse_pubkey(address_with_tabs);
        assert!(result.is_err());
    }

    #[test]
    fn test_deposit_pda_deterministic_across_runs() {
        let validator_id = Pubkey::from_str("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .expect("Failed to parse test validator ID");
        
        // Генерируем PDA несколько раз
        let pda1 = generate_deposit_pda(&validator_id);
        let pda2 = generate_deposit_pda(&validator_id);
        let pda3 = generate_deposit_pda(&validator_id);
        
        // Все должны быть одинаковыми
        assert_eq!(pda1, pda2);
        assert_eq!(pda2, pda3);
        assert_eq!(pda1, pda3);
    }

    #[test]
    fn test_deposit_pda_with_different_program_ids() {
        let validator_id = Pubkey::from_str("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .expect("Failed to parse test validator ID");
        
        // Создаем другой program ID
        let different_program_id = Pubkey::from_str("11111111111111111111111111111112")
            .expect("Failed to parse different program ID");
        
        // Генерируем PDA с нашим program ID
        let our_pda = generate_deposit_pda(&validator_id);
        
        // Генерируем PDA с другим program ID
        let (other_pda, _) = Pubkey::find_program_address(
            &[b"solana_validator_deposit", validator_id.as_ref()],
            &different_program_id
        );
        
        // PDA должны быть разными
        assert_ne!(our_pda, other_pda);
    }
}
