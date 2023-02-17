import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { Connection, PublicKey, GetProgramAccountsFilter } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";


export function generateUint8ArrayFromSecretKey(secretKey: string | undefined): Uint8Array {
    if (!secretKey) return new Uint8Array();
    const b = bs58.decode(secretKey);
    return new Uint8Array(b.buffer, b.byteOffset, b.byteLength / Uint8Array.BYTES_PER_ELEMENT);
}

export async function accountOwnedByProgram(connection: Connection, publicKey: string) {
    const accountInfo = await connection.getAccountInfo(new PublicKey(publicKey));
    return accountInfo?.owner.toString();
}

export async function getTokenAccounts(wallet: string, solanaConnection: Connection) {
    const filters: GetProgramAccountsFilter[] = [
        {
            dataSize: 165,    //size of account (bytes)
        },
        {
            memcmp: {
                offset: 32,     //location of our query in the account (bytes)
                bytes: wallet,  //our search criteria, a base58 encoded string
            },
        }];
    const accounts = await solanaConnection.getParsedProgramAccounts(
        TOKEN_PROGRAM_ID, //new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
        { filters: filters }
    );
    console.log(`Found ${accounts.length} token account(s) for wallet ${wallet}.`);
    accounts.forEach((account, i) => {
        //Parse the account data
        const parsedAccountInfo: any = account.account.data;
        const info = parsedAccountInfo["parsed"]["info"];
        // const mintAddress: string = parsedAccountInfo["parsed"]["info"]["mint"];
        // const tokenBalance: number = parsedAccountInfo["parsed"]["info"]["tokenAmount"]["uiAmount"];
        //Log results
        console.log(`Token Account No. ${i + 1}: ${account.pubkey.toString()}`);
        console.log("--info: ", info);
    });
}