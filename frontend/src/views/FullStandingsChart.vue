<template>
  <div class="standings-chart">
    <div class="controls">
      <input
        v-model="filter"
        type="text"
        placeholder="Filter by college"
        @input="updateChart"
      />
    </div>
    <div class="chart-scroll-wrapper" @click.self="clearTooltip">
      <div class="chart-inner" :style="{ height: chartHeight + 'px', minWidth: 'min-content' }">
        <canvas ref="chartCanvas" role="img" aria-label="Horizontal line chart showing Oxford Summer Eights standings by position over years, years on vertical axis">
          Standings over time for all crews in Oxford Summer Eights.
        </canvas>
        <div
          v-if="clickedCrew && tooltipPos"
          class="crew-tooltip"
          :style="{ top: tooltipPos.y + 'px', left: tooltipPos.x + 'px' }"
        >
          <div class="crew-tooltip-name">{{ clickedCrew.name }}</div>
          <div class="crew-tooltip-detail">{{ clickedCrew.year }} · Position: {{ clickedCrew.position }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount, watch } from 'vue'
import Chart from 'chart.js/auto'
import { API_URL } from '../config'

const props = defineProps({
  gender: {
    type: String,
    required: true
  }
})

const PALETTE = [
  '#3266ad','#c0392b','#27ae60','#8e44ad','#d35400','#16a085',
  '#2c3e50','#f39c12','#1abc9c','#e74c3c','#9b59b6','#2980b9',
  '#e67e22','#34495e','#7f8c8d','#a93226','#117a65','#1d8348',
  '#6c3483','#935116','#1a5276','#784212','#515a5a','#0b5345'
]

const COLLEGE_NAME_MAP = {
  Balliol: 'Balliol',
  Brasenose: 'Brasenose',
  BrasenoseStPeters: 'Brasenose & St Peters',
  ChristChurch: 'Christ Church',
  CorpusChristi: 'Corpus Christi',
  Exeter: 'Exeter',
  GreenTempleton: 'Green Templeton',
  HarrisManchester: 'Harris Manchester',
  Hertford: 'Hertford',
  Jesus: 'Jesus',
  Keble: 'Keble',
  LadyMargaretHall: 'Lady Margaret Hall',
  Linacre: 'Linacre',
  Lincoln: 'Lincoln',
  Magdalen: 'Magdalen',
  Mansfield: 'Mansfield',
  Merton: 'Merton',
  New: 'New',
  Oriel: 'Oriel',
  OslerGreen: 'Osler Green',
  OslerHouse: 'Osler House',
  Pembroke: 'Pembroke',
  Queens: "Queen's",
  RegentsPark: "Regent's Park",
  Reuben: 'Reuben',
  StAnnes: "St Anne's",
  StAnnesStHildas: "St Anne's & St Hilda's",
  StAntonys: "St Antony's",
  StBenetsHall: "St Benet's Hall",
  StCatherines: "St Catherine's",
  StEdmundHall: 'St Edmund Hall',
  StHildas: "St Hilda's",
  StHughs: "St Hugh's",
  StJohns: "St John's",
  StMaryHall: 'St Mary Hall',
  StPeters: "St Peter's",
  Somerville: 'Somerville',
  Trinity: 'Trinity',
  University: 'University',
  Wadham: 'Wadham',
  Westminster: 'Westminster',
  Wolfson: 'Wolfson',
  Worcester: 'Worcester',
}

