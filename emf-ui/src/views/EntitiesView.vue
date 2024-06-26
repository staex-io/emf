<script>
import router from '@/router'

import { initializeApiContract } from '@/smart-contract.js'
import { contractTx } from '@scio-labs/use-inkathon'
import { checkWallet } from '@/signer-extension'
import { mnemonicGenerate } from '@polkadot/util-crypto'
import { Keyring } from '@polkadot/keyring'

export default {
  data() {
    return {
      signerWallet: null,
      signerAccount: null,
      entityInitialized: false,
      states: Object.freeze({
        INIT: 0,
        CREATE_SUB_ENTITY: 1,
      }),
      state: 0,
      newSubEntity: '',
      newSubEntityMnemonic: '',
      newLocation: '',
      subEntities: [],
      readyToIssue: new Map(),
      issued: new Map(),
    }
  },
  async mounted() {
    await this.initialize()
  },
  methods: {
    async initialize() {
      try {
        const { wallet, account } = await checkWallet()
        if (!wallet) throw 'Wallet is not connected!'
        if (!account) throw 'Account is not connected!'
        this.signerWallet = wallet
        this.signerAccount = account
      } catch (e) {
        alert(e)
        return
      }
      {
        // Fetch entities.
        const res = await fetch(`/indexer/entities`, { method: 'GET' })
        switch (res.status) {
          case 200:
            break
          default:
            throw 'invalid response status code'
        }
        const entities = await res.json()
        let found = false
        for (const entity of entities) {
          if (entity.account_id === this.signerAccount.address) {
            found = true
            break
          }
        }
        if (found) {
          this.entityInitialized = true
        }
      }
      {
        // Fetch sub-entities.
        const res = await fetch(`/indexer/sub-entities?account_id=${this.signerAccount.address}`, {
          method: 'GET',
        })
        switch (res.status) {
          case 200:
            break
          default:
            throw 'invalid response status code'
        }
        this.subEntities = await res.json()
      }
      {
        // Fetch ready certificates.
        for (const subEntity of this.subEntities) {
          const res = await fetch(
            `/indexer/ready-certificates?account_id=${subEntity.account_id}`,
            {
              method: 'GET',
            },
          )
          switch (res.status) {
            case 200:
              break
            default:
              throw 'invalid response status code'
          }
          const data = await res.json()
          if (data.length !== 0) {
            this.readyToIssue.set(data[0].sub_entity, null)
          }
        }
      }
      {
        // Fetch issued certificates.
        for (const subEntity of this.subEntities) {
          const res = await fetch(
            `/indexer/issued-certificates?account_id=${subEntity.account_id}`,
            {
              method: 'GET',
            },
          )
          switch (res.status) {
            case 200:
              break
            default:
              throw 'invalid response status code'
          }
          const data = await res.json()
          if (data.length !== 0) {
            this.issued.set(data[0].sub_entity, null)
          }
        }
      }
    },
    async initEntity() {
      const { api, contract } = await initializeApiContract()
      api.setSigner(this.signerWallet.signer)
      try {
        // This method can also override current value for provided key.
        // If you try to update key with the same value as record already has,
        // there is no fee for such transaction.
        await contractTx(api, this.signerAccount.address, contract, 'create_entity', {}, [], null)
      } catch (e) {
        if (e.errorMessage == 'TokenBelowMinimum') alert('Not enough tokens to proceed.')
        else {
          alert('Some error occurred during smart contract call. See console for details.')
          console.error(e)
        }
        return
      }
      this.entityInitialized = true
    },
    showSubEntityCreationForm() {
      this.state = this.states.CREATE_SUB_ENTITY
    },
    cancelSubEntityCreationForm() {
      this.state = this.states.INIT
    },
    async createSubEntity() {
      const { api, contract } = await initializeApiContract()
      api.setSigner(this.signerWallet.signer)
      try {
        // This method can also override current value for provided key.
        // If you try to update key with the same value as record already has,
        // there is no fee for such transaction.
        await contractTx(
          api,
          this.signerAccount.address,
          contract,
          'create_sub_entity',
          {},
          [this.newSubEntity, this.newLocation],
          null,
        )
      } catch (e) {
        if (e.errorMessage == 'TokenBelowMinimum') alert('Not enough tokens to proceed.')
        else {
          alert('Some error occurred during smart contract call. See console for details.')
          console.error(e)
        }
        return
      }
      alert('Cell tower successfully created!')
      this.state = this.states.INIT
      await this.initialize()
    },
    async issueCertificate(subEntity) {
      const { api, contract } = await initializeApiContract()
      api.setSigner(this.signerWallet.signer)
      try {
        // This method can also override current value for provided key.
        // If you try to update key with the same value as record already has,
        // there is no fee for such transaction.
        await contractTx(
          api,
          this.signerAccount.address,
          contract,
          'issue_certificate',
          {},
          [subEntity],
          null,
        )
      } catch (e) {
        if (e.errorMessage == 'TokenBelowMinimum') alert('Not enough tokens to proceed.')
        if (e.errorMessage == 'NotEnoughRecords')
          alert('Not enough measurement records to proceed.')
        else {
          alert('Some error occurred during smart contract call. See console for details.')
          console.error(e)
        }
        return
      }
      alert('Cell tower certificate successfully issued!')
      this.readyToIssue.delete(subEntity)
      this.issued.set(subEntity, null)
    },
    generateKeyPair() {
      const mnemonic = mnemonicGenerate()
      this.newSubEntityMnemonic = mnemonic
      const keyring = new Keyring({ type: 'sr25519', ss58Format: 42 })
      const keypair = keyring.addFromUri(mnemonic, {}, 'sr25519')
      this.newSubEntity = keypair.address
    },
    goToCellTower(location) {
      router.push({
        name: 'map-precise-location',
        params: {
          location,
        },
      })
    },
  },
}
</script>

