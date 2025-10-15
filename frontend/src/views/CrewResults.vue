<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const results = ref([])

onMounted(async () => {
  const { college, boat } = route.params
  const res = await fetch(`/results/men/${college}/${boat}`)
  results.value = await res.json()
})
</script>

<template>
  <div>
    <h1>{{ route.params.college }} — {{ route.params.boat }}</h1>
    <ul>
      <li v-for="(r, i) in results" :key="i">
        {{ r.place }} - {{ r.time }}
      </li>
    </ul>
  </div>
</template>
