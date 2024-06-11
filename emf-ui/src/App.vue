<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>
<script>
import router from '@/router'

import {
  initializeWallets,
  initializeWallet,
  checkWallet,
  initializeAccounts,
  WALLET_STORAGE_KEY_NAME,
  ACCOUNT_STORAGE_KEY_NAME,
} from '@/signer-extension'

export default {
  data() {
    return {
      wallet: null,
      wallets: [],
      selectedWallet: null,
      account: null,
      accounts: [],
      selectedAccount: null,
      web3Injected: false,
    }
  },
  computed: {
    walletsAsArr() {
      return Array.from(this.wallets.entries())
    },
    accountsAsArr() {
      return Array.from(this.accounts.entries())
    },
  },
  watch: {
    async selectedWallet(index) {
      this.wallet = this.wallets[index]
      initializeWallet(this.wallet).then(() => {
        initializeAccounts(this.wallet).then((accounts) => (this.accounts = accounts))
      })
      sessionStorage.setItem(WALLET_STORAGE_KEY_NAME, JSON.stringify(this.wallet))
    },
    async selectedAccount(index) {
      this.account = this.accounts[index]
      sessionStorage.setItem(ACCOUNT_STORAGE_KEY_NAME, JSON.stringify(this.account))
    },
  },
  async beforeMount() {
    router.push({ path: window.location.pathname })
    const { wallet, account, isWeb3Injected } = await checkWallet()
    this.wallet = wallet
    this.account = account
    this.web3Injected = isWeb3Injected
    if (this.wallet && !this.account) {
      this.accounts = await initializeAccounts(this.wallet)
    }
  },
  methods: {
    async initWeb3() {
      try {
        this.wallets = await initializeWallets()
      } catch (e) {
        console.warn(e)
        return
      }
    },
    change() {
      sessionStorage.clear()
      window.location.reload()
    },
  },
}
</script>

<template>
  <!-- We need this id to adjust map height in MapView.vue -->
  <header id="header">
    <nav>
      <a href="/">
        <img class="logo" alt="Staex logo" src="/favicon.svg" />
      </a>
      <ul>
        <div class="line-container">
          <li>
            <RouterLink to="/map">Map</RouterLink>
            <RouterLink v-if="web3Injected" to="/entities">Cell towers</RouterLink>
          </li>
          <li v-if="!wallet && web3Injected && wallets.length === 0">
            <a class="mouse-pointer" @click="initWeb3">Connect wallet</a>
          </li>
          <li v-if="web3Injected && wallets.length !== 0 && !wallet">
            <select v-model="selectedWallet">
              <option disabled value="" selected>Select a wallet</option>
              <option v-for="[key, value] in walletsAsArr" :key="key" :value="key">
                {{ value.extensionName }}
              </option>
            </select>
          </li>
          <li v-if="web3Injected && wallet && !account">
            <select v-model="selectedAccount">
              <option disabled value="" selected>Select an account</option>
              <option v-for="[key, value] in accountsAsArr" :key="key" :value="key">
                {{ value.name }} ({{ wallet.extensionName }})
              </option>
            </select>
          </li>
          <li v-if="wallet && account" style="margin: 0">
            {{ account.name }} ({{ wallet.extensionName }})
          </li>
          <li v-if="wallet && account">
            <button style="padding: 5px 10px" @click="change">Change</button>
          </li>
        </div>
      </ul>
    </nav>
  </header>
  <RouterView />
</template>
