import * as anchor from "@coral-xyz/anchor";
import { BN, Program, web3 } from "@coral-xyz/anchor";
import fs from "fs";

import { Keypair, Connection, PublicKey, Transaction } from "@solana/web3.js";

import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

import { PredictionMarket } from "../target/types/prediction_market";
import {
  createConfigTx,
  createMarketTx,
  mintNoTokenTx,
  swapTx,
  resolutionTx,
  addLiquidityTx,
  withdrawLiquidityTx,
} from "../lib/scripts";
import { execTx } from "../lib/util";
import {
  TEST_DECIMALS,
  TEST_YES_NAME,
  TEST_YES_SYMBOL,
  TEST_YES_URI,
  TEST_NO_NAME,
  TEST_NO_SYMBOL,
  TEST_NO_URI,
  TEST_TOKEN_SUPPLY,
  TEST_VIRTUAL_RESERVES,
  TEST_INITIAL_VIRTUAL_TOKEN_RESERVES,
  TEST_INITIAL_VIRTUAL_SOL_RESERVES,
  TEST_INITIAL_REAL_TOKEN_RESERVES,
  SEED_CONFIG,
  SEED_MARKET,
  SEED_USERINFO,
} from "../lib/constant";

let solConnection: Connection = null;
let program: Program<PredictionMarket> = null;
let payer: NodeWallet = null;

/**
 * Set cluster, provider, program
 * If rpc != null use rpc, otherwise use cluster param
 * @param cluster - cluster ex. mainnet-beta, devnet ...
 * @param keypair - wallet keypair
 * @param rpc - rpc
 */
export const setClusterConfig = async (
  cluster: web3.Cluster,
  keypair: string,
  rpc?: string
) => {
  if (!rpc) {
    solConnection = new web3.Connection(web3.clusterApiUrl(cluster));
  } else {
    solConnection = new web3.Connection(rpc);
  }

  const walletKeypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync(keypair, "utf-8"))),
    { skipValidation: true }
  );
  payer = new NodeWallet(walletKeypair);

  console.log("Wallet Address: ", payer.publicKey.toBase58());

  anchor.setProvider(
    new anchor.AnchorProvider(solConnection, payer, {
      skipPreflight: true,
      commitment: "confirmed",
    })
  );

  // Generate the program client from IDL.
  program = anchor.workspace.prediction_market as Program<PredictionMarket>;

  console.log("ProgramId: ", program.programId.toBase58());
};

export const configProject = async () => {
  // Create a dummy config object to pass as argument.
  const newConfig = {
    authority: payer.publicKey,
    pendingAuthority: PublicKey.default,

    teamWallet: payer.publicKey,

    platformBuyFee: new BN(100), // Example fee: 1%
    platformSellFee: new BN(100), // Example fee: 1%
    lpBuyFee: new BN(20),
    lpSellFee: new BN(20),

    tokenSupplyConfig: new BN(TEST_INITIAL_VIRTUAL_TOKEN_RESERVES),
    tokenDecimalsConfig: 6,

    initialRealTokenReservesConfig: new BN(TEST_INITIAL_REAL_TOKEN_RESERVES),

    minSolLiquidity: new BN(5_000_000_000),

    initialized: true,
  };
  const tx = await createConfigTx(
    payer.publicKey,
    newConfig,
    solConnection,
    program
  );

  await execTx(tx, solConnection, payer);
};

export const createMarket = async () => {

  const noTokenMintTx = await mintNoTokenTx(

    //  metadata
    TEST_NO_SYMBOL,
    TEST_NO_URI,

    payer.publicKey,

    solConnection,
    program
  );

  const configPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_CONFIG)],
    program.programId
  )[0];

  const configAccount = await program.account.config.fetch(configPda);

  const marketCreationTx = await createMarketTx(

    //  metadata
    TEST_YES_SYMBOL,
    TEST_YES_URI,

    payer.publicKey,
    configAccount.teamWallet,
    noTokenMintTx.no_tokenKp.publicKey,

    solConnection,
    program
  );

  const transaction = new Transaction()
  transaction.add(...noTokenMintTx.tx.instructions)
  transaction.add(...marketCreationTx.tx.instructions)

  transaction.feePayer = payer.publicKey;
  transaction.recentBlockhash = (await solConnection.getLatestBlockhash()).blockhash;
  transaction.sign(noTokenMintTx.no_tokenKp, marketCreationTx.yes_tokenKp);



  await execTx(transaction, solConnection, payer);

  const marketPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_MARKET), marketCreationTx.yes_tokenKp.publicKey.toBytes(), noTokenMintTx.no_tokenKp.publicKey.toBytes()],
    program.programId
  )[0];

  console.log("🚀 ~ createMarket ~ no_tokenKp:", noTokenMintTx.no_tokenKp.publicKey.toBase58());
  console.log("🚀 ~ createMarket ~ yes_tokenKp:", marketCreationTx.yes_tokenKp.publicKey.toBase58());
  console.log("🚀 ~ createMarket ~ marketPda:", marketPda.toBase58())
  const marketAccount = await program.account.market.fetch(marketPda);
  console.log("🚀 ~ createMarket ~ marketAccount:", marketAccount)

};


