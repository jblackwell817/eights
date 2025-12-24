<script>
import { useRoute } from 'vue-router'
export default {
  data() {
    return {
      results: [],
      route: useRoute(),
    }
  },

  methods: {},

  async mounted() {
    const { gender, college, boat } = this.route.params
    const res = await fetch(`/api/results/${gender}/${college}/${boat}`)
    this.results = await res.json()
  },
}
</script>

<template>
  <div>
    <h1>{{ route.params.college }} — {{ route.params.gender }} — {{ route.params.boat }}</h1>
    <li v-for="([year, place], i) in results.results" :key="i">{{ year }}: {{ place }}</li>
  </div>
</template>
