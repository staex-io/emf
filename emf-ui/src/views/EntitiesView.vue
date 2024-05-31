<script>
import { initializeApiContract } from '@/smart-contract.js'
import { contractTx } from '@scio-labs/use-inkathon'
import { web3FromAddress } from '@polkadot/extension-dapp'
import { initializeSigner } from '@/signer-extension'

export default {
  data() {
    return {
      signerAccount: null,
      entityInitialized: false,
    }
  },
  async mounted() {
    try {
      this.signerAccount = await initializeSigner()
    } catch (e) {
      alert(e)
      return
    }
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
      const { api, contract } = await initializeApiContract()
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
  <div v-if="signerAccount && !entityInitialized" style="text-align: center; margin: 25px">
    <button class="mouse-pointer" style="width: 100%; padding: 50px" @click="initEntity">
      Init entity
    </button>
  </div>
  <div v-if="entityInitialized">
    <h1>Entities</h1>
  </div>
</template>

<style scoped></style>
