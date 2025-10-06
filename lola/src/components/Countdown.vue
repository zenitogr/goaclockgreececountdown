<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const remainingSeconds = ref(0)
const initialSeconds = ref(0)
const smoothRemaining = ref(0)
const lastUpdateTime = ref(Date.now())
const animationId = ref(null)
const isFinished = computed(() => remainingSeconds.value === 0)
let intervalId = null

function formatTime(seconds) {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const secs = seconds % 60
  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
}

async function fetchStatus() {
  try {
    const result = await invoke('get_countdown_status')
    remainingSeconds.value = result.remaining_seconds
    smoothRemaining.value = result.remaining_seconds
    lastUpdateTime.value = Date.now()
    if (initialSeconds.value === 0 && remainingSeconds.value > 0) {
      initialSeconds.value = remainingSeconds.value
    }
  } catch (error) {
    console.error('Error fetching countdown status:', error)
  }
}

function animateCountdown() {
  const now = Date.now()
  const delta = (now - lastUpdateTime.value) / 1000
  smoothRemaining.value -= delta
  if (smoothRemaining.value < 0) smoothRemaining.value = 0
  lastUpdateTime.value = now
  if (smoothRemaining.value > 0) {
    animationId.value = requestAnimationFrame(animateCountdown)
  }
}

const hours = computed(() => Math.floor(smoothRemaining.value / 3600))
const minutes = computed(() => Math.floor((smoothRemaining.value % 3600) / 60))
const seconds = computed(() => smoothRemaining.value % 60)
const countdownHoursMinutes = computed(() => `${hours.value.toString().padStart(2, '0')}:${minutes.value.toString().padStart(2, '0')}`)
const countdownSeconds = computed(() => Math.floor(seconds.value).toString().padStart(2, '0'))

const initialHours = computed(() => Math.floor(initialSeconds.value / 3600))
const initialMinutes = computed(() => Math.floor((initialSeconds.value % 3600) / 60))
const initialSecs = computed(() => initialSeconds.value % 60)

// Individual progress for each time unit: remaining/initial
const hoursProgress = computed(() => (initialSeconds.value - smoothRemaining.value) / initialSeconds.value)
const minutesProgress = computed(() => (3600 - (smoothRemaining.value % 3600)) / 3600)
const secondsProgress = computed(() => (60 - (smoothRemaining.value % 60)) / 60)

// Radii: outer seconds (full size), middle minutes (90% of seconds), inner hours (80% of seconds)
const secondsRadius = 40
const minutesRadius = 36
const hoursRadius = 32

// Circumferences
const hoursCircumference = computed(() => 2 * Math.PI * hoursRadius)
const minutesCircumference = computed(() => 2 * Math.PI * minutesRadius)
const secondsCircumference = computed(() => 2 * Math.PI * secondsRadius)

// Individual progress dash arrays
const hoursDashArray = computed(() => {
  const value = `${(1 - hoursProgress.value) * hoursCircumference.value} ${hoursProgress.value * hoursCircumference.value}`;
  return value;
});
const minutesDashArray = computed(() => {
  const value = `${(1 - minutesProgress.value) * minutesCircumference.value} ${minutesProgress.value * minutesCircumference.value}`;
  return value;
});
const secondsDashArray = computed(() => {
  const value = `${(1 - secondsProgress.value) * secondsCircumference.value} ${secondsProgress.value * secondsCircumference.value}`;
  return value;
});

onMounted(async () => {
  await fetchStatus()
  animateCountdown()
  intervalId = setInterval(fetchStatus, 1000)
})

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId)
  }
  if (animationId.value) {
    cancelAnimationFrame(animationId.value)
  }
})
</script>

<template>
  <div class="countdown-container">
    <svg class="circles" width="100%" height="100%" viewBox="0 0 100 100">
      <!-- Filled circles when countdown reaches zero -->
      <circle v-if="isFinished" cx="50" cy="50" r="40" fill="white" stroke="none" class="finished-seconds" />
      <circle v-if="isFinished" cx="50" cy="50" r="36" fill="black" stroke="none" class="finished-minutes" />
      <circle v-if="isFinished" cx="50" cy="50" r="32" fill="white" stroke="none" class="finished-hours" />
      <!-- Progress circles -->
      <circle v-if="!isFinished" cx="50" cy="50" :r="secondsRadius" fill="none" stroke="white" stroke-width="1" :stroke-dasharray="secondsDashArray" stroke-dashoffset="0" opacity="0.5" transform="rotate(-90 50 50)"/>
      <circle v-if="!isFinished" cx="50" cy="50" :r="minutesRadius" fill="none" stroke="white" stroke-width="2" :stroke-dasharray="minutesDashArray" stroke-dashoffset="0" opacity="0.5" transform="rotate(-90 50 50)"/>
      <circle v-if="!isFinished" cx="50" cy="50" :r="hoursRadius" fill="none" stroke="white" stroke-width="4" :stroke-dasharray="hoursDashArray" stroke-dashoffset="0" opacity="0.5" transform="rotate(-90 50 50)"/>
    </svg>
    <div class="time-display">
      <div class="time-line">{{ countdownHoursMinutes }}</div>
      <div class="time-line">{{ countdownSeconds }}</div>
    </div>
  </div>
</template>

<style scoped>
.countdown-container {
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

.finished-seconds {
  animation: pulse-seconds 4s ease-in-out infinite;
  transform-origin: 50% 50%;
}

.finished-minutes {
  animation: pulse-minutes 3.5s ease-in-out infinite;
  transform-origin: 50% 50%;
}

.finished-hours {
  animation: pulse-hours 3s ease-in-out infinite;
  transform-origin: 50% 50%;
}

@keyframes pulse-hours {
  0%, 100% { transform: scale(0.875); }
  50% { transform: scale(1.125); }
}

@keyframes pulse-minutes {
  0%, 100% { transform: scale(0.857); }
  50% { transform: scale(1.143); }
}

@keyframes pulse-seconds {
  0%, 100% { transform: scale(0.833); }
  50% { transform: scale(1.167); }
}
</style>