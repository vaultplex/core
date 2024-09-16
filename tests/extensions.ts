// tests/extensions.ts
import { PublicKey } from "@solana/web3.js";

// Define LockExtension class to mirror the Rust struct
export class LockExtension {
  lockAuthority: PublicKey;
  isLocked: boolean;

  constructor(lockAuthority: PublicKey, isLocked: boolean) {
    this.lockAuthority = lockAuthority;
    this.isLocked = isLocked;
  }

  // Deserialize the LockExtension data from a Buffer
  static fromBuffer(buffer: Buffer): LockExtension {
    const lockAuthority = new PublicKey(buffer.slice(0, 32)); // First 32 bytes for the PublicKey
    const isLocked = buffer[32] === 1; // Next 1 byte for the boolean value
    return new LockExtension(lockAuthority, isLocked);
  }
}
