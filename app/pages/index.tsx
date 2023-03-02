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
import ButtonSetTreasury from "../components/set-treasury";
import AgentByToken from '../components/agent-buy-token';
import FinalizedToken from '../components/finalize';
import ClaimOrRefundToken from '../components/claim-or-refund';
import AgentReturnToken from '../components/agent-return-token';

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
        const vaultName = "REE8";
        let [realboxVault,] = await web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName)], program.programId);

        // const transfer = await program.provider.connection.getParsedAccountInfo(address);
        // console.log("transfer: ", transfer);
        const account = await program.account.realboxVaultState.fetch(realboxVault);
        console.log('account: ', account)
        console.log("account.totalSupply: ", account.totalSupply.toNumber());
        await getTokenAccounts("66iRaLdHM6rwWdfTrK8JABh8DrXkzBWMkxHpufAK9agc", connection);
        // const tx = await program.methods.getVaultInfo().accounts({
        //     realboxVault: new PublicKey("4P1wGGQ75Pfk7nYLfgYQSGr5TJV6ruVhBd8cm93rXESs"),
        // }).rpc();
    }

    return (
        <main className={styles.container}>
            <div className={styles.main}>
                <div className={styles.walletButtons}>
                    <WalletMultiButton />
                    <WalletDisconnectButton />
                </div>

                <p className={styles.description}>
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
                    <p />
                    <AgentByToken {...{
                        provider,
                        program,
                        fromWallet
                    }} />
                    <p />
                    <AgentReturnToken {...{
                        provider,
                        program,
                        fromWallet
                    }} />
                    <p />
                    <FinalizedToken {...{
                        provider,
                        program,
                        fromWallet
                    }} />
                    <p />
                    <ClaimOrRefundToken {...{
                        provider,
                        program,
                        fromWallet
                    }} />
                </p>
            </div>
        </main>
    );
};

export default Home;
