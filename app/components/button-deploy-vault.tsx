import { BN } from "bn.js";
import * as anchor from "@project-serum/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress } from "@solana/spl-token";
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import dayjs from "dayjs";

const ButtonDeployVault = (props: any) => {
    const { provider, program, fromWallet } = props;
    const deployVault = async () => {
        if (!provider || !program) return;
        const mintKey = anchor.web3.Keypair.generate();
        const realboxNFT = anchor.web3.Keypair.generate();
        const vaultName = "REE8";
        let [realboxVault,] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName)], program.programId);

        const associatedTokenAccount = await getAssociatedTokenAddress(
            mintKey.publicKey,
            fromWallet.publicKey
        );

        // Sends and create the transaction
        await program.methods.initializeMintRealboxVault().accounts({
            mint: mintKey.publicKey,
            tokenAccount: associatedTokenAccount,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            payer: fromWallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([fromWallet, mintKey]).rpc();
        const today = dayjs('2023-03-03');
        const saleInfo = {
            publicUnitPrice: new BN(0.5 + LAMPORTS_PER_SOL), //public_unit_price: u64
            minSupply: new BN(5 * LAMPORTS_PER_SOL), //min_supply: u64,
            maxSupply: new BN(100 * LAMPORTS_PER_SOL), //max_supply: u64,
            privateStartTime: new BN(today.unix()), //private_start_time: u64,
            publicStartTime: new BN(today.add(1, "days").unix()), //public_start_time: u64,
            endTime: new BN(today.add(2, "days").unix()), //end_time: u64,
        }

        const tx = await program.methods.deployVault(
            vaultName, //vault_token_name
            fromWallet.publicKey, // treasuryAddress
            new BN(200), // treasuryFee
            saleInfo
        ).accounts({
            mintToken: mintKey.publicKey,
            baseToken: mintKey.publicKey,
            realboxVault: realboxVault,
            realx: realboxNFT.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            ownerAddress: fromWallet.publicKey,
        }).signers([fromWallet]).rpc();
        console.log("mintKey.publicKey: ", mintKey.publicKey.toString());
        // console.log("tx: ", tx);

        // Get minted token amount on the ATA for our anchor wallet

    }
    return (
        <button onClick={deployVault}>Deploy vault</button>
    )
}

export default ButtonDeployVault;