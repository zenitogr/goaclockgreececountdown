<template>
  <div class="clock-container" role="img" aria-label="Analog clock showing current time">
    <svg class="circles" width="100%" height="100%" viewBox="0 0 100 100">
      <defs>
        <linearGradient id="clockSecondsGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#40e0d0;stop-opacity:0.8" />
          <stop offset="100%" style="stop-color:#6495ed;stop-opacity:0.6" />
        </linearGradient>
        <linearGradient id="clockMinutesGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#00ced1;stop-opacity:0.9" />
          <stop offset="100%" style="stop-color:#4169e1;stop-opacity:0.7" />
        </linearGradient>
        <linearGradient id="clockHoursGradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#20b2aa;stop-opacity:1" />
          <stop offset="100%" style="stop-color:#000080;stop-opacity:0.8" />
        </linearGradient>
      </defs>
      <!-- Seconds circle (outermost) -->
      <circle cx="50" cy="50" :r="secondsRadius" fill="none" stroke="url(#clockSecondsGradient)" stroke-width="1" :stroke-dasharray="secondsDashArray" stroke-dashoffset="0" opacity="0.8" transform="rotate(-90 50 50)" class="clock-circle"/>
      <!-- Minutes circle (middle) -->
      <circle cx="50" cy="50" :r="minutesRadius" fill="none" stroke="url(#clockMinutesGradient)" stroke-width="2" :stroke-dasharray="minutesDashArray" stroke-dashoffset="0" opacity="0.85" transform="rotate(-90 50 50)" class="clock-circle"/>
      <!-- Hours circle (innermost) -->
      <circle cx="50" cy="50" :r="hoursRadius" fill="none" stroke="url(#clockHoursGradient)" stroke-width="4" :stroke-dasharray="hoursDashArray" stroke-dashoffset="0" opacity="0.9" transform="rotate(-90 50 50)" class="clock-circle"/>
      <!-- Roman numerals positioned outside the hours circle -->
      <text :x="50" :y="50 - (hoursRadius - 12)" text-anchor="middle" fill="#e0f6ff" opacity="0.9" font-size="10" font-family="serif" font-weight="bold" class="clock-numeral">XII</text>
      <text :x="50 + (hoursRadius -10 )" :y="50 + 3" text-anchor="middle" fill="#e0f6ff" opacity="0.9" font-size="10" font-family="serif" font-weight="bold" class="clock-numeral">III</text>
      <text :x="50" :y="50 + (hoursRadius-10 ) + 3" text-anchor="middle" fill="#e0f6ff" opacity="0.9" font-size="10" font-family="serif" font-weight="bold" class="clock-numeral">VI</text>
      <text :x="50 - (hoursRadius -8)" :y="50 + 3" text-anchor="middle" fill="#e0f6ff" opacity="0.9" font-size="10" font-family="serif" font-weight="bold" class="clock-numeral">IX</text>
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
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.circles {
  position: absolute;
  top: 0;
  left: 0;
  filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.3));
}

.clock-circle {
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  stroke-linecap: round;
}

.clock-numeral {
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.5));
  transition: opacity 0.3s ease;
}

.time-display {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: #e0f6ff;
  opacity: 0.9;
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
  animation: timeGlow 2s ease-in-out infinite alternate;
}

.time-line:nth-child(2) {
  animation-delay: 0.5s;
}

@keyframes timeGlow {
  from {
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5),
                 0 1px 2px rgba(0, 0, 0, 0.3),
                 0 0 8px rgba(64, 224, 208, 0.3);
  }
  to {
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5),
                 0 1px 2px rgba(0, 0, 0, 0.3),
                 0 0 12px rgba(64, 224, 208, 0.5),
                 0 0 20px rgba(64, 224, 208, 0.2);
  }
}

@media (prefers-reduced-motion: reduce) {
  .clock-circle,
  .clock-container,
  .time-display,
  .clock-numeral {
    transition: none;
    animation: none;
  }

  .time-line {
    animation: none;
  }
}

@media (max-width: 768px) {
  .time-display {
    font-size: 18px;
  }
}
</style>