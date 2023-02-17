import { WalletDisconnectButton, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import type { NextPage } from 'next';
import styles from '../styles/Home.module.css';
import {
    AnchorProvider, BN, Program, utils, web3
} from '@project-serum/anchor';
import * as anchor from "@project-serum/anchor";
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import idl from "../../target/idl/realbox_smart_contract_solana.json";
import { RealboxSmartContractSolana } from "../../target/types/realbox_smart_contract_solana";
import { generateUint8ArrayFromSecretKey, getTokenAccounts } from '../lib/helpers';
import { useEffect, useState } from 'react';
import { accountOwnedByProgram } from '../lib/helpers';
import * as borsh from 'borsh';
import ButtonDeployVault from '../components/button-deploy-vault';
import MintAndTransferToken from "../components/mint-transfer-token";
import ButtonSetTreasury from "../components/set-treasury";

const Home: NextPage = () => {
    const anchorWallet = useAnchorWallet();
    const [provider, setProvider] = useState<AnchorProvider>();
    const [program, setProgram] = useState<Program<RealboxSmartContractSolana>>();
    const fromWallet: anchor.web3.Keypair = anchor.web3.Keypair.fromSecretKey(generateUint8ArrayFromSecretKey(process.env.NEXT_PUBLIC_PHANTOM_SECRETKEY));
    // const fromWallet = anchor.web3.Keypair.generate()
    const network = "http://127.0.0.1:8899";//"https://api.devnet.solana.com";
    const connection = new Connection(network, "processed");
    useEffect(() => {
        if (!anchorWallet) {
            return;
        }
        const provider = new anchor.AnchorProvider(connection, anchorWallet, anchor.AnchorProvider.defaultOptions());
        const program = new anchor.Program(idl as any, idl.metadata.address, provider) as Program<RealboxSmartContractSolana>;
        setProgram(program)
        setProvider(provider);
    }, [anchorWallet])

    const getInfoByAddress = async () => {
        if (!provider || !program) return;
        const address = new PublicKey("49BKZDcrxBu2PHgvVhZCPJW51Pikb549bB3Sm3CvejDa");
        const vaultName = "REB3";
        let [realboxVault,] = await web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName), fromWallet.publicKey.toBuffer()], program.programId);

        const transfer = await program.provider.connection.getParsedAccountInfo(address);
        console.log("transfer: ", transfer);

        // const tx = await program.methods.getVaultInfo().accounts({
        //     realboxVault: new PublicKey("4P1wGGQ75Pfk7nYLfgYQSGr5TJV6ruVhBd8cm93rXESs"),
        // }).rpc();
    }

    return (
        <div className={styles.container}>
            <main className={styles.main}>
                <div className={styles.walletButtons}>
                    <WalletMultiButton />
                    <WalletDisconnectButton />
                </div>

                <p className={styles.description}>
                    <MintAndTransferToken {...{
                        provider,
                        program,
                        fromWallet
                    }} />
                    <p />
                    <ButtonDeployVault {...{
                        provider,
                        program,
                        fromWallet
                    }} />
                    <p />
                    <button onClick={getInfoByAddress}>Get info by address</button>
                    <ButtonSetTreasury {...{
                        provider,
                        program,
                        fromWallet
                    }} />
                </p>
            </main>
        </div>
    );
};

export default Home;
