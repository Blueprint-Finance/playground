import { setProvider, Program } from "@coral-xyz/anchor";
import { BankrunTest } from "../target/types/bankrun_test";
import {
  AccountInfoBytes,
  AddedAccount,
  BanksClient,
  BanksTransactionResultWithMeta,
  ProgramTestContext,
  startAnchor
} from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { expect } from "chai";
import {
  PublicKey,
  Transaction,
  Keypair,
  Connection,
  clusterApiUrl,
  TransactionInstruction
} from "@solana/web3.js";
import {
  ACCOUNT_SIZE,
  AccountLayout,
  getAssociatedTokenAddressSync,
  MintLayout,
  TOKEN_PROGRAM_ID
} from "@solana/spl-token";

const IDL = require("../target/idl/bankrun_test.json");
// Constants
const PROJECT_DIRECTORY = ""; // Leave empty if using default anchor project
const USDC_DECIMALS = 6;
const USDC_MINT_ADDRESS = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const MINIMUM_SLOT = 100;
const MINIMUM_USDC_BALANCE = 100_000_000_000; // 100k USDC

async function createAndProcessTransaction(
  client: BanksClient,
  payer: Keypair,
  instruction: TransactionInstruction,
  additionalSigners: Keypair[] = []
): Promise<BanksTransactionResultWithMeta> {
  const tx = new Transaction();
  const [latestBlockhash] = await client.getLatestBlockhash();
  tx.recentBlockhash = latestBlockhash;
  tx.add(instruction);
  tx.feePayer = payer.publicKey;
  tx.sign(payer, ...additionalSigners);
  return await client.tryProcessTransaction(tx);
}

async function setupATA(
  context: ProgramTestContext,
  usdcMint: PublicKey,
  owner: PublicKey,
  amount: number
): Promise<PublicKey> {
  const tokenAccData = Buffer.alloc(ACCOUNT_SIZE);
  AccountLayout.encode(
    {
      mint: usdcMint,
      owner,
      amount: BigInt(amount),
      delegateOption: 0,
      delegate: PublicKey.default,
      delegatedAmount: BigInt(0),
      state: 1,
      isNativeOption: 0,
      isNative: BigInt(0),
      closeAuthorityOption: 0,
      closeAuthority: PublicKey.default,
    },
    tokenAccData,
  );

  const ata = getAssociatedTokenAddressSync(usdcMint, owner, true);
  const ataAccountInfo = {
    lamports: 1_000_000_000,
    data: tokenAccData,
    owner: TOKEN_PROGRAM_ID,
    executable: false,
  };

  context.setAccount(ata, ataAccountInfo);
  return ata;
}

describe("Bankrun Tests", () => {
  const usdcMint = new PublicKey(USDC_MINT_ADDRESS);
  let context: ProgramTestContext;
  let client: BanksClient;
  let payer: Keypair;
  let provider: BankrunProvider;
  let program: Program<BankrunTest>;

  before(async () => {
    const connection = new Connection(clusterApiUrl("mainnet-beta"));
    const accountInfo = await connection.getAccountInfo(usdcMint);
    const usdcAccount: AddedAccount = { address: usdcMint, info: accountInfo };

    context = await startAnchor(PROJECT_DIRECTORY, [], [usdcAccount]);
    client = context.banksClient;
    payer = context.payer;
    provider = new BankrunProvider(context);
    setProvider(provider);
    program = new Program<BankrunTest>(IDL, provider);
  });

  describe("Time Travel Tests", () => {
    const testCases = [
      { desc: "(too early)", slot: MINIMUM_SLOT - 1, shouldSucceed: false },
      { desc: "(at or above threshold)", slot: MINIMUM_SLOT, shouldSucceed: true },
    ]
    testCases.forEach(({ desc, slot, shouldSucceed }) => {
      describe(`When slot is ${slot} ${desc}`, () => {
        let txResult: BanksTransactionResultWithMeta;
        let newData: Keypair;
        let dataAccount: Keypair;

        before(async () => {
          provider.context.warpToSlot(BigInt(slot));
          newData = Keypair.generate();
          dataAccount = Keypair.generate();
          const ix = await program.methods
            .setData()
            .accounts({
              payer: payer.publicKey,
              newData: newData.publicKey,
              dataAccount: dataAccount.publicKey,
            })
            .signers([newData, dataAccount])
            .instruction();
          txResult = await createAndProcessTransaction(client, payer, ix, [newData, dataAccount]);
        });

        if (!shouldSucceed) {
          it("transaction should fail", () => {
            expect(txResult.result).to.exist;
          });

          it("should contain specific error details in log", () => {
            const errorLog = txResult.meta.logMessages.find(log =>
              log.includes('AnchorError') &&
              log.includes('InvalidSlot') &&
              log.includes('6000') &&
              log.includes('Error Message: Invalid slot')
            );
            expect(errorLog).to.exist;
          });

          it("last log message should indicate failure", () => {
            expect(txResult.meta.logMessages[txResult.meta.logMessages.length - 1]).to.include('failed');
          });
        } else {
          it("transaction should succeed", () => {
            expect(txResult.result).to.be.null;
          });

          it("last log message should indicate success", () => {
            expect(txResult.meta.logMessages[txResult.meta.logMessages.length - 1]).to.include('success');
          });

          it("should contain expected log message", () => {
            const expectedLog = "Set new data: " + newData.publicKey.toString();
            const foundLog = txResult.meta.logMessages.some(log => log.includes(expectedLog));
            expect(foundLog).to.be.true;
          });

          it("should set new data correctly", async () => {
            const onChainData = await program.account.dataAccount.fetch(dataAccount.publicKey);
            expect(onChainData.newData.toString()).to.equal(newData.publicKey.toString());
          });
        }
      });
    });
  });

});