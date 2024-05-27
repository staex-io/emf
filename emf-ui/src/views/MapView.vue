<script>
import * as L from 'leaflet'

export default {
  data() {
    return {
      cellTowerOpened: false,
      id: '[VantageTowers] Tower#1',
    }
  },
  mounted() {
    const southWest = L.latLng(55.229, 3.779),
      northEast = L.latLng(47.339, 15.667),
      bounds = L.latLngBounds(southWest, northEast)
    const map = L.map('map', {
      attributionControl: false,
      maxBounds: bounds,
      doubleClickZoom: false,
    }).setView([52.523, 13.381], 10)
    L.tileLayer('https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png', {
      minZoom: 8,
      maxZoom: 12,
    }).addTo(map)

    const towerIcon = L.icon({
      iconUrl: 'tower.svg',
      iconSize: [50, 50],
    })
    L.marker([52.523, 13.381], {
      icon: towerIcon,
      riseOnHover: true,
      riseOffset: 250,
    })
      .addTo(map)
      .on('click', () => {
        this.cellTowerOpened = true
        L.popup([52.523, 13.381], {
          content: 'You see this tower card',
        })
          .on('remove', () => {
            this.cellTowerOpened = false
          })
          .openOn(map)
      })
  },
}
</script>

<template>
  <div class="float-container">
    <div id="map" />
    <div class="float-card">
      <div v-if="cellTowerOpened" class="card card-static">
        <div class="card-header">Cell Tower</div>
        <div class="card-content">
          <div class="card-field">
            <span class="card-field-label">ID</span>
            <span class="card-field-value">{{ id }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
#map {
  height: 90vh;
  width: 100%;
  margin: 1px 0 0 0;
  border-radius: 10px;
}
</style>
