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
      <a href="/">
        <img class="logo" alt="Staex logo" src="/favicon.svg" />
      </a>
      <ul>
        <li>
          <RouterLink to="/map"> Map </RouterLink>
          <RouterLink to="/entities"> Entities </RouterLink>
        </li>
      </ul>
    </nav>
  </header>
  <RouterView />
</template>
