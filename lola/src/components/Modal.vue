<template>
  <div class="modal-overlay" @click="closeModal" v-if="isVisible">
    <div class="modal-content" @click.stop>
      <div class="settings-section">
        <div class="time-setting">
          <label>Hours:</label>
          <input v-if="!allowOver3Hours" type="range" min="0" max="3" v-model.number="draftHours" class="slider">
          <div v-else class="input-group">
            <button @click="draftHours = Math.max(0, draftHours - 1)">-</button>
            <input type="number" min="0" v-model.number="draftHours" class="number-input">
            <button @click="draftHours += 1">+</button>
          </div>
          <span>{{ draftHours }}</span>
        </div>
        <div class="time-setting">
          <label>Minutes:</label>
          <input type="range" min="0" max="60" v-model.number="draftMinutes" class="slider">
          <span>{{ draftMinutes }}</span>
        </div>
        <div class="time-setting">
          <label>Seconds:</label>
          <input type="range" min="0" max="60" v-model.number="draftSeconds" class="slider">
          <span>{{ draftSeconds }}</span>
        </div>
        <div class="checkbox-setting">
          <input type="checkbox" v-model="allowOver3Hours" id="allow-over-3">
          <label for="allow-over-3">Allow over 3 hours</label>
        </div>
      </div>
      <div class="buttons-section">
        <button @click="cancelModal">Cancel</button>
        <button @click="startResumeCountdown">{{ isRunning ? 'Resume' : 'Start' }} Countdown</button>
        <button @click="confirmCloseOverlay">Close Overlay</button>
      </div>
    </div>
  </div>

  <!-- Confirmation Modal for Close Overlay -->
  <div class="modal-overlay" @click="cancelCloseOverlay" v-if="showConfirmClose">
    <div class="confirm-modal" @click.stop>
      <p>Are you sure you want to close the overlay?</p>
      <div class="confirm-buttons">
        <button @click="closeOverlay">Yes</button>
        <button @click="cancelCloseOverlay">No</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  isVisible: Boolean,
  currentHours: Number,
  currentMinutes: Number,
  currentSeconds: Number,
  isRunning: Boolean
})

const emit = defineEmits(['close', 'updateSettings'])

const draftHours = ref(0)
const draftMinutes = ref(0)
const draftSeconds = ref(0)
const allowOver3Hours = ref(false)
const showConfirmClose = ref(false)

const closeModal = () => {
  if (!showConfirmClose.value) {
    saveDraftSettings()
    emit('close')
  }
}

const cancelModal = () => {
  emit('close')
}

const saveDraftSettings = () => {
  localStorage.setItem('draftHours', draftHours.value)
  localStorage.setItem('draftMinutes', draftMinutes.value)
  localStorage.setItem('draftSeconds', draftSeconds.value)
  localStorage.setItem('allowOver3Hours', allowOver3Hours.value)
}

const loadDraftSettings = () => {
  draftHours.value = parseInt(localStorage.getItem('draftHours')) || props.currentHours || 0
  draftMinutes.value = parseInt(localStorage.getItem('draftMinutes')) || props.currentMinutes || 0
  draftSeconds.value = parseInt(localStorage.getItem('draftSeconds')) || props.currentSeconds || 0
  allowOver3Hours.value = localStorage.getItem('allowOver3Hours') === 'true'
}

const restartCountdown = async () => {
  const totalSeconds = draftHours.value * 3600 + draftMinutes.value * 60 + draftSeconds.value
  try {
    await invoke('restart_countdown', { seconds: totalSeconds })
    emit('updateSettings', { hours: draftHours.value, minutes: draftMinutes.value, seconds: draftSeconds.value })
    closeModal()
  } catch (error) {
    console.error('Failed to restart countdown:', error)
  }
}

const startResumeCountdown = async () => {
  try {
    if (props.isRunning) {
      await invoke('resume_countdown')
    } else {
      const totalSeconds = draftHours.value * 3600 + draftMinutes.value * 60 + draftSeconds.value
      await invoke('start_countdown', { seconds: totalSeconds })
      emit('updateSettings', { hours: draftHours.value, minutes: draftMinutes.value, seconds: draftSeconds.value })
    }
    closeModal()
  } catch (error) {
    console.error('Failed to start/resume countdown:', error)
  }
}

const confirmCloseOverlay = () => {
  showConfirmClose.value = true
}

const cancelCloseOverlay = () => {
  showConfirmClose.value = false
}

const closeOverlay = async () => {
  try {
    await invoke('close_overlay')
  } catch (error) {
    console.error('Failed to close overlay:', error)
  }
  showConfirmClose.value = false
}

watch(() => props.isVisible, async (newVal) => {
  if (newVal) {
    loadDraftSettings()
    // Make window focusable temporarily
    try {
      await invoke('set_window_focusable', { focusable: true })
      // Focus the window to ensure it can receive input
      // Note: Tauri doesn't have a direct focus method, but making it focusable should help
    } catch (error) {
      console.error('Failed to set window focusable:', error)
    }
  } else {
    try {
      await invoke('set_window_focusable', { focusable: false })
    } catch (error) {
      console.error('Failed to set window unfocusable:', error)
    }
  }
})
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  overflow: hidden;
}

.modal-content {
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 10px;
  border-radius: 8px;
  min-width: 280px;
  max-width: 90vw;
  max-height: 90vh;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  font-size: 18px;
}

.settings-section {
  margin-bottom: 10px;
}

.time-setting {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
  font-size: 12px;
}

label {
  width: 50px;
  margin-right: 5px;
  flex-shrink: 0;
}

.slider {
  flex: 1;
  margin-right: 5px;
  height: 20px;
}

.input-group {
  display: flex;
  align-items: center;
  flex: 1;
  margin-right: 5px;
}

.input-group button {
  width: 20px;
  height: 20px;
  padding: 0;
  font-size: 16px;
  line-height: 1;
}

.number-input {
  width: 40px;
  text-align: center;
  margin: 0 2px;
  height: 20px;
  font-size: 12px;
}

.checkbox-setting {
  margin-bottom: 8px;
  font-size: 12px;
}

.checkbox-setting input[type="checkbox"] {
  margin-right: 5px;
  width: 14px;
  height: 14px;
}

.buttons-section {
  display: flex;
  justify-content: space-between;
  gap: 5px;
}

button {
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  background: white;
  color: black;
  font-size: 16px;
  flex: 1;
  min-width: 0;
}

button:hover {
  background: rgba(255, 255, 255, 0.8);
}

.confirm-modal {
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 10px;
  border-radius: 8px;
  text-align: center;
  font-size: 14px;
  min-width: 200px;
  max-width: 80vw;
}

.confirm-modal p {
  margin: 0 0 10px 0;
  font-size: 12px;
}

.confirm-buttons {
  margin-top: 10px;
  display: flex;
  justify-content: space-around;
  gap: 5px;
}

@media (max-width: 320px), (max-height: 240px) {
  .modal-content {
    padding: 8px;
    min-width: 260px;
    font-size: 12px;
  }

  .time-setting {
    margin-bottom: 6px;
    font-size: 11px;
  }

  label {
    width: 45px;
    font-size: 11px;
  }

  .slider {
    height: 16px;
  }

  .number-input {
    width: 35px;
    height: 18px;
    font-size: 11px;
  }

  .input-group button {
    width: 18px;
    height: 18px;
    font-size: 12px;
  }

  button {
    padding: 5px 10px;
    font-size: 11px;
  }

  .confirm-modal {
    padding: 8px;
    font-size: 12px;
  }

  .confirm-modal p {
    font-size: 11px;
  }
}
</style>