<template>
  <div class="container">
    <div>
      <p
        v-if="signerWallet === null || signerAccount === null"
        class="error alert"
        style="margin: 0"
      >
        There are no connected wallets (see signer extension request).
      </p>
    </div>
    <div
      v-if="signerWallet && signerAccount && !entityInitialized"
      style="text-align: center; margin: 25px"
      class="item"
    >
      <button style="width: 100%; padding: 50px" @click="initEntity">
        Initialize cell company
      </button>
    </div>
    <div v-if="state === states.CREATE_SUB_ENTITY" style="margin: 50px">
      <label for="subEntity">Cell tower address</label>
      <input
        id="subEntity"
        v-model="newSubEntity"
        type="text"
        name="subEntity"
        style="margin-bottom: 25px"
        placeholder="5CS3ZHVZRSKckfQ583aCszSsMiJ6F32kNUGgxTvzdTpdcrCh"
      />
      <label for="location">Cell tower location (lat,lng)</label>
      <input
        id="location"
        v-model="newLocation"
        type="text"
        name="location"
        placeholder="52.4338,13.6505"
        style="margin-bottom: 25px"
      />
      <div v-if="newSubEntityMnemonic !== ''">
        Generated cell tower secret mnemonic:
        <br />
        <code>{{ newSubEntityMnemonic }}</code>
      </div>
      <div class="one-line-container">
        <button style="width: 100%; padding: 25px" @click="createSubEntity">
          Create cell tower
        </button>
        <button style="width: 100%; padding: 25px; margin: 25px" @click="generateKeyPair">
          Generate key pair
        </button>
        <button style="width: 100%; padding: 25px" @click="cancelSubEntityCreationForm">
          Cancel
        </button>
      </div>
    </div>
    <div v-if="entityInitialized">
      <div class="item">
        <button
          v-if="state !== states.CREATE_SUB_ENTITY"
          style="width: 100%; padding: 25px"
          @click="showSubEntityCreationForm"
        >
          Add cell tower
        </button>
      </div>
      <h1 v-if="subEntities.length === 0">There are not cell towers at the moment.</h1>
      <div v-else>
        <div class="item">
          <h1>Owned cell towers</h1>
        </div>
        <div class="item">
          <table style="margin: 0 25px 0 25px">
            <thead>
              <tr>
                <th>Address</th>
                <th>Location</th>
                <th />
              </tr>
            </thead>
            <tbody>
              <tr v-for="{ account_id, location } in subEntities" :key="account_id">
                <td class="mouse-pointer" @click="() => goToCellTower(location)">
                  {{ account_id }}
                </td>
                <td>
                  <a :href="`https://www.google.com/maps/place/${location}`" target="_blank">
                    {{ location }}
                  </a>
                </td>
                <td style="text-align: right">
                  <button
                    v-if="readyToIssue.get(account_id) === null"
                    class="action-btn"
                    @click="() => issueCertificate(account_id)"
                  >
                    Issue
                  </button>
                  <span v-else-if="issued.get(account_id) === null">Issued</span>
                  <span v-else>Not ready</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.one-line-container {
  display: flex;
  justify-content: left;
  align-items: center;
}

.action-btn {
  padding: 2px 25px 2px 25px;
}
</style>
