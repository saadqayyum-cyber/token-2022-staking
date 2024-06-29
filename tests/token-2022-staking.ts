import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token2022Staking } from "../target/types/token_2022_staking";
import * as dotenv from "dotenv";

dotenv.config();

// -------------------------------------------------------------------------------------------------
//                                GLOBAL VARIABLES
// -------------------------------------------------------------------------------------------------

const TOKEN_MINT = process.env.TOKEN_MINT;
const CONFIG_PDA_SEED = process.env.CONFIG_PDA_SEED;
const CONFIG_ATA_SEED = process.env.CONFIG_ATA_SEED;
const AUTHORITY_WALLET: number[] = JSON.parse(process.env.AUTHORITY_WALLET);

let CONFIG_PDA = { configPda: null, configPdaBump: null };
let CONFIG_ATA = { configAta: null, configAtaBump: null };

// -------------------------------------------------------------------------------------------------

describe("token-2022-staking", () => {
  anchor.setProvider(anchor.AnchorProvider.local());

  const program = anchor.workspace.Token2022Staking as Program<Token2022Staking>;

  // -------------------------------------------------------------------------------------------------
  //                                Setting Global Variables
  // -------------------------------------------------------------------------------------------------

  const TOKEN_MINT_PUBLIC_KEY = new anchor.web3.PublicKey(TOKEN_MINT);

  const authorityWallet = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(AUTHORITY_WALLET));
  const authorityPublicKey = authorityWallet.publicKey;

  [CONFIG_PDA.configPda, CONFIG_PDA.configPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(CONFIG_PDA_SEED)],
    program.programId
  );

  [CONFIG_ATA.configAta, CONFIG_ATA.configAtaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(CONFIG_ATA_SEED)],
    program.programId
  );

  // -------------------------------------------------------------------------------------------------
  //                                        TESTS
  // -------------------------------------------------------------------------------------------------

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize(new anchor.BN(30))
      .accounts({
        authority: authorityPublicKey,
        tokenMint: TOKEN_MINT_PUBLIC_KEY,
      })
      .signers([authorityWallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Update Staking Period", async () => {
    const tx = await program.methods
      .updateMinStakePeriod(new anchor.BN(60))
      .accounts({
        authority: authorityPublicKey,
      })
      .signers([authorityWallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
