import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vaultplex } from "../target/types/vaultplex";
import { assert } from "chai";
import { BN } from "bn.js";
import { randomBytes } from "crypto";
import { Authorized, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { LockExtension } from './extensions';

describe("vaultplex", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const connection = provider.connection;
  const program = anchor.workspace.Vaultplex as Program<Vaultplex>;

  const confirmTx = async (signature: string) => {
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      "confirmed"
    );
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/tx/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  const user = Keypair.generate();

  // Generating a big number random to create a unique seed
  const seed = new BN(randomBytes(8));

  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), seed.toArrayLike(Buffer, "le", 8)],
    program.programId
  )[0];

  it("should get some SOL for testing", async () => {
    let tx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: user.publicKey,
        lamports: 10 * LAMPORTS_PER_SOL,
      })
    );

    await provider.sendAndConfirm(tx).then(log);

    const balance = await connection.getBalance(user.publicKey);
    assert.equal(balance, 10 * LAMPORTS_PER_SOL);
  });

  it("should initialize the vault with balance 0", async () => {
    try {
      const tx = await program.methods
        .initializeVault(seed)
        .accounts({
          authority: user.publicKey,
          vault,
          /* systemProgram: SystemProgram.programId, */
        })
        .signers([user])
        .rpc()
        .then(confirmTx)
        .then(log);

      const vaultAccountData = await program.account.vault.fetch(vault);
      console.log(vaultAccountData);

      assert.equal(vaultAccountData.authority.toString(), user.publicKey.toString());
      assert.equal(vaultAccountData.seed.toString(), seed.toString());
    } catch (e) {
      console.error(e);
      throw e;
    }
  });
 
  it("should initialize the Lock Extension", async () => {
    try {
      const lockAuthority = user.publicKey; // Set lock authority as the user
      const tx = await program.methods
        .initializeLockExtension(lockAuthority)
        .accounts({
          authority: user.publicKey,
          vault,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc()
        .then(confirmTx)
        .then(log);

      // Fetch the updated Vault account data
      const vaultAccountData = await program.account.vault.fetch(vault);
      console.log("Vault Account Data after initializing Lock Extension:", vaultAccountData);

      // Verify that the vault's authority and seed have not changed
      assert.equal(vaultAccountData.authority.toString(), user.publicKey.toString());
      assert.equal(vaultAccountData.seed.toString(), seed.toString());

      // Extract the extensions data from the vault account
      const extensionsData = vaultAccountData.extensions as Buffer;

      // Deserialize the LockExtension data from the extensions buffer
      const lockExtensionSize = 32 + 1; // Pubkey (32 bytes) + is_locked (1 byte)
      const lockExtensionOffset = 0; // Assuming LockExtension is at the start of the extensions vector

      // Extract the LockExtension slice
      const lockExtensionData = extensionsData.slice(lockExtensionOffset, lockExtensionOffset + lockExtensionSize);

      // Deserialize LockExtension data (using Borsh or your chosen method)
      const lockExtension = LockExtension.fromBuffer(lockExtensionData); // You'll need to implement this deserialization method

      // Verify the lock extension contents
      assert.equal(lockExtension.lockAuthority.toString(), lockAuthority.toString());
      assert.equal(lockExtension.isLocked, false); // Should be false after initialization

      console.log("Lock Extension initialized correctly:", lockExtension);

    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("should lock the vault and prevent further deposits", async () => {
    try {
      const lockAuthority = user.publicKey; // Set lock authority as the user
      
      const tx = await program.methods
        .lockVault()
        .accounts({
          authority: user.publicKey,
          lockAuthority: user.publicKey,
          vault,
        })
        .signers([user])
        .rpc()
        .then(confirmTx)
        .then(log);

      const vaultAccountData = await program.account.vault.fetch(vault);
        // Extract the extensions data from the vault account
      const extensionsData = vaultAccountData.extensions as Buffer;

      // Deserialize the LockExtension data from the extensions buffer
      const lockExtensionSize = 32 + 1; // Pubkey (32 bytes) + is_locked (1 byte)
      const lockExtensionOffset = 0; // Assuming LockExtension is at the start of the extensions vector

      // Extract the LockExtension slice
      const lockExtensionData = extensionsData.slice(lockExtensionOffset, lockExtensionOffset + lockExtensionSize);

      // Deserialize LockExtension data (using Borsh or your chosen method)
      const lockExtension = LockExtension.fromBuffer(lockExtensionData); // You'll need to implement this deserialization method

      // Verify the lock extension contents
      assert.equal(lockExtension.lockAuthority.toString(), lockAuthority.toString());
      assert.equal(lockExtension.isLocked, true); // Should be false after initialization

      // Attempt to deposit when vault is locked
      const amount = new BN(5000000); // Attempt to deposit 0.005 SOL
      try {
        await program.methods
          .deposit(amount)
          .accounts({
            user: user.publicKey,
            vault,
            systemProgram: SystemProgram.programId,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          })
          .signers([user])
          .rpc();
        assert.fail("Deposit should not be allowed when vault is locked");
      } catch (err) {
        assert.include(err.toString(), "VaultLocked"); // Expect a vault locked error
      }
    } catch (e) {
      console.error(e);
      throw e;
    }
  });
/*
  it("should initialize the Time Interval Extension", async () => {
    try {
      const startSlot = new BN(100); // Example start slot
      const endSlot = new BN(200);   // Example end slot
      const tx = await program.methods
        .initializeTimeIntervalExtension(startSlot, endSlot)
        .accounts({
          authority: user.publicKey,
          vault,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc()
        .then(confirmTx)
        .then(log);
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("should deposit SOL into the vault and reflect balance", async () => {
    try {
      const amount = new BN(10000000); // Deposit 0.01 SOL
      const tx = await program.methods
        .deposit(amount)
        .accounts({
          user: user.publicKey,
          vault,
          systemProgram: SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        })
        .signers([user])
        .rpc()
        .then(confirmTx)
        .then(log);

      const balance = await connection.getBalance(vault);
      assert.equal(balance, amount.toNumber());
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("should lock the vault and prevent further deposits", async () => {
    try {
      const tx = await program.methods
        .lockVault()
        .accounts({
          lockAuthority: user.publicKey,
          vault,
        })
        .signers([user])
        .rpc()
        .then(confirmTx)
        .then(log);

      // Attempt to deposit when vault is locked
      const amount = new BN(5000000); // Attempt to deposit 0.005 SOL
      try {
        await program.methods
          .deposit(amount)
          .accounts({
            user: user.publicKey,
            vault,
            systemProgram: SystemProgram.programId,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          })
          .signers([user])
          .rpc();
        assert.fail("Deposit should not be allowed when vault is locked");
      } catch (err) {
        assert.include(err.toString(), "VaultLocked"); // Expect a vault locked error
      }
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("should unlock the vault and allow deposits", async () => {
    try {
      const tx = await program.methods
        .unlockVault()
        .accounts({
          lockAuthority: user.publicKey,
          vault,
        })
        .signers([user])
        .rpc()
        .then(confirmTx)
        .then(log);

      // Deposit should be allowed after unlocking
      const amount = new BN(5000000); // Deposit 0.005 SOL
      const depositTx = await program.methods
        .deposit(amount)
        .accounts({
          user: user.publicKey,
          vault,
          systemProgram: SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        })
        .signers([user])
        .rpc()
        .then(confirmTx)
        .then(log);

      const balance = await connection.getBalance(vault);
      assert.equal(balance, 15000000); // 0.015 SOL
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("should not allow deposit outside the specified time interval", async () => {
    try {
      // Manually set the clock to a slot outside the valid interval
      const slotOutsideInterval = new BN(250); // Example slot outside interval
      await anchor.setProvider(anchor.AnchorProvider.local(provider.wallet, { commitment: "processed" }));
      anchor.workspace.Vaultplex.provider.connection.setSlot(slotOutsideInterval.toNumber());

      const amount = new BN(5000000); // Attempt to deposit 0.005 SOL
      try {
        await program.methods
          .deposit(amount)
          .accounts({
            user: user.publicKey,
            vault,
            systemProgram: SystemProgram.programId,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          })
          .signers([user])
          .rpc();
        assert.fail("Deposit should not be allowed outside the specified time interval");
      } catch (err) {
        assert.include(err.toString(), "VaultClosedForDeposits"); // Expect a time interval error
      }
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("should withdraw SOL from the vault", async () => {
    try {
      const amount = new BN(1000000); // Withdraw 0.001 SOL
      const tx = await program.methods
        .withdraw(amount)
        .accounts({
          user: user.publicKey,
          vault,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc()
        .then(confirmTx)
        .then(log);

      const balance = await connection.getBalance(vault);
      assert.equal(balance, 14000000); // Should have 0.014 SOL left
    } catch (e) {
      console.error(e);
      throw e;
    }
  }); */
});
