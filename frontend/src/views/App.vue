<template>
  <div class="nav-wrapper">
    <!-- Men/Women toggle -->
    <nav class="toggle-buttons">
      <router-link
        to="#"
        class="btn"
        :class="{ active: activeGender === 'men' }"
        @click.prevent="selectGender('men')"
      >
        Men
      </router-link>

      <router-link
        to="#"
        class="btn"
        :class="{ active: activeGender === 'women' }"
        @click.prevent="selectGender('women')"
      >
        Women
      </router-link>
    </nav>

    <!-- Crew dropdown -->
    <select class="crew-select" v-model="selectedCrew" @change="onCrewChange">
      <option value="">All Crews</option>
      <option v-for="crew in availableCrews" :key="crew.id" :value="crew.id">
        {{ crew.name }}
      </option>
    </select>
  </div>

  <router-view />
</template>

<script>
import { mensCrews, womensCrews } from '@/data/crews.js'

export default {
  data() {
    return {
      mensCrews,
      womensCrews,
      selectedCrew: '',
      activeGender: null,
    }
  },
  computed: {
    currentGender() {
      if (this.$route.name === 'CrewResults') {
        return this.$route.params.gender
      }

      if (this.$route.name === 'WomensResults') return 'women'
      if (this.$route.name === 'MensResults') return 'men'
      return null
    },
    isMens() {
      return this.currentGender === 'men'
    },
    isWomens() {
      return this.currentGender === 'women'
    },
    availableCrews() {
      if (this.activeGender === 'women') return this.womensCrews
      if (this.activeGender === 'men') return this.mensCrews
      return []
    },
  },

  methods: {
    selectGender(gender) {
      this.selectedCrew = ''
      this.activeGender = gender
      this.$router.push({ name: gender === 'men' ? 'MensResults' : 'WomensResults' })
    },

    onCrewChange() {
      const crewId = this.selectedCrew

      if (!crewId) {
        this.$router.push({
          name: this.isWomens ? 'WomensResults' : 'MensResults',
        })
        return
      }

      const [college, boat] = crewId.split('_')

      this.$router.push({
        name: 'CrewResults',
        params: {
          gender: this.isWomens ? 'women' : 'men',
          college,
          boat,
        },
      })
    },
  },
}
</script>
