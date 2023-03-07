import { Program, web3 } from '@project-serum/anchor';
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import { BN } from 'bn.js';
import React from "react";
import { RealboxSmartContractSolana } from "../../target/types/realbox_smart_contract_solana";
import * as anchor from "@project-serum/anchor";
import { createAssociatedTokenAccountInstruction, createInitializeMintInstruction, getAssociatedTokenAddress } from '@solana/spl-token';

const AgentReturnToken = (props: { provider: any, program: Program<RealboxSmartContractSolana> | undefined, fromWallet: any }) => {
    const { provider, program, fromWallet } = props;

    const AgentReturnToken = async () => {
        if (!provider || !program) return;
        const vaultName = "REE10";
        let [realboxVault,] = await web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName)], program.programId);
        const realboxVaultData = await program.account.realboxVaultState.fetch(realboxVault);
        console.log("realboxVaultData: ", realboxVaultData);
        const associatedTokenAccount = await getAssociatedTokenAddress(
            realboxVaultData.mintToken,
            fromWallet.publicKey
        );

        const tx = await program.methods.agentReturnToken(
            new BN(15 * LAMPORTS_PER_SOL), // amount
            1, // idx
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
            <button onClick={AgentReturnToken}>Agent return token</button>
        </>
    )
}

export default AgentReturnToken;