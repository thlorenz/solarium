import { AddressLabels, airdrop, LOCALHOST } from '@metaplex-foundation/amman'
import {
  AccountMeta,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'

import { PROGRAM_ID } from '../.ammanrc.js'
const programId = new PublicKey(PROGRAM_ID)

function accountMeta(pubkey: PublicKey, isSigner = false): AccountMeta {
  return { pubkey, isWritable: true, isSigner }
}

const addressLabels = new AddressLabels(
  { vault: PROGRAM_ID },
  console.log,
  process.env.ADDRESS_LABEL_PATH
)

type InstructionData = {
  vaultBumpSeed: number
  lamports: beet.bignum
}

const instructionDataStruct = new beet.BeetArgsStruct<InstructionData>([
  ['vaultBumpSeed', beet.u8],
  ['lamports', beet.u64],
])

async function setupPayer(
  connection: Connection
): Promise<[PublicKey, Keypair]> {
  const [payerPubkey, payer] = addressLabels.genKeypair('payer')
  console.error('Airdropping payer')
  await airdrop(connection, payerPubkey, 1)
  return [payerPubkey, payer]
}

async function main() {
  const connection = new Connection(LOCALHOST, 'confirmed')
  const [payerPubkey, payer] = await setupPayer(connection)

  console.error('Finding PDA for vault')
  const [vaultPubkey, vaultBumpSeed] = await PublicKey.findProgramAddress(
    [Buffer.from('vault'), payerPubkey.toBuffer()],
    programId
  )
  addressLabels.addLabel('vaultPubkey', vaultPubkey)

  const vaultSize = 1024
  const lamports = await connection.getMinimumBalanceForRentExemption(vaultSize)

  const ixData = { vaultBumpSeed, lamports }
  const keys = [
    accountMeta(payerPubkey, true),
    accountMeta(vaultPubkey, false),
    accountMeta(SystemProgram.programId, false),
  ]

  const ix = new TransactionInstruction({
    programId,
    keys,
    data: instructionDataStruct.serialize(ixData)[0],
  })

  const { blockhash } = await connection.getRecentBlockhash()
  const tx = new Transaction({
    recentBlockhash: blockhash,
    feePayer: payerPubkey,
  }).add(ix)

  const sig = await connection.sendTransaction(tx, [payer])
  await connection.confirmTransaction(sig, 'confirmed')

  console.error(`Transaction ${sig} succeeded`)
}

main()
  .then(() => process.exit(0))
  .catch((err: any) => {
    console.error(err)
    process.exit(1)
  })
