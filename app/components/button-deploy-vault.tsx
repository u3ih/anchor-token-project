import { BN } from "bn.js";
import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Struct } from '@solana/web3.js';
import dayjs from "dayjs";

const ButtonDeployVault = (props: any) => {
    const { provider, program, fromWallet } = props;
    const deployVault = async () => {
        if (!provider || !program) return;
        const mintKey = anchor.web3.Keypair.generate();
        const realboxNFT = anchor.web3.Keypair.generate();
        const vaultName = "REE1";
        let [realboxVault,] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(vaultName)], program.programId);
        console.log("mintKey.publicKey: ", mintKey.publicKey.toString())
        console.log("realboxVault: ", realboxVault.toString());
        console.log("realboxNFT: ", realboxNFT.publicKey.toString())
        // Sends and create the transaction
        await program.methods.initializeMintRealboxVault().accounts({
            mint: mintKey.publicKey,
            payer: fromWallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([fromWallet, mintKey]).rpc();
        const today = dayjs('2023-02-22');
        const saleInfo = {
            publicUnitPrice: new BN(5), //public_unit_price: u64
            minSupply: new BN(5), //min_supply: u64,
            maxSupply: new BN(100), //max_supply: u64,
            privateStartTime: new BN(today.unix()), //private_start_time: u64,
            publicStartTime: new BN(today.add(1, "days").unix()), //public_start_time: u64,
            endTime: new BN(today.add(2, "days").unix()), //end_time: u64,
        }

        const tx = await program.methods.deployVault(
            vaultName, //vault_token_name
            saleInfo
        ).accounts({
            mint: mintKey.publicKey,
            realboxVault: realboxVault,
            realx: realboxNFT.publicKey,
            systemProgram: SystemProgram.programId,
            baseToken: TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
            ownerAddress: fromWallet.publicKey,
        }).signers([fromWallet]).rpc();
        console.log("tx: ", tx);

        // Get minted token amount on the ATA for our anchor wallet

    }
    return (
        <button onClick={deployVault}>Deploy vault</button>
    )
}

export default ButtonDeployVault;