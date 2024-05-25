<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>
<script>
import router from '@/router'
export default {
  beforeMount() {
    router.push({ path: window.location.pathname })
  },
}
</script>

<template>
  <header>
    <nav>
      <a href="/devices">
        <img class="logo" alt="Staex logo" src="@/assets/logo-light.svg" />
      </a>
    </nav>
  </header>
  <RouterView />
</template>