const COLLEGE_COLOURS = {
  Balliol: '#E80C0C',
  Brasenose: '#E8D20C',
  BrasenoseStPeters: '#D6E80C',
  ChristChurch: '#09084A',
  CorpusChristi: '#A31C2A',
  Exeter: '#E81A1A',
  GreenTempleton: '#1A5C36',
  HarrisManchester: '#1C1A5C',
  Hertford: '#540929',
  Jesus: '#167015',
  Keble: '#C41A14',
  LadyMargaretHall: '#FAEA0F',
  Linacre: '#D9CC21',
  Lincoln: '#2B50B5',
  Magdalen: '#000000',
  Mansfield: '#EB1207',
  Merton: '#660E21',
  New: '#4E1280',
  Oriel: '#111759',
  OslerGreen: '#961423',
  OslerHouse: '#961423',
  Pembroke: '#EE7DF0',
  Queens: '#31579E',
  RegentsPark: '#ED150C',
  Reuben: '#116941',
  StAnnes: '#0C2185',
  StAnnesStHildas: '#0C2185',
  StAntonys: '#EDD015',
  StBenetsHall: '#C9B214',
  StCatherines: '#3BA9ED',
  StEdmundHall: '#993F51',
  StHildas: '#203187',
  StHughs: '#0B1547',
  StJohns: '#1934BD',
  StMaryHall: '#42C299',
  StPeters: '#20692F',
  Somerville: '#C71619',
  Trinity: '#201D69',
  University: '#BA992B',
  Wadham: '#149BE3',
  Westminster: '#082F45',
  Wolfson: '#D6B220',
  Worcester: '#D620A9',
}

const PX_PER_YEAR = 28

const cache = {}
const data = ref([])
const filter = ref('')
const chartCanvas = ref(null)
const clickedCrew = ref(null)
const tooltipPos = ref(null)
const selectedDatasetIndex = ref(null)
let chartInstance = null

onMounted(async () => {
  data.value = await loadData(props.gender)
  initChart()
  document.addEventListener('click', onDocumentClick)
})

onBeforeUnmount(() => {
  chartInstance?.destroy()
  document.removeEventListener('click', onDocumentClick)
})

watch(() => props.gender, async (newGender) => {
  data.value = []
  clickedCrew.value = null
  tooltipPos.value = null
  selectedDatasetIndex.value = null
  data.value = await loadData(newGender)
  initChart()
})

watch(data, () => initChart())

async function loadData(gender) {
  if (cache[gender]) return cache[gender]
  const res = await fetch(`${API_URL}/api/results/${gender}`)
  cache[gender] = await res.json()
  return cache[gender]
}

function onDocumentClick(evt) {
  if (!chartCanvas.value?.contains(evt.target)) {
    clickedCrew.value = null
    tooltipPos.value = null
    selectedDatasetIndex.value = null
    clearFade()
  }
}

function formatName(college, boat) {
  const name = COLLEGE_NAME_MAP[college] || college
  return `${name} ${boat}`
}

function processData(rawData) {
  const years = [...new Set(rawData.map(d => d.year))].sort((a, b) => a - b)
  const crewMap = {}
  rawData.forEach(yearData => {
    yearData.standings.forEach((entry, idx) => {
      const key = `${entry.college}_${entry.boat}`
      if (!crewMap[key]) {
        crewMap[key] = {
          label: formatName(entry.college, entry.boat),
          positions: {}
        }
      }
      crewMap[key].positions[yearData.year] = idx + 1
    })
  })
  return { years, crewMap }
}

function buildDatasets(crewMap, years, filterText) {
  return Object.entries(crewMap)
    .filter(([, crew]) => !filterText || crew.label.toLowerCase().includes(filterText.toLowerCase()))
    .map(([key, crew]) => {
      const college = key.split('_')[0]
      const color = COLLEGE_COLOURS[college] || '#888888'
      const pointColor = years.map(() => color)
      return {
        label: crew.label,
        data: years.map(y => crew.positions[y] ?? null),
        borderColor: color,
        backgroundColor: color,
        _color: color,
        pointBackgroundColor: pointColor,
        pointBorderColor: pointColor,
        borderWidth: 1.5,
        pointRadius: 2,
        pointHoverRadius: 2,
        pointHitRadius: 2,
        spanGaps: false,
        tension: 0.2,
      }
    })
}

const chartHeight = computed(() => {
  const years = [...new Set(data.value.map(d => d.year))]
  return Math.max(400, years.length * PX_PER_YEAR + 80)
})

