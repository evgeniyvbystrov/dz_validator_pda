use std::process::Command;
use std::str;

/// Интеграционные тесты для CLI приложения
#[cfg(test)]
mod integration_tests {
    use super::*;

    fn get_binary_path() -> String {
        // В Windows используем .exe расширение
        if cfg!(target_os = "windows") {
            "target/debug/dz_validator_pda.exe".to_string()
        } else {
            "target/debug/dz_validator_pda".to_string()
        }
    }

    #[test]
    fn test_cli_with_valid_validator_id() {
        let output = Command::new(get_binary_path())
            .arg("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Command should succeed");
        
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
        
        // Проверяем, что в stdout есть ожидаемый вывод
        assert!(stdout.contains("Validator pubkey is FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL"));
        assert!(stdout.contains("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL"));
        
        // Проверяем, что в stderr нет ошибок
        assert!(stderr.is_empty(), "Should not have errors in stderr");
        
        // Проверяем, что выводится PDA (должен быть длинной строкой)
        assert!(stdout.contains("deposit PDA is"));
        
        // Находим PDA в выводе
        let lines: Vec<&str> = stdout.lines().collect();
        assert!(lines.len() >= 1, "Should have at least 1 line of output");
        
        // PDA должен быть в той же строке
        let output_line = lines[0];
        assert!(output_line.contains("deposit PDA is"));
    }

    #[test]
    fn test_cli_with_invalid_validator_id() {
        let output = Command::new(get_binary_path())
            .arg("invalid_address")
            .output()
            .expect("Failed to execute command");

        assert!(!output.status.success(), "Command should fail with invalid input");
        
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
        
        // Проверяем, что в stderr есть сообщение об ошибке
        assert!(stderr.contains("Error:"));
        assert!(stderr.contains("Invalid pubkey format"));
    }

    #[test]
    fn test_cli_with_empty_input() {
        let output = Command::new(get_binary_path())
            .arg("")
            .output()
            .expect("Failed to execute command");

        assert!(!output.status.success(), "Command should fail with empty input");
        
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
        
        // Проверяем, что в stderr есть сообщение об ошибке
        assert!(stderr.contains("Error: Validator address parameter cannot be empty"));
    }

    #[test]
    fn test_cli_with_whitespace_only_input() {
        let output = Command::new(get_binary_path())
            .arg("   ")
            .output()
            .expect("Failed to execute command");

        assert!(!output.status.success(), "Command should fail with whitespace-only input");
        
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
        
        // Проверяем, что в stderr есть сообщение об ошибке
        assert!(stderr.contains("Error: Validator address parameter cannot be empty"));
    }

    #[test]
    fn test_cli_without_arguments() {
        let output = Command::new(get_binary_path())
            .output()
            .expect("Failed to execute command");

        // Без аргументов программа должна завершиться с ошибкой
        assert!(!output.status.success(), "Command should fail without arguments");
        
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
        
        // Должно быть сообщение об ошибке
        assert!(stderr.contains("Error: Please provide a validator address as a parameter"));
        assert!(stderr.contains("Usage:"));
    }

    #[test]
    fn test_cli_with_multiple_arguments() {
        let output = Command::new(get_binary_path())
            .arg("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .arg("extra_argument")
            .output()
            .expect("Failed to execute command");

        // Программа должна использовать только первый аргумент
        assert!(output.status.success(), "Command should succeed");
        
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
        
        // Проверяем, что используется только первый аргумент
        assert!(stdout.contains("Validator pubkey is FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL"));
    }

    #[test]
    fn test_cli_deterministic_output() {
        // Запускаем команду дважды с одинаковым входом
        let output1 = Command::new(get_binary_path())
            .arg("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .output()
            .expect("Failed to execute command");

        let output2 = Command::new(get_binary_path())
            .arg("FjYEr2UCeFzNfAKiFrbhG34Zv8LxbmfHYAFhAfc7SLQL")
            .output()
            .expect("Failed to execute command");

        assert!(output1.status.success());
        assert!(output2.status.success());
        
        let stdout1 = str::from_utf8(&output1.stdout).expect("Invalid UTF-8");
        let stdout2 = str::from_utf8(&output2.stdout).expect("Invalid UTF-8");
        
        // Вывод должен быть идентичным
        assert_eq!(stdout1, stdout2, "Output should be deterministic");
    }
}
