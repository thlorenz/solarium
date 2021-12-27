// @ts-check
'use strict'
const path = require('path')
const PROGRAM_ID = '6ds1BgdmEDDX74bNbpyw8Sm12vFasGL4wqKcbv1wuwDp'

const deployPath = path.resolve(__dirname, '../target/deploy/vault.so')

module.exports = {
  PROGRAM_ID,
  validator: {
    programs: [{ programId: PROGRAM_ID, deployPath }],
    commitment: 'confirmed',
  },
}
