import { Program, web3 } from '@project-serum/anchor';
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import { BN } from 'bn.js';
import React from "react";
import { RealboxSmartContractSolana } from "../../target/types/realbox_smart_contract_solana";
import * as anchor from "@project-serum/anchor";
import { createAssociatedTokenAccountInstruction, createInitializeMintInstruction, getAssociatedTokenAddress } from '@solana/spl-token';

const AgentByToken = (props: { provider: any, program: Program<RealboxSmartContractSolana> | undefined, fromWallet: any }) => {
    const { provider, program, fromWallet } = props;
    const toWallet = web3.Keypair.generate();
    const AgentByToken = async () => {
        if (!provider || !program) return;
        const vaultName = "REE8";
        let [realboxVault,] = await web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName)], program.programId);
        const realboxVaultData = await program.account.realboxVaultState.fetch(realboxVault);
        console.log("realboxVaultData: ", realboxVaultData);
        // const signature = await provider.connection.requestAirdrop(
        //     toWallet.publicKey,
        //     10 * LAMPORTS_PER_SOL
        // );
        // const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash();
        // console.log("here")
        // await provider.connection.confirmTransaction({
        //     blockhash,
        //     lastValidBlockHeight,
        //     signature
        // }, 'finalized');
        const associatedTokenAccount = await getAssociatedTokenAddress(
            realboxVaultData.mintToken,
            fromWallet.publicKey
        );

        const tx = await program.methods.agentBuyToken(
            new BN(15 * LAMPORTS_PER_SOL), // amount
            0.2, // prices
            { indirect: {} }, //channel // indirect, directOnchain
            "638727261558635d1da04b35", // uid
        ).accounts({
            mintToken: realboxVaultData.mintToken,
            realboxVault: realboxVault,
            tokenAccount: associatedTokenAccount,
            tokenProgram: realboxVaultData.tokenProgram,
            ownerAddress: fromWallet.publicKey,
        }).signers([fromWallet]).rpc();
        // console.log("tx: ", tx);
    }
    return (
        <>
            <button onClick={AgentByToken}>Agent buy token</button>
        </>
    )
}

export default AgentByToken;