import { Program, web3 } from '@project-serum/anchor';
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import { BN } from 'bn.js';
import React from "react";
import { RealboxSmartContractSolana } from "../../target/types/realbox_smart_contract_solana";
import * as anchor from "@project-serum/anchor";

const ButtonSetTreasury = (props: { provider: any, program: Program<RealboxSmartContractSolana> | undefined, fromWallet: any }) => {
    const { provider, program, fromWallet } = props;
    const mintKey = anchor.web3.Keypair.generate();
    const setTreasury = async () => {
        if (!provider || !program) return;
        const vaultName = "REEB7";
        let [realboxVault,] = await web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName), fromWallet.publicKey.toBuffer()], program.programId);

        const tx = await program.methods.setTreasury(
            new BN(20), //treasury_fee: u64,
        ).accounts({
            realboxVault: realboxVault,
            treasuryAddress: fromWallet.publicKey,
            ownerAddress: fromWallet.publicKey,
        }).signers([fromWallet]).rpc();
        console.log("tx: ", tx);
    }
    return (
        <>
            <p />
            <button onClick={setTreasury}>Set new treasury</button>
        </>
    )
}

export default ButtonSetTreasury;