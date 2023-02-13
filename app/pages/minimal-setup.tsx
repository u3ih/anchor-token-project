import { useEffect, useState } from "react";
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import { PublicKey, Connection } from "@solana/web3.js";
import { RealboxSmartContractSolana } from "../../target/types/realbox_smart_contract_solana";
import idl from "../../target/idl/realbox_smart_contract_solana.json";
import * as anchor from "@project-serum/anchor";
import { AnchorProvider, Program } from "@project-serum/anchor";
import React from "react";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { generateUint8ArrayFromSecretKey } from "../lib/helpers";
const MinimalSetup = () => {
    const wallet = useAnchorWallet();
    const [program, setProgram] = useState<Program<RealboxSmartContractSolana>>();
    const InitAccount = async () => {
        if (!program || !anchor) return;
        const { provider } = program;
        if (!provider.publicKey) {
            return;
        }
        try {
            const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.fromSecretKey(generateUint8ArrayFromSecretKey(process.env.NEXT_PUBLIC_PHANTOM_SECRETKEY));
            const associatedTokenAccount = await getAssociatedTokenAddress(
                mintKey.publicKey,
                provider.publicKey,
            );
            await program.methods.mintToken().accounts({
                mint: mintKey.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                tokenAccount: associatedTokenAccount,
                authority: provider.publicKey,
            }).rpc();
            console.log("here")
            const minted = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount))?.value?.data?.parsed?.info?.tokenAmount?.amount;
            console.log("minted: ", minted);
        } catch (err) {
            console.log(err);
        }
    };

    const loadAnchor = async () => {
        if (!wallet) return;
        const programId = new PublicKey(
            idl.metadata.address,
        );
        const connection = new Connection("http://127.0.0.1:8899", {
            commitment: "processed",
        });

        const provider = new AnchorProvider(connection, wallet, {
            commitment: "processed",
        });

        const newProgram = new Program(idl as any, programId, provider);

        setProgram(newProgram);
    };

    useEffect(() => {
        if (wallet) {
            loadAnchor();
        }
    }, [wallet]);

    return (
        <div>
            <button
                onClick={() => {
                    InitAccount();
                }}
            >
                Action
            </button>
        </div>
    );
};

export default MinimalSetup;