import * as anchor from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { EscrowAnchor } from "../target/types/escrow_anchor";

describe("escrow-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EscrowAnchor as Program<EscrowAnchor>;

  const escrowState = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state")],
    program.programId
  )[0];

  const escrowManagerKeypair = anchor.web3.Keypair.generate();
  const escrowManager = escrowManagerKeypair.publicKey;

  const newManagerKeypair = anchor.web3.Keypair.generate();
  const newManager = newManagerKeypair.publicKey;

  const fundingAccountKeypair = anchor.web3.Keypair.generate();
  const fundingAccount = fundingAccountKeypair.publicKey;

  const tokenMintKeypairA = anchor.web3.Keypair.generate();
  const tokenMintA = tokenMintKeypairA.publicKey;

  const tokenMintKeypairB = anchor.web3.Keypair.generate();
  const tokenMintB = tokenMintKeypairB.publicKey;

  const escrowTokenAFeeAccount = anchor.utils.token.associatedAddress({
    mint: tokenMintA,
    owner: escrowState
  });

  const escrowTokenBFeeAccount = anchor.utils.token.associatedAddress({
    mint: tokenMintB,
    owner: escrowState
  });

  const makerKeypair = anchor.web3.Keypair.generate();
  const maker = makerKeypair.publicKey;

  const takerKeypair = anchor.web3.Keypair.generate();
  const taker = takerKeypair.publicKey;

  const makerTokenAAccount = anchor.utils.token.associatedAddress({
    mint: tokenMintA,
    owner: maker
  });

  const makerTokenBAccount = anchor.utils.token.associatedAddress({
    mint: tokenMintB,
    owner: maker
  });

  const takerTokenAAccount = anchor.utils.token.associatedAddress({
    mint: tokenMintA,
    owner: taker
  });

  const takerTokenBAccount = anchor.utils.token.associatedAddress({
    mint: tokenMintB,
    owner: taker
  });

  const escrowAccount1 = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("offer"),
      maker.toBuffer(),
      new anchor.BN(0).toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  )[0];

  const escrowAccount2 = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("offer"),
      maker.toBuffer(),
      new anchor.BN(1).toArrayLike(Buffer, "le", 8)
    ],
    program.programId
  )[0];

  const escrowTokenAVaultAccount1 = anchor.utils.token.associatedAddress({
    mint: tokenMintA,
    owner: escrowAccount1
  });

  const escrowTokenAVaultAccount2 = anchor.utils.token.associatedAddress({
    mint: tokenMintA,
    owner: escrowAccount2
  });

  const initializeArgs = { makerFeeBps: 100, takerFeeBps: 100 };

  const setFeesArgs = { makerFeeBps: 100, takerFeeBps: 100 };

  const makeOffer1Args = {
    id: new anchor.BN(0),
    tokenAOfferedAmount: new anchor.BN(1000000000000),
    tokenBWantedAmount: new anchor.BN(1000000000000)
  };

  const makeOffer2Args = {
    id: new anchor.BN(1),
    tokenAOfferedAmount: new anchor.BN(1000000000000),
    tokenBWantedAmount: new anchor.BN(1000000000000)
  };

  before(async () => {
    const tx0 = await program.provider.connection.requestAirdrop(
      fundingAccount,
      anchor.web3.LAMPORTS_PER_SOL * 1000
    );
    await program.provider.connection.confirmTransaction(tx0);

    const tx1 = await program.provider.connection.requestAirdrop(
      escrowManager,
      anchor.web3.LAMPORTS_PER_SOL * 1000
    );
    await program.provider.connection.confirmTransaction(tx1);

    const tx2 = await program.provider.connection.requestAirdrop(
      newManager,
      anchor.web3.LAMPORTS_PER_SOL * 1000
    );
    await program.provider.connection.confirmTransaction(tx2);

    await token.createMint(
      program.provider.connection,
      fundingAccountKeypair,
      fundingAccount,
      fundingAccount,
      9,
      tokenMintKeypairA
    );
    await token.createMint(
      program.provider.connection,
      fundingAccountKeypair,
      fundingAccount,
      fundingAccount,
      9,
      tokenMintKeypairB
    );

    await token.createAccount(
      program.provider.connection,
      fundingAccountKeypair,
      tokenMintA,
      maker
    );
    await token.createAccount(
      program.provider.connection,
      fundingAccountKeypair,
      tokenMintB,
      taker
    );

    await token.mintTo(
      program.provider.connection,
      fundingAccountKeypair,
      tokenMintA,
      makerTokenAAccount,
      fundingAccountKeypair,
      1000000000000
    );

    await token.mintTo(
      program.provider.connection,
      fundingAccountKeypair,
      tokenMintB,
      takerTokenBAccount,
      fundingAccountKeypair,
      1000000000000
    );
  });

  it("escrow state initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(initializeArgs)
      .accounts({
        escrowState: escrowState,
        escrowManager: escrowManager,
        fundingAccount: fundingAccount,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([escrowManagerKeypair, fundingAccountKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("escrow state set fees", async () => {
    // Add your test here.
    const tx = await program.methods
      .setFees(setFeesArgs)
      .accounts({
        escrowState: escrowState,
        escrowManager: escrowManager
      })
      .signers([escrowManagerKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("escrow state set manager", async () => {
    // Add your test here.
    const tx = await program.methods
      .setManager()
      .accounts({
        escrowState: escrowState,
        escrowManager: escrowManager,
        newManager: newManager
      })
      .signers([escrowManagerKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("make offer", async () => {
    const tx = await program.methods
      .makeOffer(makeOffer1Args)
      .accounts({
        escrowAccount: escrowAccount1,
        tokenAMintAccount: tokenMintA,
        tokenBMintAccount: tokenMintB,
        makerTokenAAccount: makerTokenAAccount,
        escrowTokenAVaultAccount: escrowTokenAVaultAccount1,
        maker: maker,
        fundingAccount: fundingAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([makerKeypair, fundingAccountKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("take offer", async () => {
    const tx = await program.methods
      .takeOffer()
      .accounts({
        escrowState: escrowState,
        escrowAccount: escrowAccount1,
        tokenAMintAccount: tokenMintA,
        tokenBMintAccount: tokenMintB,
        makerTokenBAccount: makerTokenBAccount,
        takerTokenAAccount: takerTokenAAccount,
        takerTokenBAccount: takerTokenBAccount,
        escrowTokenAFeeAccount: escrowTokenAFeeAccount,
        escrowTokenBFeeAccount: escrowTokenBFeeAccount,
        escrowTokenAVaultAccount: escrowTokenAVaultAccount1,
        maker: maker,
        taker: taker,
        fundingAccount: fundingAccount
      })
      .signers([takerKeypair, fundingAccountKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("make offer", async () => {
    const tx = await program.methods
      .makeOffer(makeOffer2Args)
      .accounts({
        escrowAccount: escrowAccount2,
        tokenAMintAccount: tokenMintA,
        tokenBMintAccount: tokenMintB,
        makerTokenAAccount: makerTokenAAccount,
        escrowTokenAVaultAccount: escrowTokenAVaultAccount2,
        maker: maker,
        fundingAccount: fundingAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([makerKeypair, fundingAccountKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("cancel offer", async () => {
    const tx = await program.methods
      .cancelOffer()
      .accounts({
        escrowAccount: escrowAccount2,
        tokenAMintAccount: tokenMintA,
        makerTokenAAccount: makerTokenAAccount,
        escrowTokenAVaultAccount: escrowTokenAVaultAccount2,
        maker: maker,
        fundingAccount: fundingAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([makerKeypair, fundingAccountKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("escrow state collect fee A", async () => {
    const escrowFeeAccount = anchor.utils.token.associatedAddress({
      mint: tokenMintA,
      owner: escrowState
    });
    const managerTokenAccount = anchor.utils.token.associatedAddress({
      mint: tokenMintA,
      owner: escrowManager
    });

    // Add your test here.
    const tx = await program.methods
      .collectFee()
      .accounts({
        escrowState: escrowState,
        escrowManager: escrowManager,
        tokenMintAccount: tokenMintA,
        escrowFeeAccount: escrowFeeAccount,
        managerTokenAccount: managerTokenAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([escrowManagerKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("escrow state collect fee B", async () => {
    const escrowFeeAccount = anchor.utils.token.associatedAddress({
      mint: tokenMintB,
      owner: escrowState
    });
    const managerTokenAccount = anchor.utils.token.associatedAddress({
      mint: tokenMintB,
      owner: escrowManager
    });
    const tx = await program.methods
      .collectFee()
      .accounts({
        escrowState: escrowState,
        escrowManager: escrowManager,
        tokenMintAccount: tokenMintB,
        escrowFeeAccount: escrowFeeAccount,
        managerTokenAccount: managerTokenAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([escrowManagerKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
