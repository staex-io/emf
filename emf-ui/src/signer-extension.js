import { web3Enable, web3Accounts } from '@polkadot/extension-dapp'

export const initializeSigner = async () => {
  const signerExtensions = await new web3Enable('EMF')
  if (signerExtensions.length === 0) {
    throw 'Please, connect signer extension to use the website!'
  }
  const signerAccounts = await web3Accounts({})
  if (signerAccounts.length === 0) {
    throw 'Please, connect signer extension to use the website!'
  }
  const signerAccount = signerAccounts[0]
  return signerAccount
}
