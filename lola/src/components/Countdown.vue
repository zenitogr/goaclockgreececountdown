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
  <div class="countdown-container" role="timer" :aria-label="isFinished ? 'Countdown finished' : `Countdown timer showing ${countdownHoursMinutes}:${countdownSeconds}`">
    <svg class="circles" width="100%" height="100%" viewBox="0 0 100 100">
      <defs>
        <linearGradient id="countdownSecondsGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#ff6b6b;stop-opacity:0.8" />
          <stop offset="100%" style="stop-color:#ee5a24;stop-opacity:0.6" />
        </linearGradient>
        <linearGradient id="countdownMinutesGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#ffa500;stop-opacity:0.9" />
          <stop offset="100%" style="stop-color:#ff4500;stop-opacity:0.7" />
        </linearGradient>
        <linearGradient id="countdownHoursGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#ffd700;stop-opacity:1" />
          <stop offset="100%" style="stop-color:#ff8c00;stop-opacity:0.8" />
        </linearGradient>
        <radialGradient id="finishedGlow" cx="50%" cy="50%" r="50%">
          <stop offset="0%" style="stop-color:#40e0d0;stop-opacity:0.8" />
          <stop offset="100%" style="stop-color:#6495ed;stop-opacity:0.2" />
        </radialGradient>
      </defs>
      <!-- Filled circles when countdown reaches zero -->
      <circle v-if="isFinished" cx="50" cy="50" r="40" fill="url(#finishedGlow)" stroke="none" class="finished-seconds" />
      <circle v-if="isFinished" cx="50" cy="50" r="36" fill="rgba(15, 15, 35, 0.9)" stroke="none" class="finished-minutes" />
      <circle v-if="isFinished" cx="50" cy="50" r="32" fill="url(#finishedGlow)" stroke="none" class="finished-hours" />
      <!-- Progress circles -->
      <circle v-if="!isFinished" cx="50" cy="50" :r="secondsRadius" fill="none" stroke="url(#countdownSecondsGradient)" stroke-width="1" :stroke-dasharray="secondsDashArray" stroke-dashoffset="0" transform="rotate(-90 50 50)" class="countdown-circle"/>
      <circle v-if="!isFinished" cx="50" cy="50" :r="minutesRadius" fill="none" stroke="url(#countdownMinutesGradient)" stroke-width="2" :stroke-dasharray="minutesDashArray" stroke-dashoffset="0" transform="rotate(-90 50 50)" class="countdown-circle"/>
      <circle v-if="!isFinished" cx="50" cy="50" :r="hoursRadius" fill="none" stroke="url(#countdownHoursGradient)" stroke-width="4" :stroke-dasharray="hoursDashArray" stroke-dashoffset="0" transform="rotate(-90 50 50)" class="countdown-circle"/>
    </svg>
    <div class="time-display">
      <div class="time-line" :class="{ finished: isFinished }">{{ countdownHoursMinutes }}</div>
      <div class="time-line" :class="{ finished: isFinished }">{{ countdownSeconds }}</div>
    </div>
  </div>
</template>

<style scoped>
.countdown-container {
  position: relative;
  width: 100%;
  height: 100%;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.circles {
  position: absolute;
  top: 0;
  left: 0;
  filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.3));
}

.countdown-circle {
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  stroke-linecap: round;
}

.time-display {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: #ffe4e1;
  opacity: 1;
  font-size: 20px;
  font-weight: bold;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  display: flex;
  flex-direction: column;
  align-items: center;
  line-height: 1.1;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5),
               0 1px 2px rgba(0, 0, 0, 0.3);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.time-line {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.time-line.finished {
  color: #40e0d0;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5),
               0 1px 2px rgba(0, 0, 0, 0.3),
               0 0 12px rgba(64, 224, 208, 0.5);
  animation: finishedGlow 1.5s ease-in-out infinite alternate;
}

.finished-seconds {
  animation: pulse-seconds 4s ease-in-out infinite;
  transform-origin: 50% 50%;
  filter: drop-shadow(0 0 20px rgba(64, 224, 208, 0.6));
}

.finished-minutes {
  animation: pulse-minutes 3.5s ease-in-out infinite;
  transform-origin: 50% 50%;
}

.finished-hours {
  animation: pulse-hours 3s ease-in-out infinite;
  transform-origin: 50% 50%;
  filter: drop-shadow(0 0 20px rgba(64, 224, 208, 0.6));
}

@keyframes finishedGlow {
  from {
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5),
                 0 1px 2px rgba(0, 0, 0, 0.3),
                 0 0 12px rgba(64, 224, 208, 0.5);
  }
  to {
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5),
                 0 1px 2px rgba(0, 0, 0, 0.3),
                 0 0 20px rgba(64, 224, 208, 0.8),
                 0 0 30px rgba(64, 224, 208, 0.3);
  }
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

@media (prefers-reduced-motion: reduce) {
  .countdown-circle,
  .countdown-container,
  .time-display,
  .time-line,
  .finished-seconds,
  .finished-minutes,
  .finished-hours {
    transition: none;
    animation: none;
  }
}

@media (max-width: 768px) {
  .time-display {
    font-size: 18px;
  }
}
</style>