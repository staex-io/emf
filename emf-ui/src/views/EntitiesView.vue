<script>
import { ContractPromise } from '@polkadot/api-contract'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { contractTx } from '@scio-labs/use-inkathon'
import metadata from '@/assets/emf_contract.metadata.json'
import { web3Enable, web3Accounts, web3FromAddress } from '@polkadot/extension-dapp'

export default {
  data() {
    return {
      signerAccount: null,
      entityInitialized: false,
    }
  },
  async mounted() {
    const signerExtensions = await new web3Enable('EMF')
    if (signerExtensions.length === 0) {
      alert('Please, connect signer extension to use the website!')
      return
    }
    const signerAccounts = await web3Accounts({})
    if (signerAccounts.length === 0) {
      alert('Please, connect signer extension to use the website!')
      return
    }
    this.signerAccount = signerAccounts[0]
    const res = await fetch(`/indexer/entities`, { method: 'GET' })
    switch (res.status) {
      case 200:
        break
      default:
        throw 'invalid response status code'
    }
    const accounts = await res.json()
    let found = false
    for (const account of accounts) {
      if (account.account_id === this.signerAccount.address) {
        found = true
        break
      }
    }
    if (found) {
      this.entityInitialized = true
      return
    }
  },
  methods: {
    async initEntity() {
      // todo: move to separate function and use in every view
      // Connect to Substrate and init API and contract.
      const provider = new WsProvider('ws://127.0.0.1:9944')
      const api = await ApiPromise.create({ provider })
      const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version(),
      ])
      console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`)
      const contract = new ContractPromise(
        api,
        metadata,
        '5GPGUPaCzQKHao1bQ5y9BybDzbpsbjAribjTQ3xSe1dcxJxe',
      )
      const injector = await web3FromAddress(this.signerAccount.address)
      api.setSigner(injector.signer)

      try {
        // This method can also override current value for provided key.
        // If you try to update key with the same value as record already has,
        // there is no fee for such transaction.
        await contractTx(api, this.signerAccount.address, contract, 'create_entity', {}, [], null)
      } catch (e) {
        if (e.errorMessage == 'TokenBelowMinimum') alert('Not enough tokens to proceed.')
        return
      }
    },
  },
}
</script>

<template>
  <div v-if="signerAccount" style="margin: 25px; width: fit-content">
    <h3>Connected account</h3>
    <input v-model="signerAccount.address" style="margin: 5px 0 5px 0" />
    <input v-model="signerAccount.meta.source" />
  </div>
  <hr />
  <div>
    <p v-if="signerAccount === null" class="error alert">
      There are no connected accounts (see signer extension request).
    </p>
  </div>
  <div v-if="!entityInitialized" style="text-align: center; margin: 25px">
    <button class="mouse-pointer" style="width: 100%; padding: 50px" @click="initEntity">
      Init entity
    </button>
  </div>
  <div v-else>
    <h1>Entities</h1>
  </div>
</template>

<style scoped></style>
