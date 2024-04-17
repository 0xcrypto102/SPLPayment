import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplPayment } from "../target/types/spl_payment";

import { SystemProgram, Keypair, PublicKey, Transaction, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createAccount, createAssociatedTokenAccount, getAssociatedTokenAddress , ASSOCIATED_TOKEN_PROGRAM_ID,createMint, mintTo, mintToChecked, getAccount, getMint, getAssociatedTokenAddressSync,  } from "@solana/spl-token";

describe("spl_payment", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplPayment as Program<SplPayment>;

  let globalState, tokenVaultAccount, userInfo, tokenOwnerAccount: PublicKey;
  let globalStateBump,tokenVaultAccountBump,userInfoBump: number;
  
  const GLOBAL_STATE_SEED = "GLOBAL-STATE-SEED";
  const USER_INFO_SEED = "USER-INFO-SEED"; 
  const VAULT_SEED = "VAULT-SEED";
  const tokenMint = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");

  let owner = Keypair.fromSecretKey(Uint8Array.from(/**/));

  let user = Keypair.fromSecretKey(
    Uint8Array.from(/**/)
  );

  it("is initialized accounts", async() => {
    [globalState, globalStateBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(GLOBAL_STATE_SEED)
      ],
      program.programId
    );

    [tokenVaultAccount, tokenVaultAccountBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(VAULT_SEED),
        tokenMint.toBuffer()
      ],
      program.programId
    );

    [userInfo, userInfoBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(USER_INFO_SEED),
        user.publicKey.toBuffer()
      ],
      program.programId
    );

    tokenOwnerAccount = await getAssociatedTokenAddress(
      tokenMint,
      user.publicKey
    );
  })

  it("Is initialized!", async () => {
    const max_amount = 100000000000;
    const tx = await program.rpc.initialize(
      new anchor.BN(max_amount),{
        accounts: {
          owner: owner.publicKey,
          globalState,
          tokenMint,
          tokenVaultAccount,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [owner]
      }
    );
    console.log("Your transaction signature", tx);
    const globalStateData = await program.account.globalState.fetch(globalState);
    console.log("globalStateData->", globalStateData);
  });

  it("update max_amount", async() => {
    const max_amount = 50000000;

    const tx = await program.rpc.updateMaxAmount(
      new anchor.BN(max_amount),
      {
        accounts: {
          owner: owner.publicKey,
          globalState,
          systemProgram: SystemProgram.programId
        },
        signers: [owner]
      }
    );
    const globalStateData = await program.account.globalState.fetch(globalState);
    console.log("updated max_amount", Number(globalStateData.maxAmount));
  });

  it("update owner", async() => {
    const new_owner = new PublicKey("7bVNisSuPayDrdBdNh7uiJNSiLrjatnADPSTLWBoUUHb");

    const tx = await program.rpc.updateOwner(
      new_owner,
      {
        accounts: {
          owner: owner.publicKey,
          globalState,
          systemProgram: SystemProgram.programId
        },
        signers: [owner]
      }
    );
    const globalStateData = await program.account.globalState.fetch(globalState);
    console.log("updated owner",globalStateData.owner.toString());
  });
  
  it("deposit", async() => {
   
    const depositAmount = 50000000;
    
    try {
      console.log("before deposit: current balance ->")
      getTokenBalanceSpl(program.provider.connection, tokenOwnerAccount).catch(err => console.log(err));
      const tx = await program.rpc.deposit(
        new anchor.BN(depositAmount),
        {
          accounts: {
            user: user.publicKey,
            globalState,
            userInfo,
            tokenMint,
            tokenVaultAccount,
            tokenOwnerAccount,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            clock: SYSVAR_CLOCK_PUBKEY
          },
          signers: [user]
        }
      );
      const userInfoData = await program.account.userInfo.fetch(userInfo);
      console.log("userInfoData -> ", userInfoData);
    } catch (error) {
      console.log(error);
    } 
    console.log("after deposit: current balance ->")
    getTokenBalanceSpl(program.provider.connection, tokenOwnerAccount).catch(err => console.log(err));
  });

  
  it("withdraw", async() => {
    const withdrawAmount = 49000000;

    try {
      const tx = await program.rpc.withdraw(
        new anchor.BN(withdrawAmount),
        {
          accounts: {
            user: user.publicKey,
            globalState,
            userInfo,
            tokenMint,
            tokenVaultAccount,
            tokenOwnerAccount,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            clock: SYSVAR_CLOCK_PUBKEY
          },
          signers: [user]
        }
      );
      console.log("after withdraw: current balance ->")
      getTokenBalanceSpl(program.provider.connection, tokenOwnerAccount).catch(err => console.log(err));
    } catch (error) {
      console.log(error);
    }
  });
 
});

async function getTokenBalanceSpl(connection, tokenAccount) {
  const info = await getAccount(connection, tokenAccount);
  const amount = Number(info.amount);
  const mint = await getMint(connection, info.mint);
  const balance = amount / (10 ** mint.decimals);
  console.log('Balance (using Solana-Web3.js): ', balance);
  return balance;
}
