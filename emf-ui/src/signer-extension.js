import { getWallets } from '@talismn/connect-wallets'

export const WALLET_STORAGE_KEY_NAME = 'wallet'
export const ACCOUNT_STORAGE_KEY_NAME = 'account'

export const checkWallet = async () => {
  const installedWallets = getWallets().filter((wallet) => wallet.installed)
  if (installedWallets.length === 0) return { wallet: null, account: null, isWeb3Injected: false }
  const storedWallet = JSON.parse(sessionStorage.getItem(WALLET_STORAGE_KEY_NAME))
  if (storedWallet === null) return { wallet: null, account: null, isWeb3Injected: true }
  const storedAccount = JSON.parse(sessionStorage.getItem(ACCOUNT_STORAGE_KEY_NAME))
  const wallet = installedWallets.find(
    (wallet) => wallet.extensionName === storedWallet.extensionName,
  )
  await initializeWallet(wallet)
  return { wallet, account: storedAccount, isWeb3Injected: true }
}

export const initializeWallet = async (wallet) => {
  await wallet.enable('EMF')
}

export const initializeWallets = async () => {
  const installedWallets = getWallets().filter((wallet) => wallet.installed)
  return installedWallets
}

export const initializeAccounts = async (wallet) => {
  const accounts = await wallet.getAccounts()
  return accounts
}
