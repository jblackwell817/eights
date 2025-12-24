<script>
import { useRoute } from 'vue-router'
export default {
  props: {
    gender: String,
    college: String,
    boat: String,
  },

  data() {
    return {
      results: [],
    }
  },

  watch: {
    gender: 'fetchResults',
    college: 'fetchResults',
    boat: 'fetchResults',
  },

  methods: {
    async fetchResults() {
      const res = await fetch(`/api/results/${this.gender}/${this.college}/${this.boat}`)
      this.results = await res.json()
    },
  },

  async mounted() {
    this.fetchResults()
  },
}
</script>

<template>
  <div>
    <h1>{{ college }} {{ boat }}</h1>
    <li v-for="([year, place], i) in results.results" :key="i">{{ year }}: {{ place }}</li>
  </div>
</template>
