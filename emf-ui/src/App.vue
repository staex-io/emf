<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>
<script>
import router from '@/router'

import { initializeSigner } from '@/signer-extension'

export default {
  data() {
    return {
      signer: null,
    }
  },
  async beforeMount() {
    router.push({ path: window.location.pathname })
    try {
      this.signer = await initializeSigner()
    } catch (e) {
      console.warn(e)
      return
    }
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
        <li>
          <RouterLink to="/map">Map</RouterLink>
          <RouterLink to="/entities">Cell towers</RouterLink>
        </li>
        <li v-if="signer" style="margin: 0">
          <span>{{ signer.address.slice(0, 5) }}</span>
          ..
          <span>{{ signer.address.slice(43) }}</span>
          &nbsp;(
          <span>{{ signer.meta.source }}</span>
          )
        </li>
      </ul>
    </nav>
  </header>
  <RouterView />
</template>
