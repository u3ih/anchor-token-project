import * as anchor from "@project-serum/anchor";
import { MINT_SIZE, TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, createInitializeMintInstruction, getAssociatedTokenAddress, getMinimumBalanceForRentExemptAccount } from "@solana/spl-token";
import { BN } from "bn.js";
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import React from "react"

const MintAndTransferToken = (props: any) => {
    const { provider, program, fromWallet } = props;
    const mintKey = anchor.web3.Keypair.generate();
    const toWallet = anchor.web3.Keypair.generate();
    async function mintToken() {
        if (!provider || !program) return;
        try {
            const associatedTokenAccount = await getAssociatedTokenAddress(
                mintKey.publicKey,
                fromWallet.publicKey
            );
            console.log("associatedTokenAccount: ", associatedTokenAccount.toString());
            console.log("mintKey: ", mintKey.publicKey.toString());
            const mint_tx = new anchor.web3.Transaction().add(
                // Use anchor to create an account from the mint key that we created
                anchor.web3.SystemProgram.createAccount({
                    fromPubkey: fromWallet.publicKey,
                    newAccountPubkey: mintKey.publicKey,
                    space: MINT_SIZE,
                    programId: TOKEN_PROGRAM_ID,
                    lamports: await getMinimumBalanceForRentExemptAccount(provider.connection),
                }),
                // Fire a transaction to create our mint account that is controlled by our anchor wallet
                createInitializeMintInstruction(
                    mintKey.publicKey, 0, fromWallet.publicKey, fromWallet.publicKey
                ),
                // Create the ATA account that is associated with our mint on our anchor wallet
                createAssociatedTokenAccountInstruction(
                    fromWallet.publicKey, associatedTokenAccount, fromWallet.publicKey, mintKey.publicKey
                )
            );

            // Sends and create the transaction
            await provider.sendAndConfirm(mint_tx, [fromWallet, mintKey]);

            // const res = await provider.sendAndConfirm(mint_tx1, [mintKey]);
            console.log("associatedTokenAccount: ", associatedTokenAccount.toString());

            await program.methods.mintToken(new BN(100)).accounts({
                mint: mintKey.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                tokenAccount: associatedTokenAccount,
                authority: fromWallet.publicKey,
            }).rpc();
            const minted = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount) as any).value.data.parsed.info;
            console.log("minted: ", minted);
        } catch (err) {
            console.log(err);
        }
    }

    const transferToken = async () => {
        if (!provider || !program) return;
        const mintKey = new PublicKey("7ijEVNKm8ryx7wdmW1qwcx8U3hFXbGtMsZE7SfYPjVge")
        const toATA = await getAssociatedTokenAddress(
            mintKey,
            toWallet.publicKey
        );
        const associatedTokenAccount = await getAssociatedTokenAddress(
            mintKey,
            fromWallet.publicKey
        );
        const mint_tx_tranfer = new anchor.web3.Transaction().add(
            // Create the ATA account that is associated with our To wallet
            createAssociatedTokenAccountInstruction(
                fromWallet.publicKey, toATA, toWallet.publicKey, mintKey
            )
        );

        // Sends and create the transaction
        await provider.sendAndConfirm(mint_tx_tranfer, []);
        console.log("toATA: ", toATA.toString());

        // Executes our transfer smart contract 
        await program.methods.transferToken().accounts({
            tokenProgram: TOKEN_PROGRAM_ID,
            from: associatedTokenAccount,
            signer: fromWallet.publicKey,
            to: toATA,
        }).rpc();

        // Get minted token amount on the ATA for our anchor wallet
        const transfer = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount) as any)?.value?.data?.parsed?.info;
        console.log("transfer: ", transfer);
        // await getTokenAccounts("66iRaLdHM6rwWdfTrK8JABh8DrXkzBWMkxHpufAK9agc", connection);
    }
    return (
        <>
            <button onClick={mintToken}>Mint token</button>
            <p />
            <button onClick={transferToken}>transfer token</button>
        </>
    )
}

export default MintAndTransferToken;