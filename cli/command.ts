import { program } from "commander";
import { PublicKey } from "@solana/web3.js";
import {
  addLiquidity,
  configProject,
  createMarket,
  resolution,
  setClusterConfig,
  swap,
  withdrawLiquidity,
} from "./scripts";

program.version("0.0.1");

programCommand("config").action(async (directory, cmd) => {
  const { env, keypair, rpc } = cmd.opts();

  console.log("Solana Cluster:", env);
  console.log("Keypair Path:", keypair);
  console.log("RPC URL:", rpc);


  await setClusterConfig(env, keypair, rpc);

  await configProject();
});

programCommand("market").action(async (directory, cmd) => {
  const { env, keypair, rpc } = cmd.opts();

  console.log("Solana Cluster:", env);
  console.log("Keypair Path:", keypair);
  console.log("RPC URL:", rpc);

  await setClusterConfig(env, keypair, rpc);

  await createMarket();
});

programCommand("swap")
  .option("-y, --yesToken <string>", "yesToken address")
  .option("-n, --noToken <string>", "noToken address")
  .option("-a, --amount <number>", "swap amount")
  .option("-s, --style <string>", "0: buy token, 1: sell token")
  .option("-t, --tokenType <string>", "0: no token, 1: yes token")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, yesToken, noToken, amount, style, tokenType } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (yesToken === undefined) {
      console.log("Error yesToken address");
      return;
    }

    if (noToken === undefined) {
      console.log("Error noToken address");
      return;
    }

    if (amount === undefined) {
      console.log("Error swap amount");
      return;
    }

    if (style === undefined) {
      console.log("Error swap style");
      return;
    }

    if (tokenType === undefined) {
      console.log("Error token style");
      return;
    }

    await swap(new PublicKey(yesToken), new PublicKey(noToken), amount, style, tokenType);
  });

programCommand("resolution")
  .option("-y, --yesToken <string>", "yesToken address")
  .option("-n, --noToken <string>", "noToken address")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, yesToken, noToken } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (yesToken === undefined) {
      console.log("Error yesToken address");
      return;
    }

    if (noToken === undefined) {
      console.log("Error noToken address");
      return;
    }

    await resolution(new PublicKey(yesToken), new PublicKey(noToken));
  });

programCommand("addlp")
  .option("-y, --yesToken <string>", "yesToken address")
  .option("-n, --noToken <string>", "noToken address")
  .option("-a, --amount <number>", "swap amount")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, yesToken, noToken, amount, style, tokenType } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (yesToken === undefined) {
      console.log("Error yesToken address");
      return;
    }

    if (noToken === undefined) {
      console.log("Error noToken address");
      return;
    }

    if (amount === undefined) {
      console.log("Error swap amount");
      return;
    }


    await addLiquidity(new PublicKey(yesToken), new PublicKey(noToken), amount);
  });


programCommand("withdraw")
  .option("-y, --yesToken <string>", "yesToken address")
  .option("-n, --noToken <string>", "noToken address")
  .option("-a, --amount <number>", "swap amount")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, yesToken, noToken, amount, style, tokenType } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (yesToken === undefined) {
      console.log("Error yesToken address");
      return;
    }

    if (noToken === undefined) {
      console.log("Error noToken address");
      return;
    }

    if (amount === undefined) {
      console.log("Error swap amount");
      return;
    }


    await withdrawLiquidity(new PublicKey(yesToken), new PublicKey(noToken), amount);
  });


function programCommand(name: string) {
  return program
    .command(name)
    .option(
      //  mainnet-beta, testnet, devnet
      "-e, --env <string>",
      "Solana cluster env name",
      "devnet"
    )
    .option(
      "-r, --rpc <string>",
      "Solana cluster RPC name",
      "https://api.devnet.solana.com"
    )
    .option(
      "-k, --keypair <string>",
      "Solana wallet Keypair Path",
      "./keys/EgBcC7KVQTh1QeU3qxCFsnwZKYMMQkv6TzgEDkKvSNLv.json"
    );
}


program.parse(process.argv);

/*

yarn script config
yarn script market
yarn script addlp -y A5LmU5ZciBaJZKT9u59tasTDNrdMpGAo4WkGhmTkoBBA -n 9jnqZhBjuAjmj6qwmGBAm7ftok437prs6WkgcfLnKzrr -a 2000000000
yarn script withdraw -y A5LmU5ZciBaJZKT9u59tasTDNrdMpGAo4WkGhmTkoBBA -n 9jnqZhBjuAjmj6qwmGBAm7ftok437prs6WkgcfLnKzrr -a 2000000000
yarn script swap -y A5LmU5ZciBaJZKT9u59tasTDNrdMpGAo4WkGhmTkoBBA -n 9jnqZhBjuAjmj6qwmGBAm7ftok437prs6WkgcfLnKzrr -a 2000000000 -s 0 -t 1
yarn script resolution -y A5LmU5ZciBaJZKT9u59tasTDNrdMpGAo4WkGhmTkoBBA -n 9jnqZhBjuAjmj6qwmGBAm7ftok437prs6WkgcfLnKzrr

*/