import { Program, web3 } from '@project-serum/anchor';
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import { BN } from 'bn.js';
import React from "react";
import { RealboxSmartContractSolana } from "../../target/types/realbox_smart_contract_solana";
import * as anchor from "@project-serum/anchor";
import { createAssociatedTokenAccountInstruction, createInitializeMintInstruction, getAssociatedTokenAddress } from '@solana/spl-token';

const UnlockToken = (props: { provider: any, program: Program<RealboxSmartContractSolana> | undefined, fromWallet: any }) => {
    const { provider, program, fromWallet } = props;

    const UnlockToken = async () => {
        if (!provider || !program) return;
        const vaultName = "REE10";
        let [realboxVault,] = await web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName)], program.programId);

        const tx = await program.methods.unlockToken().accounts({
            realboxVault: realboxVault,
            ownerAddress: fromWallet.publicKey,
        }).signers([fromWallet]).rpc();
        // console.log("tx: ", tx);
    }
    return (
        <>
            <button onClick={UnlockToken}>Unlock token</button>
        </>
    )
}

export default UnlockToken;