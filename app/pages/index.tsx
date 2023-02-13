import { WalletDisconnectButton, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import type { NextPage } from 'next';
import styles from '../styles/Home.module.css';
import {
    AnchorProvider, BN, Program, utils, web3
} from '@project-serum/anchor';
import * as anchor from "@project-serum/anchor";
import { Connection, PublicKey } from '@solana/web3.js';
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import idl from "../../target/idl/realbox_smart_contract_solana.json";
import { RealboxSmartContractSolana } from "../../target/types/realbox_smart_contract_solana";
import {
    TOKEN_PROGRAM_ID,
    NATIVE_MINT,
    MINT_SIZE,
    createAssociatedTokenAccountInstruction,
    getAssociatedTokenAddress,
    createInitializeMintInstruction,
} from "@solana/spl-token";
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import MinimalSetup from "./minimal-setup";
import { generateUint8ArrayFromSecretKey } from '../lib/helpers';

const utf8 = utils.bytes.utf8;

const Home: NextPage = () => {
    const anchorWallet = useAnchorWallet();
    // const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.fromSecretKey(generateUint8ArrayFromSecretKey(process.env.NEXT_PUBLIC_PHANTOM_SECRETKEY));
    const fromWallet = anchor.web3.Keypair.generate()
    const mint = anchor.web3.Keypair.generate()
    async function sendTransaction() {
        if (!anchorWallet) {
            return;
        }
        const network = "http://127.0.0.1:8899";
        const connection = new Connection(network, "processed");
        const provider = new anchor.AnchorProvider(connection, anchorWallet, anchor.AnchorProvider.defaultOptions());
        const program = new anchor.Program(idl as any, idl.metadata.address, provider) as Program<RealboxSmartContractSolana>;//anchor.workspace.RealboxSmartContractSolana as Program<RealboxSmartContractSolana>;
        const key = anchorWallet.publicKey;
        console.log("key: ", key.toString());
        try {
            const associatedTokenAccount = await getAssociatedTokenAddress(
                mint.publicKey,
                fromWallet.publicKey
            );
            // const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(
            //     MINT_SIZE
            // ); 
            // const mint_tx1 = new anchor.web3.Transaction().add(
            //     anchor.web3.SystemProgram.createAccount({
            //         fromPubkey: key,
            //         newAccountPubkey: mintKey.publicKey,
            //         space: MINT_SIZE,
            //         programId: program.programId,
            //         lamports,
            //     }),
            //     createInitializeMintInstruction(
            //         mintKey.publicKey, 0, key, key
            //     ),
            //     createAssociatedTokenAccountInstruction(
            //         key, associatedTokenAccount, key, mintKey.publicKey
            //     )
            // );

            // const res = await provider.sendAndConfirm(mint_tx1, [mintKey]);
            console.log("associatedTokenAccount: ", associatedTokenAccount.toString());
            // const toATA = await getAssociatedTokenAddress(
            //     mintKey.publicKey,
            //     mintKey.publicKey,
            // );

            // const mint_tx = new anchor.web3.Transaction().add(
            //     createAssociatedTokenAccountInstruction(
            //         key, toATA, mintKey.publicKey, mintKey.publicKey, program.programId
            //     )
            // );

            // Sends and create the transaction
            // await provider.sendAndConfirm(mint_tx, []);
            // await program.methods.mintToken().accounts({
            //     mint: mintKey.publicKey,
            //     tokenProgram: TOKEN_PROGRAM_ID,
            //     tokenAccount: associatedTokenAccount,
            //     authority: key,
            // }).rpc();
            await program.methods.mintToken(new BN(100)).accounts({
                mint: mint.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                tokenAccount: associatedTokenAccount,
                authority: fromWallet.publicKey,
            }).signers([fromWallet]).rpc();
            // console.log("mint_tx: ", mint_tx);
            console.log("here")

            // Executes our transfer smart contract 
            // await program.methods.transferToken().accounts({
            //     tokenProgram: TOKEN_PROGRAM_ID,
            //     from: associatedTokenAccount,
            //     signer: key,
            //     to: toATA,
            // }).rpc();

            // Get minted token amount on the ATA for our anchor wallet
            // const minted = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount) as any)?.value?.data?.parsed?.info?.tokenAmount?.amount;
            // console.log("minted: ", minted);
        } catch (err) {
            console.log(err);
        }
    }

    return (
        <div className={styles.container}>
            <main className={styles.main}>
                <h1 className={styles.title}>
                    Welcome to <a href="https://nextjs.org">Next.js!</a>
                </h1>
                {/* <MinimalSetup /> */}
                <div className={styles.walletButtons}>
                    <WalletMultiButton />
                    <WalletDisconnectButton />
                </div>

                <p className={styles.description}>
                    <button onClick={sendTransaction}>Create Transaction</button>
                </p>
            </main>
        </div>
    );
};

export default Home;
