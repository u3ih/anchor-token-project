import { Program, web3 } from '@project-serum/anchor';
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import { BN } from 'bn.js';
import React from "react";
import { RealboxSmartContractSolana } from "../../target/types/realbox_smart_contract_solana";

const FinalizedToken = (props: { provider: any, program: Program<RealboxSmartContractSolana> | undefined, fromWallet: any }) => {
    const { provider, program, fromWallet } = props;
    const handleFinalized = async () => {
        if (!provider || !program) return;
        const vaultName = "REE8";
        let [realboxVault,] = await web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName)], program.programId);

        const tx = await program.methods.finalize(
            new BN(15 * LAMPORTS_PER_SOL), // totalSupply
        ).accounts({
            realboxVault: realboxVault,
            ownerAddress: fromWallet.publicKey,
        }).signers([fromWallet]).rpc();
        // console.log("tx: ", tx);
    }
    return (
        <>
            <button onClick={handleFinalized}>Finalize token</button>
        </>
    )
}

export default FinalizedToken;