export const swap = async (
  yes_token: PublicKey,
  no_token: PublicKey,

  amount: number,
  style: number,
  token_type: number,
) => {
  const tx = await swapTx(
    payer.publicKey,
    yes_token,
    no_token,
    amount,
    style,
    token_type,
    solConnection,
    program
  );

  await execTx(tx, solConnection, payer);

  const marketPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_MARKET), yes_token.toBytes(), no_token.toBytes()],
    program.programId
  )[0];

  console.log("🚀 ~ createMarket ~ marketPda:", marketPda.toBase58())
  const marketAccount = await program.account.market.fetch(marketPda);
  console.log("🚀 ~ createMarket ~ marketAccount:", marketAccount)

};


export const resolution = async (
  yes_token: PublicKey,
  no_token: PublicKey,
) => {

  const marketPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_MARKET), yes_token.toBytes(), no_token.toBytes()],
    program.programId
  )[0];

  console.log("🚀 ~ createMarket ~ marketPda:", marketPda.toBase58())
  const marketAccount = await program.account.market.fetch(marketPda);
  console.log("🚀 ~ createMarket ~ marketAccount:", marketAccount)

  const tx = await resolutionTx(
    payer.publicKey,
    payer.publicKey,
    yes_token,
    no_token,

    solConnection,
    program
  );

  await execTx(tx, solConnection, payer);

};

export const addLiquidity = async (
  yes_token: PublicKey,
  no_token: PublicKey,

  amount: number,
) => {
  const tx = await addLiquidityTx(
    payer.publicKey,
    yes_token,
    no_token,
    amount,
    solConnection,
    program
  );

  await execTx(tx, solConnection, payer);

  const marketPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_MARKET), yes_token.toBytes(), no_token.toBytes()],
    program.programId
  )[0];
  console.log("🚀 ~ createMarket ~ marketPda:", marketPda.toBase58())
  const marketAccount = await program.account.market.fetch(marketPda);
  console.log("🚀 ~ createMarket ~ marketAccount:", marketAccount)

  const [userInfoPda, _] = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_USERINFO), payer.publicKey.toBytes(), marketPda.toBytes()],
    program.programId
  );
  console.log("🚀 ~ resolutionTx ~ userInfoPda:", userInfoPda.toBase58())
  const userInfoAccount = await program.account.userInfo.fetch(userInfoPda);
  console.log("🚀 ~ userInfoAccount:", userInfoAccount)
};

export const withdrawLiquidity = async (
  yes_token: PublicKey,
  no_token: PublicKey,

  amount: number,
) => {
  const tx = await withdrawLiquidityTx(
    payer.publicKey,
    yes_token,
    no_token,
    amount,
    solConnection,
    program
  );

  await execTx(tx, solConnection, payer);

  const marketPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_MARKET), yes_token.toBytes(), no_token.toBytes()],
    program.programId
  )[0];

  console.log("🚀 ~ createMarket ~ marketPda:", marketPda.toBase58())
  const marketAccount = await program.account.market.fetch(marketPda);
  console.log("🚀 ~ createMarket ~ marketAccount:", marketAccount)

  const [userInfoPda, _] = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_USERINFO), payer.publicKey.toBytes(), marketPda.toBytes()],
    program.programId
  );
  console.log("🚀 ~ resolutionTx ~ userInfoPda:", userInfoPda.toBase58())
  const userInfoAccount = await program.account.userInfo.fetch(userInfoPda);
  console.log("🚀 ~ userInfoAccount:", userInfoAccount)
};

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
} 