import pytest
from main import MultisigWallet
from bitcoinutils.setup import setup
from bitcoinutils.transactions import Transaction, TxInput, TxOutput
from bitcoinutils.keys import P2wpkhAddress
import re

# Setup bitcoin-utils for testing
setup('testnet')

class TestSolutionMultisigWallet:
    
    def test_init_validation(self):
        """Test wallet initialization validation"""
        # Solution should work with valid parameters
        solution = MultisigWallet(2, 3)
        assert solution.required_signatures == 2
        assert solution.total_signers == 3
        
        # Solution should raise error for invalid parameters
        with pytest.raises(ValueError):
            MultisigWallet(4, 3)  # required > total
        with pytest.raises(ValueError):
            MultisigWallet(0, 3)  # required <= 0
    
    def test_wallet_generation(self):
        """Test wallet generation process"""
        solution = MultisigWallet(2, 3)
        solution.generate_wallet()
        
        # Check if correct number of keypairs generated
        assert len(solution.key_pairs) == 3
        
        # Check if addresses were generated
        assert solution.addresses is not None
        assert 'p2sh' in solution.addresses
        assert 'p2wsh' in solution.addresses
    
    def test_key_pair_generation(self):
        """Test individual keypair generation"""
        solution = MultisigWallet(2, 3)
        key_pair = solution.generate_key_pair(0)
        
        # Check key pair structure
        assert 'mnemonic' in key_pair
        assert 'path' in key_pair
        assert 'public_key' in key_pair
        assert 'private_key' in key_pair
        
        # Check path format
        assert re.match(r"m/49'/0'/0'/0/\d+", key_pair['path'])
    
    def test_multisig_address_creation(self):
        """Test multisig address creation"""
        solution = MultisigWallet(2, 3)
        solution.generate_wallet()
        solution.create_multisig_addresses()
        
        # Check address formats
        assert isinstance(solution.addresses['p2sh'], str)
        assert isinstance(solution.addresses['p2wsh'], str)
        assert solution.addresses['p2sh'].startswith('2') or solution.addresses['p2sh'].startswith('3')
        assert solution.addresses['p2wsh'].startswith('tb1')
    
    def test_transaction_signing(self):
        """Test transaction signing"""
        solution = MultisigWallet(2, 3)
        solution.generate_wallet()
        
        # Create a dummy transaction with a valid hex txid (non-coinbase)
        tx_input = TxInput("a" * 64, 0)  # Use a different valid 64-character hex string
        
        # Create proper output script
        addr = P2wpkhAddress('tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx')
        script_pubkey = addr.to_script_pub_key()
        tx_output = TxOutput(1000, script_pubkey)
        
        tx = Transaction([tx_input], [tx_output])
        
        # Test signing
        key_pair = solution.key_pairs[0]
        signed_tx = solution.sign_transaction(tx.to_hex(), 0, key_pair)
        
        assert signed_tx is not None
        assert isinstance(signed_tx, str)  # Full transaction returned
    
    def test_config_handling(self):
        """Test configuration handling"""
        config = {
            'derivation_path': "m/84'/0'/0'/0"
        }
        
        # Solution wallet should use config
        solution = MultisigWallet(2, 3, config)
        assert solution.derivation_path == "m/84'/0'/0'/0"
    
    def test_mnemonic_strength(self):
        """Test mnemonic generation strength"""
        solution = MultisigWallet(2, 3)
        key_pair = solution.generate_key_pair(0)
        
        # Count words in mnemonic (24 words for 256 bits)
        word_count = len(key_pair['mnemonic'].split())
        expected_words = 256 // 32 * 3
        
        assert word_count == expected_words 