<script>
import * as L from 'leaflet'

export default {
  mounted() {
    const southWest = L.latLng(55.229, 3.779),
      northEast = L.latLng(47.339, 15.667),
      bounds = L.latLngBounds(southWest, northEast)
    const map = L.map('map', {
      attributionControl: false,
      maxBounds: bounds,
    }).setView([52.523, 13.381], 10)
    L.tileLayer('https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png', {
      minZoom: 8,
      maxZoom: 12,
    }).addTo(map)

    const towerIcon = L.icon({
      iconUrl: 'tower.svg',
      iconSize: [30, 30],
    })
    L.marker([52.523, 13.381], { icon: towerIcon }).addTo(map).bindPopup('VantageTowers Tower#1')
  },
}
</script>

<template>
  <div id="map" />
</template>

<style scoped>
#map {
  height: 90vh;
  width: 100%;
  margin: 1px 0 0 0;
  border-radius: 10px;
}
</style>