function initChart() {
  if (!chartCanvas.value || !data.value.length) return
  const { years, crewMap } = processData(data.value)
  const maxPos = Math.max(...data.value.map(d => d.standings.length))
  const datasets = buildDatasets(crewMap, years, filter.value)

  if (chartInstance) chartInstance.destroy()

  chartInstance = new Chart(chartCanvas.value, {
    type: 'line',
    data: { labels: years, datasets },
    options: {
      indexAxis: 'y',
      responsive: true,
      maintainAspectRatio: false,
      animation: false,
      interaction: { mode: 'index', intersect: false },
      plugins: {
        legend: { display: false },
        tooltip: { enabled: false },
      },
      scales: {
        x: {
          position: 'top',
          min: 1,
          max: maxPos,
          title: { display: true, text: 'Position', font: { size: 12 }, color: '#888' },
          grid: { color: 'rgba(128,128,128,0.1)' },
          ticks: {
            stepSize: 1,
            callback: v => Number.isInteger(v) ? v : '',
            font: { size: 11 },
          },
        },
        y: {
          title: { display: true, text: 'Year', font: { size: 12 }, color: '#888' },
          grid: { color: 'rgba(128,128,128,0.08)' },
          ticks: { font: { size: 11 } },
        },
      },
      onClick: (evt) => {
        const els = chartInstance.getElementsAtEventForMode(evt.native, 'nearest', { intersect: true }, true)
        if (els.length === 0) {
          clickedCrew.value = null
          tooltipPos.value = null
          selectedDatasetIndex.value = null
          clearFade()
          return
        }
        const idx = els[0].datasetIndex
        const dataIndex = els[0].index
        const dataset = chartInstance.data.datasets[idx]
        const year = chartInstance.data.labels[dataIndex]
        const position = dataset.data[dataIndex]
        selectedDatasetIndex.value = idx
        clickedCrew.value = { name: dataset.label, year, position }
        tooltipPos.value = { x: evt.native.offsetX, y: evt.native.offsetY }
        applyFade(idx)
      },
    },
  })
}

function updateChart() {
  if (!chartInstance || !data.value.length) return
  const { years, crewMap } = processData(data.value)
  const datasets = buildDatasets(crewMap, years, filter.value)
  chartInstance.data.datasets = datasets
  selectedDatasetIndex.value = null
  clickedCrew.value = null
  tooltipPos.value = null
  chartInstance.update()
}

function clearTooltip() {
  clickedCrew.value = null
  tooltipPos.value = null
}

function applyFade(index) {
  if (!chartInstance) return
  chartInstance.data.datasets.forEach((ds, i) => {
    const active = i === index
    const color = ds._color
    const pointColor = Array(ds.data.length).fill(active ? color : color + '40')
    ds.borderColor = active ? color : color + '40'
    ds.pointBackgroundColor = pointColor
    ds.pointBorderColor = pointColor
  })
  chartInstance.update('none')
}

function clearFade() {
  if (!chartInstance) return
  chartInstance.data.datasets.forEach((ds) => {
    const color = ds._color
    const pointColor = Array(ds.data.length).fill(color)
    ds.borderColor = color
    ds.pointBackgroundColor = pointColor
    ds.pointBorderColor = pointColor
  })
  chartInstance.update('none')
}
</script>

<style scoped>
.standings-chart {
  font-family: sans-serif;
}

.controls {
  margin-bottom: 0.75rem;
}

.chart-scroll-wrapper {
  width: 100%;
  overflow-y: auto;
  overflow-x: auto;
}

.chart-inner {
  position: relative;
  width: 100%;
  min-width: min-content;
}

.crew-tooltip {
  position: absolute;
  background: #222;
  color: #fff;
  font-size: 13px;
  padding: 6px 12px;
  border-radius: 6px;
  pointer-events: none;
  white-space: nowrap;
  transform: translate(12px, -50%);
  z-index: 10;
}

.crew-tooltip-name {
  font-weight: 500;
  margin-bottom: 2px;
}

.crew-tooltip-detail {
  font-size: 11px;
  opacity: 0.75;
}
</style>
