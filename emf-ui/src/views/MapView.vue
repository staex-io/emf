<script>
import * as L from 'leaflet'

import { ContractPromise } from '@polkadot/api-contract'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { contractQuery } from '@scio-labs/use-inkathon'
import metadata from '@/assets/emf_contract.metadata.json'

export default {
  data() {
    return {
      activeCellTower: null,
      certificateLoaded: false,
      certificate: null,
    }
  },
  mounted() {
    // Restrict to go further than Germany on the map.
    const southWest = L.latLng(55.229, 3.779),
      northEast = L.latLng(47.339, 15.667),
      bounds = L.latLngBounds(southWest, northEast)
    // Create map itself.
    const map = L.map('map', {
      attributionControl: false,
      maxBounds: bounds,
      doubleClickZoom: false,
    }).setView([52.523, 13.381], 10)
    L.tileLayer('https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png', {
      minZoom: 8,
      maxZoom: 12,
    }).addTo(map)
    // Adjust map height.
    const headerHeight = document.querySelector('#header').offsetHeight
    document.querySelector('#map').style.height = `calc(100vh - ${headerHeight}px)`
    // Initialize icon.
    const towerIcon = L.icon({
      iconUrl: '/tower.svg',
      iconSize: [50, 50],
    })
    // Fetch towers and show them.
    this.getTowers().then((towers) => {
      for (const tower of towers) {
        const coordinates = tower.location.split(',')
        const lat = coordinates[0]
        const lng = coordinates[1]
        L.marker([lat, lng], {
          icon: towerIcon,
          riseOnHover: true,
          riseOffset: 250,
        })
          .addTo(map)
          .on('click', () => {
            L.popup([lat, lng], {
              content: 'You see this tower card',
            })
              .on('add', () => {
                this.activeCellTower = {
                  entity: tower.entity,
                  accountId: tower.account_id,
                }
                setTimeout(() => {
                  this.loadCertificate(tower.account_id).then((certificate) => {
                    this.certificateLoaded = true
                    setTimeout(() => {
                      this.fetchCertificate(certificate[0].c_index)
                    }, 1500)
                  })
                }, 1500)
              })
              .on('remove', () => {
                this.activeCellTower = null
              })
              .openOn(map)
          })
      }
    })
  },
  methods: {
    async getTowers() {
      const res = await fetch(`/indexer/sub-entities`, { method: 'GET' })
      switch (res.status) {
        case 200:
          break
        default:
          throw 'invalid response status code'
      }
      return await res.json()
    },
    async loadCertificate(towerAccountId) {
      const res = await fetch(`/indexer/issued-certificates?account_id=${towerAccountId}`, {
        method: 'GET',
      })
      switch (res.status) {
        case 200:
          break
        default:
          throw 'invalid response status code'
      }
      return await res.json()
    },
    async fetchCertificate(index) {
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
        '5GsVSYcLAWXEF5mKMnBJZaXLJAKMFMKNsqXHYDyAVgfbi5XV',
      )
      const { result, output } = await contractQuery(api, '', contract, 'fetch_certificate', {}, [
        index,
      ])
      if (!result.isOk) throw 'Error while fetching on-chain certificate.'
      this.certificate = output.toJSON().ok.ok
    },
  },
}
</script>

<template>
  <div class="float-container">
    <div id="map" />
    <div class="float-card">
      <div v-if="activeCellTower !== null" class="card card-static">
        <div class="card-header">Cell Tower</div>
        <div class="card-content">
          <div class="card-field">
            <span class="card-field-label">Entity</span>
            <span class="card-field-value">{{ activeCellTower.entity }}</span>
          </div>
          <div class="card-field">
            <span class="card-field-label">Account ID</span>
            <span class="card-field-value">{{ activeCellTower.accountId }}</span>
          </div>
          <hr style="margin-bottom: 15px" />
          <div v-if="certificate === null" class="card-field loader-container">
            <div class="loader" />
            <span v-if="!certificateLoaded">Loading certificate...</span>
            <span v-if="certificateLoaded">Fetching on-chain data...</span>
          </div>
          <div v-else>
            <div class="card-field loader-container">
              <img v-if="certificate.status === 'Ok'" alt="" src="/check.svg" style="width: 25px" />
              <img v-else alt="" src="/cross.svg" style="width: 25px" />
              &nbsp;{{ certificate.status }}
            </div>
            <div class="card-field">
              <span class="card-field-label">Avg measurement</span>
              <span class="card-field-value">{{ certificate.avgMeasurement }}</span>
            </div>
            <div class="card-field">
              <span class="card-field-label">Min measurement</span>
              <span class="card-field-value">{{ certificate.minMeasurement }}</span>
            </div>
            <div class="card-field">
              <span class="card-field-label">Max measurement</span>
              <span class="card-field-value">{{ certificate.maxMeasurement }}</span>
            </div>
            <div class="card-field">
              <span class="card-field-label">First measurement</span>
              <span class="card-field-value">
                {{ new Date(certificate.firstMeasurementTimestamp).toUTCString() }}
              </span>
            </div>
            <div class="card-field">
              <span class="card-field-label">Last measurement</span>
              <span class="card-field-value">
                {{ new Date(certificate.lastMeasurementTimestamp).toUTCString() }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
#map {
  height: 100vh;
  width: 100%;
  border-radius: 10px;
}

.loader-container {
  display: flex;
  justify-content: left;
  align-items: center;
}
</style>
