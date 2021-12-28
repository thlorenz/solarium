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
import fs from 'fs/promises'
import path from 'path'

import { PROGRAM_ID } from '../.ammanrc.js'
const programId = new PublicKey(PROGRAM_ID)
const VAULT_SIZE = 32 // PubkeyBytes

function accountMeta(pubkey: PublicKey, isSigner = false): AccountMeta {
  return { pubkey, isWritable: true, isSigner }
}

function accountMetaReadonly(pubkey: PublicKey, isSigner = false): AccountMeta {
  return { pubkey, isWritable: false, isSigner }
}

const addressLabels = new AddressLabels(
  { vault: PROGRAM_ID },
  console.log,
  process.env.ADDRESS_LABEL_PATH
)

type InstructionData = {
  instruction: number
  vaultBumpSeed: number
  lamports: beet.bignum
}

const instructionDataStruct = new beet.BeetArgsStruct<InstructionData>([
  ['instruction', beet.u8],
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

async function initVault() {
  const connection = new Connection(LOCALHOST, 'confirmed')
  const [payerPubkey, payer] = await setupPayer(connection)

  console.error('Finding PDA for vault')
  const [vaultPubkey, vaultBumpSeed] = await PublicKey.findProgramAddress(
    [Buffer.from('vault'), payerPubkey.toBuffer()],
    programId
  )
  addressLabels.addLabel('vaultPubkey', vaultPubkey)

  const vaultSize = VAULT_SIZE
  const lamports = await connection.getMinimumBalanceForRentExemption(vaultSize)

  const ixData = {
    instruction: 0,
    vaultBumpSeed,
    lamports,
  }
  const keys = [
    accountMeta(payerPubkey, true),
    accountMeta(vaultPubkey, false),
    accountMetaReadonly(SystemProgram.programId, false),
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

  const payerBuf = JSON.stringify(Array.from(payer.secretKey))
  await fs.writeFile(path.join(__dirname, 'payer.json'), payerBuf)
  const vaultBuf = JSON.stringify(Array.from(vaultPubkey.toBytes()))
  await fs.writeFile(path.join(__dirname, 'vault.json'), vaultBuf)
}

async function withdrawFromVault(
  payer: Keypair,
  vaultPubkey: PublicKey,
  lamports: number
) {
  const connection = new Connection(LOCALHOST, 'confirmed')

  const ixData = {
    instruction: 1,
    vaultBumpSeed: 0, // not used
    lamports,
  }
  const keys = [
    accountMeta(payer.publicKey, true),
    accountMeta(vaultPubkey, false),
    accountMetaReadonly(SystemProgram.programId, false),
  ]

  const ix = new TransactionInstruction({
    programId,
    keys,
    data: instructionDataStruct.serialize(ixData)[0],
  })

  const { blockhash } = await connection.getRecentBlockhash()
  const tx = new Transaction({
    recentBlockhash: blockhash,
    feePayer: payer.publicKey,
  }).add(ix)

  const sig = await connection.sendTransaction(tx, [payer])
  await connection.confirmTransaction(sig, 'confirmed')

  console.error(`Transaction ${sig} succeeded`)
}

async function readJSON(file: string) {
  const p = path.join(__dirname, file)
  return JSON.parse(await fs.readFile(p, 'utf-8'))
}

async function main() {
  const args = process.argv.slice(2)
  const withdraw = args[0]
  if (withdraw == null) return initVault()

  const payerSecretKey = Uint8Array.from(await readJSON('./payer.json'))
  const payer = Keypair.fromSecretKey(payerSecretKey)
  addressLabels.addLabel('payer', payer)

  const vaultBuffer = await readJSON('./vault.json')

  const vault = new PublicKey(vaultBuffer)
  addressLabels.addLabel('vault', vault)

  return withdrawFromVault(payer, vault, parseInt(withdraw))
}

main()
  .then(() => process.exit(0))
  .catch((err: any) => {
    console.error(err)
    process.exit(1)
  })
