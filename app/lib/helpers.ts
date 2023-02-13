import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { Connection, PublicKey } from '@solana/web3.js';

export function generateUint8ArrayFromSecretKey(secretKey: string | undefined): Uint8Array {
    if (!secretKey) return new Uint8Array();
    const b = bs58.decode(secretKey);
    return new Uint8Array(b.buffer, b.byteOffset, b.byteLength / Uint8Array.BYTES_PER_ELEMENT);
}

export async function accountOwnedByProgram(connection: Connection, publicKey: string) {
    const accountInfo = await connection.getAccountInfo(new PublicKey(publicKey));
    return accountInfo?.owner.toString();
}