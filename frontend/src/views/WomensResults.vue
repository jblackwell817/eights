<script>
export default {
  data() {
    return {
      results: [],
    }
  },

  methods: {},

  async mounted() {
    const res = await fetch('/api/results/women')
    this.results = await res.json()
  },
}
</script>

<template>
  <div class="p-4">
    <h1 class="text-xl font-bold">Women’s Results</h1>
    <ul>
      <li v-for="(r, i) in results" :key="i">
        {{ r.year }}:
        <ul>
          <li v-for="(s, j) in r.standings" :key="j">{{ s.college }} {{ s.boat }}</li>
        </ul>
      </li>
    </ul>
  </div>
</template>
