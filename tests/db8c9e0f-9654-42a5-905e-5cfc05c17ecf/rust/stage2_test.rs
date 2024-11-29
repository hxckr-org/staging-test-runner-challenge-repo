use tx_builder_rs::{create_transaction, UTXO};

#[test]
fn test_valid_transaction_with_sufficient_funds() {
    let utxos = vec![UTXO {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9".to_string(),
        vout: 0,
        value: 50000,
        address: None,
        script_pub_key: None,
    }];

    let private_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let result = create_transaction(
        utxos,
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        30000,
        private_key,
    );

    assert!(
        result.is_ok(),
        "Transaction creation failed: {:?}",
        result.err()
    );
    let tx_hex = result.unwrap();
    assert!(!tx_hex.is_empty());
    assert!(tx_hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_insufficient_funds() {
    let utxos = vec![UTXO {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9".to_string(),
        vout: 0,
        value: 500,
        address: None,
        script_pub_key: None,
    }];

    let private_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let result = create_transaction(
        utxos,
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        1000,
        private_key,
    );

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Insufficient funds");
}

#[test]
fn test_parameter_validation() {
    let utxos = vec![UTXO {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9".to_string(),
        vout: 0,
        value: 50000,
        address: None,
        script_pub_key: None,
    }];

    let private_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    // Test missing private key
    let result = create_transaction(
        utxos.clone(),
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        30000,
        "",
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Private key is missing");

    // Test missing UTXOs
    let result = create_transaction(
        vec![],
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        30000,
        private_key,
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "No UTXOs provided");

    // Test missing target address
    let result = create_transaction(utxos.clone(), "", 30000, private_key);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Target address is missing");

    // Test invalid amount
    let result = create_transaction(
        utxos.clone(),
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        0,
        private_key,
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Invalid amount");
}

#[test]
fn test_multiple_utxos_handling() {
    let utxos = vec![
        UTXO {
            txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9".to_string(),
            vout: 0,
            value: 30000,
            address: None,
            script_pub_key: None,
        },
        UTXO {
            txid: "8ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9".to_string(),
            vout: 1,
            value: 20000,
            address: None,
            script_pub_key: None,
        },
    ];

    let private_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let result = create_transaction(
        utxos,
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        45000,
        private_key,
    );

    assert!(
        result.is_ok(),
        "Transaction creation failed: {:?}",
        result.err()
    );
    let tx_hex = result.unwrap();
    assert!(!tx_hex.is_empty());
    assert!(tx_hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_change_output_creation() {
    let utxos = vec![UTXO {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9".to_string(),
        vout: 0,
        value: 50000,
        address: None,
        script_pub_key: None,
    }];

    let private_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let result = create_transaction(
        utxos,
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        30000,
        private_key,
    );

    assert!(result.is_ok());
    let tx_hex = result.unwrap();
    assert!(tx_hex.len() > 200); // Approximate length check
}

#[test]
fn test_transaction_format() {
    let utxos = vec![UTXO {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9".to_string(),
        vout: 0,
        value: 50000,
        address: None,
        script_pub_key: None,
    }];

    let private_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let result = create_transaction(
        utxos,
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        30000,
        private_key,
    );

    assert!(result.is_ok());
    let tx_hex = result.unwrap();

    // Version should be first 8 characters (4 bytes)
    assert_eq!(&tx_hex[0..8], "01000000");

    // Transaction should end with locktime (4 bytes)
    assert_eq!(&tx_hex[tx_hex.len() - 8..], "00000000");
}

#[test]
fn test_invalid_utxo_format() {
    let invalid_utxos = vec![UTXO {
        txid: String::new(), // Invalid empty txid
        vout: 0,
        value: 50000,
        address: None,
        script_pub_key: None,
    }];

    let private_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let result = create_transaction(
        invalid_utxos,
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        30000,
        private_key,
    );

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Invalid UTXO: empty txid");
}

// Add more invalid UTXO test cases
#[test]
fn test_invalid_utxo_txid_format() {
    let invalid_utxos = vec![UTXO {
        txid: "not_a_hex_string".to_string(),
        vout: 0,
        value: 50000,
        address: None,
        script_pub_key: None,
    }];

    let private_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let result = create_transaction(
        invalid_utxos,
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        30000,
        private_key,
    );

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid UTXO: txid must be hexadecimal"
    );
}

#[test]
fn test_invalid_utxo_txid_length() {
    let invalid_utxos = vec![UTXO {
        txid: "abcd".to_string(), // Too short
        vout: 0,
        value: 50000,
        address: None,
        script_pub_key: None,
    }];

    let private_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let result = create_transaction(
        invalid_utxos,
        "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF",
        30000,
        private_key,
    );

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid UTXO: txid length must be 64, got 4"
    );
}
