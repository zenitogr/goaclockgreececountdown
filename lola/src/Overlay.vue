<template>
  <div class="overlay" @mousedown="handleMouseDown" :style="{ width: overlayWidth + 'px', height: overlayHeight + 'px' }">
    <button @click="pinTo('top-left')" class="pin-btn top-left" aria-label="Pin to top-left corner">
      <ArrowUpLeft :size="16" />
    </button>
    <button @click="pinTo('top-center')" class="pin-btn top-center" aria-label="Pin to top-center">
      <ArrowUp :size="16" />
    </button>
    <button @click="pinTo('top-right')" class="pin-btn top-right" aria-label="Pin to top-right corner">
      <ArrowUpRight :size="16" />
    </button>
    <button @click="pinTo('left')" class="pin-btn left" aria-label="Pin to left side">
      <ArrowLeft :size="16" />
    </button>
    <button @click="pinTo('right')" class="pin-btn right" aria-label="Pin to right side">
      <ArrowRight :size="16" />
    </button>
    <button @click="pinTo('bottom-left')" class="pin-btn bottom-left" aria-label="Pin to bottom-left corner">
      <ArrowDownLeft :size="16" />
    </button>
    <button @click="pinTo('bottom-center')" class="pin-btn bottom-center" aria-label="Pin to bottom-center">
      <ArrowDown :size="16" />
    </button>
    <button @click="pinTo('bottom-right')" class="pin-btn bottom-right" aria-label="Pin to bottom-right corner">
      <ArrowDownRight :size="16" />
    </button>
    <button @click="settingsVisible = true" class="pin-btn settings-btn" aria-label="Open settings">
      <Settings :size="16" />
    </button>
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
import { ArrowUpLeft, ArrowUp, ArrowUpRight, ArrowLeft, ArrowRight, ArrowDownLeft, ArrowDown, ArrowDownRight, Settings } from 'lucide-vue-next'
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

onMounted(async () => {
  document.addEventListener('mouseup', handleMouseUp)
  window.addEventListener('resize', updateWindowSize)
  fetchCountdownStatus()

  // Debug: Check window transparency
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    const window = getCurrentWindow()
    const isTransparent = await window.isTransparent()
    console.log('Window is transparent:', isTransparent)
  } catch (e) {
    console.error('Error checking transparency:', e)
  }
})
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
  background: transparent;
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
  width: 44px;
  height: 44px;
  background: rgba(255, 255, 255, 0.06);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.07);
  border-radius: 50%;
  color: #e0f6ff;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3),
              0 4px 16px rgba(0, 0, 0, 0.2),
              inset 0 1px 0 rgba(255, 255, 255, 0.1);
}

.pin-btn:hover {
  background: linear-gradient(135deg, rgba(64, 224, 208, 0.2), rgba(100, 149, 237, 0.2));
  border-color: rgba(64, 224, 208, 0.4);
  transform: scale(1.1);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.4),
              0 6px 20px rgba(0, 0, 0, 0.3),
              inset 0 2px 0 rgba(255, 255, 255, 0.2);
}

.pin-btn:active {
  transform: scale(0.95);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3),
              inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

.pin-btn:focus-visible {
  outline: 2px solid #40e0d0;
  outline-offset: 2px;
}

.pin-btn.top-left {
  top: 16px;
  left: 16px;
}

.pin-btn.top-center {
  top: 16px;
  left: 50%;
  transform: translateX(-50%);
}

.pin-btn.top-right {
  top: 16px;
  right: 16px;
}

.pin-btn.left {
  top: 50%;
  left: 16px;
  transform: translateY(-50%);
}

.pin-btn.right {
  top: 50%;
  right: 16px;
  transform: translateY(-50%);
}

.pin-btn.bottom-left {
  bottom: 16px;
  left: 16px;
}

.pin-btn.bottom-center {
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%);
}

.pin-btn.bottom-right {
  bottom: 16px;
  right: 16px;
}

.pin-btn.settings-btn {
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: linear-gradient(135deg, rgba(64, 224, 208, 0.06), rgba(100, 149, 237, 0.06));
  border-color: rgba(64, 224, 208, 0.07);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.4),
              0 6px 20px rgba(0, 0, 0, 0.3),
              inset 0 2px 0 rgba(255, 255, 255, 0.15);
}

.pin-btn.settings-btn:hover {
  background: linear-gradient(135deg, rgba(64, 224, 208, 0.2), rgba(100, 149, 237, 0.2));
  border-color: rgba(64, 224, 208, 0.4);
}

.clock-area {
  position: absolute;
  top: 2vh;
  left: 0vw;
  width: 50%;
  height: calc(100% - 2vh);
  background: rgba(15, 15, 35, 0.08);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 24px;
  margin: 16px;
  width: calc(50% - 32px);
  height: calc(100% - 2vh - 32px);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 16px 64px rgba(0, 0, 0, 0.3),
              inset 0 1px 0 rgba(255, 255, 255, 0.05);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.countdown-area {
  position: absolute;
  top: 2vh;
  right: 0;
  width: 50%;
  height: calc(100% - 2vh);
  background: rgba(15, 15, 35, 0.08);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 24px;
  margin: 16px;
  width: calc(50% - 32px);
  height: calc(100% - 2vh - 32px);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 16px 64px rgba(0, 0, 0, 0.3),
              inset 0 1px 0 rgba(255, 255, 255, 0.05);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

@media (prefers-reduced-motion: reduce) {
  .pin-btn,
  .clock-area,
  .countdown-area {
    transition: none;
  }

  .pin-btn:hover {
    transform: none;
  }

  .pin-btn:active {
    transform: none;
  }
}
</style>