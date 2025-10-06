<template>
  <div class="overlay" @mousedown="handleMouseDown" :style="{ width: overlayWidth + 'px', height: overlayHeight + 'px' }">
    <button @click="pinTo('top-left')" class="pin-btn top-left">↖</button>
    <button @click="pinTo('top-center')" class="pin-btn top-center">↑</button>
    <button @click="pinTo('top-right')" class="pin-btn top-right">↗</button>
    <button @click="pinTo('left')" class="pin-btn left">←</button>
    <button @click="pinTo('right')" class="pin-btn right">→</button>
    <button @click="pinTo('bottom-left')" class="pin-btn bottom-left">↙</button>
    <button @click="pinTo('bottom-center')" class="pin-btn bottom-center">↓</button>
    <button @click="pinTo('bottom-right')" class="pin-btn bottom-right">↘</button>
    <button @click="settingsVisible = true" class="pin-btn settings-btn">⚙</button>
    <div class="clock-area">
      <Clock />
    </div>
    <div class="countdown-area">
      <Countdown />
    </div>
    <Modal
      :isVisible="settingsVisible"
      :currentHours="currentCountdown.hours"
      :currentMinutes="currentCountdown.minutes"
      :currentSeconds="currentCountdown.seconds"
      :isRunning="currentCountdown.isRunning"
      @close="settingsVisible = false"
      @updateSettings="updateCountdownSettings"
    />
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted, onUnmounted, computed } from 'vue'
import Clock from './components/Clock.vue'
import Countdown from './components/Countdown.vue'
import Modal from './components/Modal.vue'

const isDragging = ref(false)
const dragStartX = ref(0)
const dragStartY = ref(0)
const windowStartX = ref(0)
const windowStartY = ref(0)

const settingsVisible = ref(false)
const currentCountdown = ref({ hours: 0, minutes: 0, seconds: 0, isRunning: false })

const windowWidth = ref(window.innerWidth)
const windowHeight = ref(window.innerHeight)

const overlayWidth = computed(() => Math.max(windowWidth.value, 300))
const overlayHeight = computed(() => Math.max(windowHeight.value, 200))

const updateWindowSize = () => {
  windowWidth.value = window.innerWidth
  windowHeight.value = window.innerHeight
}

const fetchCountdownStatus = async () => {
  try {
    const result = await invoke('get_countdown_status')
    currentCountdown.value = {
      hours: Math.floor(result.remaining_seconds / 3600),
      minutes: Math.floor((result.remaining_seconds % 3600) / 60),
      seconds: result.remaining_seconds % 60,
      isRunning: result.is_running
    }
  } catch (error) {
    console.error('Error fetching countdown status:', error)
  }
}

const updateCountdownSettings = (settings) => {
  // Update local state if needed
  fetchCountdownStatus()
}

const handleMouseDown = async (e) => {
  if (!e.target.closest('.pin-btn') && !e.target.closest('.modal-overlay') && e.button === 0) { // Only left mouse button, not on pin buttons or modals
    console.log('handleMouseDown: Starting drag')

    try {
      await invoke('start_dragging')
      isDragging.value = true
    } catch (error) {
      console.error('Failed to start dragging:', error)
    }

    e.preventDefault()
  }
}

const handleMouseUp = () => {
  isDragging.value = false
}

onMounted(() => {
  document.addEventListener('mouseup', handleMouseUp)
  window.addEventListener('resize', updateWindowSize)
  fetchCountdownStatus()
})

const pinTo = async (position) => {
  console.log('pinTo called with position:', position)
  try {
    console.log('Calling get_screen_size...')
    const [screenWidth, screenHeight] = await invoke('get_screen_size')
    console.log('Screen size:', screenWidth, screenHeight)
    console.log('Calling get_window_size...')
    const { width: windowWidth, height: windowHeight } = await invoke('get_window_size')
    console.log('Window size:', windowWidth, windowHeight)
    let x = 0
    let y = 0

    console.log('Calculating position for:', position)
    switch (position) {
      case 'top-left':
        x = 0
        y = 0
        break
      case 'top-center':
        x = (screenWidth - windowWidth) / 2
        y = 0
        break
      case 'top-right':
        x = screenWidth - windowWidth
        y = 0
        break
      case 'left':
        x = 0
        y = (screenHeight - windowHeight) / 2
        break
      case 'right':
        x = screenWidth - windowWidth
        y = (screenHeight - windowHeight) / 2
        break
      case 'bottom-left':
        x = 0
        y = screenHeight - windowHeight
        break
      case 'bottom-center':
        x = (screenWidth - windowWidth) / 2
        y = screenHeight - windowHeight
        break
      case 'bottom-right':
        x = screenWidth - windowWidth
        y = screenHeight - windowHeight
        break
    }
    console.log('Position calculated:', x, y)

    // Clamp positions to keep window within screen bounds
    x = Math.max(0, Math.min(x, screenWidth - windowWidth))
    y = Math.max(0, Math.min(y, screenHeight - windowHeight))
    console.log('Calculated position:', x, y)

    console.log('Calling set_window_position with:', { x, y })
    await invoke('set_window_position', { x, y })
    console.log('set_window_position completed successfully')
  } catch (error) {
    console.error('Failed to pin window:', error)
  }
}

onUnmounted(() => {
  document.removeEventListener('mouseup', handleMouseUp)
  window.removeEventListener('resize', updateWindowSize)
})
</script>

<style>
html, body {
  overflow: hidden;
  margin: 0;
  padding: 0;
}
</style>

<style scoped>
.overlay {
  position: relative;
  background: transparent;
  cursor: move;
  overflow: hidden;
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
}

.pin-btn {
  position: absolute;
  background: rgba(255, 255, 255, 0.2);
  border: none;
  color: white;
  cursor: pointer;
  font-size: 16px;
  padding: 3px 6px;
  border-radius: 2px;
  z-index: 10;
}

.pin-btn:hover {
  background: rgba(255, 255, 255, 0.4);
}

.pin-btn.top-left {
  top: 0;
  left: 0;
}

.pin-btn.top-center {
  top: 0;
  left: 50%;
  transform: translateX(-50%);
}

.pin-btn.top-right {
  top: 0;
  right: 0;
}

.pin-btn.left {
  top: 50%;
  left: 0;
  transform: translateY(-50%);
}

.pin-btn.right {
  top: 50%;
  right: 0;
  transform: translateY(-50%);
}

.pin-btn.bottom-left {
  bottom: 0;
  left: 0;
}

.pin-btn.bottom-center {
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
}

.pin-btn.bottom-right {
  bottom: 0;
  right: 0;
}

.pin-btn.settings-btn {
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.clock-area {
  position: absolute;
  top: 2vh;
  left: 0;
  width: 50%;
  height: calc(100% - 2vh);
  background: rgba(0, 0, 0, 0.5); /* semi-transparent background */
  display: flex;
  align-items: center;
  justify-content: center;
}

.countdown-area {
  position: absolute;
  top: 2vh;
  right: 0;
  width: 50%;
  height: calc(100% - 2vh);
  background: rgba(0, 0, 0, 0.5); /* semi-transparent background */
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>