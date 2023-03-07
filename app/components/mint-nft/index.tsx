import { Program, web3 } from '@project-serum/anchor';
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import { BN } from 'bn.js';
import React from "react";
import { RealboxSmartContractSolana } from "../../../target/types/realbox_smart_contract_solana";
import * as anchor from "@project-serum/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, MINT_SIZE, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, createInitializeMintInstruction, getAssociatedTokenAddress } from '@solana/spl-token';
import { TOKEN_METADATA_PROGRAM_ID, getMasterEdition, getMetadata, mintKey } from './helper';


const MintNft = (props: { provider: any, program: Program<RealboxSmartContractSolana> | undefined, fromWallet: any }) => {
    const { provider, program, fromWallet } = props;

    const handleMintNft = async () => {
        if (!provider || !program) return;
        const NftTokenAccount = await getAssociatedTokenAddress(
            mintKey.publicKey,
            fromWallet.publicKey
        );
        const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(
            MINT_SIZE
        );
        console.log("NFT Account: ", NftTokenAccount.toBase58());
        console.log(
            await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
        );
        console.log("Mint key: ", mintKey.publicKey.toString());
        console.log("User: ", fromWallet.publicKey.toString());
        const metadataAddress = await getMetadata(mintKey.publicKey);
        const masterEdition = await getMasterEdition(mintKey.publicKey);
        console.log("Metadata address: ", metadataAddress.toBase58());
        console.log("MasterEdition: ", masterEdition.toBase58());

        const tx = await program.methods.mintNft(
            mintKey.publicKey,
            "https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA", // uri
            "Realbox NFT", // title
            "Realx" // symbol
        ).accounts({
            mintAuthority: fromWallet.publicKey,
            mint: mintKey.publicKey,
            tokenAccount: NftTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            metadata: metadataAddress,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
            payer: fromWallet.publicKey,
            systemProgram: SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            masterEdition: masterEdition,
        }).signers([mintKey]).rpc();
        console.log("Your transaction signature", tx);
    }
    return (
        <button onClick={handleMintNft}>
            Mint nft
        </button>
    )
}

export default MintNft;