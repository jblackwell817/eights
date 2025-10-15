<script setup>
import { ref, onMounted } from 'vue'
const results = ref([])

onMounted(async () => {
  const res = await fetch('/results/women')
  results.value = await res.json()
})
</script>

<template>
  <div class="p-4">
    <h1 class="text-xl font-bold">Women’s Results</h1>
    <ul>
      <li v-for="(r, i) in results" :key="i">
        {{ r.college }} - {{ r.boat }} - {{ r.time }}
      </li>
    </ul>
  </div>
</template>
