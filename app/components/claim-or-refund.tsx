import { Program, web3 } from '@project-serum/anchor';
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import * as anchor from "@project-serum/anchor";
import { BN } from 'bn.js';
import React from "react";
import { RealboxSmartContractSolana } from "../../target/types/realbox_smart_contract_solana";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from '@solana/spl-token';

const ClaimOrRefundToken = (props: { provider: any, program: Program<RealboxSmartContractSolana> | undefined, fromWallet: any }) => {
    const { provider, program, fromWallet } = props;

    const handleClaimorRefund = async () => {
        if (!provider || !program) return;
        const vaultName = "REE8";
        let [realboxVault,] = await web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName)], program.programId);
        const realboxVaultData = await program.account.realboxVaultState.fetch(realboxVault);
        console.log("realboxVaultData: ", realboxVaultData);

        const ATABaseToken = await getAssociatedTokenAddress(
            realboxVaultData.mintBase,
            realboxVaultData.ownerAddress
        );

        const ATATokenMint = await getAssociatedTokenAddress(
            realboxVaultData.mintToken,
            realboxVaultData.ownerAddress
        );

        const ATATreasuryAccount = await getAssociatedTokenAddress(
            realboxVaultData.mintBase,
            realboxVaultData.treasuryAddress
        );


        const tx = await program.methods.claimOrRefund().accounts({
            mintToken: realboxVaultData.mintToken,
            mintBase: realboxVaultData.mintBase,
            tokenProgram: TOKEN_PROGRAM_ID,
            baseTokenAccount: ATABaseToken,
            treasuryAccount: ATATreasuryAccount,
            treasuryAddress: realboxVaultData.treasuryAddress,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            realboxVault: realboxVault,
            tokenAccount: ATATokenMint,
            ownerAddress: fromWallet.publicKey,
        }).signers([fromWallet]).rpc();
        // console.log("tx: ", tx);
    }
    return (
        <>
            <button onClick={handleClaimorRefund}>Claim or refund token</button>
        </>
    )
}

export default ClaimOrRefundToken;