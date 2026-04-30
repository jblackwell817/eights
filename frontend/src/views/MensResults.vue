<script>
import { formatCollegeName } from '@/utils/formatters'

export default {
  data() {
    return {
      results: [],
    }
  },

  methods: {
    formatCollegeName,
  },

  async mounted() {
    const res = await fetch('/api/results/men')
    this.results = await res.json()
  },
}
</script>

<template>
  <div class="p-4">
    <h1 class="text-xl font-bold">Men’s Results</h1>
    <ul>
      <li v-for="(r, i) in results" :key="i">
        {{ r.year }}:
        <ul>
          <li v-for="(s, j) in r.standings" :key="j">
            {{ formatCollegeName(s.college) }} {{ s.boat }}
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
