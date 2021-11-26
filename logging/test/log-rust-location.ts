import {
  AccountMeta,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js'

const LOCALHOST = 'http://127.0.0.1:8899'
const keypair = Keypair.fromSecretKey(
  Uint8Array.from(require('../../target/deploy/sol_logging-keypair.json'))
)
const programId = keypair.publicKey

function accountMeta(
  pubkey: PublicKey,
  isWritable = false,
  isSigner = false
): AccountMeta {
  return { pubkey, isWritable, isSigner }
}

async function airdrop(connection: Connection, publicKey: PublicKey, sol = 1) {
  const sig = await connection.requestAirdrop(publicKey, sol * LAMPORTS_PER_SOL)
  return connection.confirmTransaction(sig)
}

async function main() {
  const payer = Keypair.generate()
  const logger = Keypair.generate()

  const connection = new Connection(LOCALHOST, 'processed')

  await airdrop(connection, payer.publicKey, 2)

  const transaction = new Transaction()

  const createLoggerIx = SystemProgram.createAccount({
    programId,
    space: 5,
    lamports: LAMPORTS_PER_SOL,
    fromPubkey: payer.publicKey,
    newAccountPubkey: logger.publicKey,
  })
  const setupLogIx = new TransactionInstruction({
    programId,
    keys: [
      accountMeta(payer.publicKey, false, true),
      accountMeta(logger.publicKey, true),
    ],
    data: Buffer.from([0]),
  })

  const logLocationIx = new TransactionInstruction({
    programId,
    keys: [
      accountMeta(payer.publicKey, false, true),
      accountMeta(logger.publicKey, true),
    ],
    data: Buffer.from([1]),
  })

  transaction.add(createLoggerIx, setupLogIx, logLocationIx)
  await connection.sendTransaction(transaction, [payer, logger])
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err)
    process.exit(1)
  })
