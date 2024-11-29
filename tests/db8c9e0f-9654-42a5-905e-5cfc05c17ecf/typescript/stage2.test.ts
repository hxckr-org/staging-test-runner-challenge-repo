//@ts-nocheck

import { expect, test, describe } from "bun:test";
import { createTransaction } from "./main";

describe("Transaction Builder", () => {
  // Test valid transaction creation
  test("creates a valid transaction with sufficient funds", () => {
    const utxos = [
      {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9",
        vout: 0,
        value: 50000,
      },
    ];
    const targetAddress = "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF";
    const amount = 30000;
    const privateKey =
      "a1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    const result = createTransaction(utxos, targetAddress, amount, privateKey);
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
    expect(result).toMatch(/^[0-9a-f]+$/); // Should be hexadecimal
    expect(result.length).toBeGreaterThan(0);
  });

  // Test insufficient funds
  test("throws error with insufficient funds", () => {
    const utxos = [
      {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9",
        vout: 0,
        value: 500,
      },
    ];
    const targetAddress = "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF";
    const amount = 1000;
    const privateKey =
      "a1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    expect(() =>
      createTransaction(utxos, targetAddress, amount, privateKey),
    ).toThrow("Insufficient funds");
  });

  // Test input validation
  test("validates all required parameters", () => {
    const utxos = [
      {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9",
        vout: 0,
        value: 50000,
      },
    ];
    const targetAddress = "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF";
    const amount = 30000;
    const privateKey =
      "a1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    // Test missing private key
    expect(() => createTransaction(utxos, targetAddress, amount, "")).toThrow(
      "Private key is missing",
    );

    // Test missing UTXOs
    expect(() =>
      createTransaction([], targetAddress, amount, privateKey),
    ).toThrow("No UTXOs provided");

    // Test missing target address
    expect(() => createTransaction(utxos, "", amount, privateKey)).toThrow(
      "Target address is missing",
    );

    // Test invalid amount
    expect(() =>
      createTransaction(utxos, targetAddress, 0, privateKey),
    ).toThrow("Invalid amount");
    expect(() =>
      createTransaction(utxos, targetAddress, -1000, privateKey),
    ).toThrow("Invalid amount");
  });

  // Test multiple UTXOs handling
  test("handles multiple UTXOs correctly", () => {
    const utxos = [
      {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9",
        vout: 0,
        value: 30000,
      },
      {
        txid: "8ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9",
        vout: 1,
        value: 20000,
      },
    ];
    const targetAddress = "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF";
    const amount = 45000;
    const privateKey =
      "a1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    const result = createTransaction(utxos, targetAddress, amount, privateKey);
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
    expect(result).toMatch(/^[0-9a-f]+$/);
  });

  // Test change output creation
  test("creates correct change output", () => {
    const utxos = [
      {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9",
        vout: 0,
        value: 50000,
      },
    ];
    const targetAddress = "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF";
    const amount = 30000;
    const privateKey =
      "a1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    const result = createTransaction(utxos, targetAddress, amount, privateKey);
    expect(result && result.length).toBeGreaterThan(200); // Approximate length check
  });

  // Test transaction format compliance
  test("creates transaction with correct format", () => {
    const utxos = [
      {
        txid: "7ea75da574ebff364f0f4cc9d0315b7d9523f7f38558918aff8570842cba74c9",
        vout: 0,
        value: 50000,
      },
    ];
    const targetAddress = "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF";
    const amount = 30000;
    const privateKey =
      "a1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    const result = createTransaction(utxos, targetAddress, amount, privateKey);

    // Version should be first 8 characters (4 bytes)
    expect(result && result.slice(0, 8)).toBe("01000000");

    // Transaction should end with locktime (4 bytes)
    expect(result && result.slice(-8)).toBe("00000000");
  });

  // Test invalid UTXO format
  test("handles invalid UTXO format", () => {
    // @ts-expect-error - intentionally creating invalid UTXO for testing
    const invalidUtxos: UTXO[] = [
      {
        vout: 0,
        value: 50000,
      },
    ];

    const targetAddress = "2N8hwP1WmJrFF5QWABn38y63uYLhnJYJYTF";
    const amount = 30000;
    const privateKey =
      "a1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    expect(() =>
      createTransaction(invalidUtxos, targetAddress, amount, privateKey),
    ).toThrow();
  });
});