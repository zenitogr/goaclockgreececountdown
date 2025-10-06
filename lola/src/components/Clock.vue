<template>
  <div class="clock-container">
    <svg class="circles" width="100%" height="100%" viewBox="0 0 100 100">
      <!-- Seconds circle (outermost) -->
      <circle cx="50" cy="50" :r="secondsRadius" fill="none" stroke="white" stroke-width="1" :stroke-dasharray="secondsDashArray" stroke-dashoffset="0" opacity="0.5" transform="rotate(-90 50 50)"/>
      <!-- Minutes circle (middle) -->
      <circle cx="50" cy="50" :r="minutesRadius" fill="none" stroke="white" stroke-width="2" :stroke-dasharray="minutesDashArray" stroke-dashoffset="0" opacity="0.5" transform="rotate(-90 50 50)"/>
      <!-- Hours circle (innermost) -->
      <circle cx="50" cy="50" :r="hoursRadius" fill="none" stroke="white" stroke-width="4" :stroke-dasharray="hoursDashArray" stroke-dashoffset="0" opacity="0.5" transform="rotate(-90 50 50)"/>
      <!-- Roman numerals positioned outside the seconds circle -->
      <text :x="50" :y="50 - (hoursRadius - 13)" text-anchor="middle" fill="white" opacity="0.75" font-size="12" font-family="serif" font-weight="bold">XII</text>
      <text :x="50 + (hoursRadius -11 )" :y="50 + 3" text-anchor="middle" fill="white" opacity="0.75" font-size="12" font-family="serif" font-weight="bold">III</text>
      <text :x="50" :y="50 + (hoursRadius-12 ) + 3" text-anchor="middle" fill="white" opacity="0.75" font-size="12" font-family="serif" font-weight="bold">VI</text>
      <text :x="50 - (hoursRadius -10)" :y="50 + 3" text-anchor="middle" fill="white" opacity="0.75" font-size="12" font-family="serif" font-weight="bold">IX</text>
    </svg>
    <div class="time-display">
      <div class="time-line">{{ hoursMinutes }}</div>
      <div class="time-line">{{ secondsDisplay }}</div>
    </div>
  </div>
</template>


<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const time = ref('00:00:00')

let intervalId

const updateTime = async () => {
  try {
    const response = await invoke('get_current_time')
    time.value = response.time
  } catch (error) {
    console.error('Failed to get current time:', error)
  }
}

const hours = computed(() => parseInt(time.value.split(':')[0]) || 0)
const minutes = computed(() => parseInt(time.value.split(':')[1]) || 0)
const seconds = computed(() => parseInt(time.value.split(':')[2]) || 0)
const hoursMinutes = computed(() => `${hours.value.toString().padStart(2, '0')}:${minutes.value.toString().padStart(2, '0')}`)
const secondsDisplay = computed(() => seconds.value.toString().padStart(2, '0'))

// Individual progress for each time unit: hours/12 (12-hour format), minutes/60, seconds/60
const hoursProgress = computed(() => (hours.value % 12) / 12)
const minutesProgress = computed(() => minutes.value / 60)
const secondsProgress = computed(() => seconds.value / 60)

// Radii: outer seconds (full size), middle minutes (90% of seconds), inner hours (80% of seconds)
const secondsRadius = 40
const minutesRadius = 36
const hoursRadius = 32

// Circumference for dash array (2 * pi * r)
const hoursCircumference = computed(() => 2 * Math.PI * hoursRadius)
const minutesCircumference = computed(() => 2 * Math.PI * minutesRadius)
const secondsCircumference = computed(() => 2 * Math.PI * secondsRadius)

// Individual progress dash arrays
const hoursDashArray = computed(() => `${hoursProgress.value * hoursCircumference.value} ${(1 - hoursProgress.value) * hoursCircumference.value}`)
const minutesDashArray = computed(() => `${minutesProgress.value * minutesCircumference.value} ${(1 - minutesProgress.value) * minutesCircumference.value}`)
const secondsDashArray = computed(() => `${secondsProgress.value * secondsCircumference.value} ${(1 - secondsProgress.value) * secondsCircumference.value}`)

onMounted(() => {
  updateTime() // Initial update
  intervalId = setInterval(updateTime, 1000)
})
onMounted(() => {
  updateTime() // Initial update
  intervalId = setInterval(updateTime, 1000)
})

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId)
  }
})
</script>

<style scoped>
.clock-container {
  position: relative;
  width: 100%;
  height: 100%;
}

.circles {
  position: absolute;
  top: 0;
  left: 0;
}

.time-display {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: white;
  opacity: 0.75;
  font-size: 20px;
  font-weight: bold;
  font-family: monospace;
  display: flex;
  flex-direction: column;
  align-items: center;
  line-height: 1;
}
</style>