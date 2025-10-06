<template>
  <div class="modal-overlay" @click="closeModal" v-if="isVisible" :class="{ hovered: isHovered }">
    <div class="modal-content" @click.stop :class="{ hovered: isHovered }">
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
  <div class="modal-overlay" @click="cancelCloseOverlay" v-if="showConfirmClose" :class="{ hovered: isHovered }">
    <div class="confirm-modal" @click.stop :class="{ hovered: isHovered }">
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
  isRunning: Boolean,
  isHovered: Boolean
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
  background: rgba(15, 15, 35, 0.25);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-content {
  background: rgba(15, 15, 35, 0.25);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  color: #e0f6ff;
  padding: 24px;
  border-radius: 24px;
  min-width: 320px;
  max-width: 90vw;
  max-height: 90vh;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 32px 64px rgba(0, 0, 0, 0.4),
              0 16px 32px rgba(0, 0, 0, 0.3),
              inset 0 1px 0 rgba(255, 255, 255, 0.05);
  font-size: 16px;
  animation: modalSlideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-overlay.hovered {
  background: rgba(15, 15, 35, 0.4);
}

.modal-content.hovered {
  background: rgba(15, 15, 35, 0.4);
}

.settings-section {
  margin-bottom: 24px;
}

.time-setting {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
  font-size: 14px;
}

label {
  width: 60px;
  margin-right: 12px;
  flex-shrink: 0;
  color: #e0f6ff;
  font-weight: 500;
}

.slider {
  flex: 1;
  margin-right: 12px;
  height: 8px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  outline: none;
  -webkit-appearance: none;
  appearance: none;
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: linear-gradient(135deg, #40e0d0, #6495ed);
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(64, 224, 208, 0.3);
  transition: all 0.2s ease;
}

.slider::-webkit-slider-thumb:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(64, 224, 208, 0.5);
}

.slider::-moz-range-thumb {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: linear-gradient(135deg, #40e0d0, #6495ed);
  cursor: pointer;
  border: none;
  box-shadow: 0 2px 8px rgba(64, 224, 208, 0.3);
  transition: all 0.2s ease;
}

.slider::-moz-range-thumb:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(64, 224, 208, 0.5);
}

.input-group {
  display: flex;
  align-items: center;
  flex: 1;
  margin-right: 12px;
}

.input-group button {
  width: 28px;
  height: 28px;
  padding: 0;
  font-size: 16px;
  line-height: 1;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  color: #e0f6ff;
  cursor: pointer;
  transition: all 0.2s ease;
}

.input-group button:hover {
  background: rgba(64, 224, 208, 0.2);
  border-color: rgba(64, 224, 208, 0.4);
  transform: scale(1.05);
}

.number-input {
  width: 50px;
  text-align: center;
  margin: 0 8px;
  height: 28px;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  color: #e0f6ff;
  outline: none;
  transition: all 0.2s ease;
}

.number-input:focus {
  border-color: #40e0d0;
  box-shadow: 0 0 0 2px rgba(64, 224, 208, 0.2);
}

.checkbox-setting {
  margin-bottom: 12px;
  font-size: 14px;
}

.checkbox-setting input[type="checkbox"] {
  margin-right: 8px;
  width: 16px;
  height: 16px;
  accent-color: #40e0d0;
}

.checkbox-setting label {
  color: #e0f6ff;
  cursor: pointer;
}

.buttons-section {
  display: flex;
  justify-content: space-between;
  gap: 12px;
}

button {
  padding: 12px 20px;
  border: none;
  border-radius: 12px;
  cursor: pointer;
  background: linear-gradient(135deg, rgba(64, 224, 208, 0.2), rgba(100, 149, 237, 0.2));
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: #e0f6ff;
  font-size: 14px;
  font-weight: 500;
  flex: 1;
  min-width: 0;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2),
              inset 0 1px 0 rgba(255, 255, 255, 0.1);
}

button:hover {
  background: linear-gradient(135deg, rgba(64, 224, 208, 0.3), rgba(100, 149, 237, 0.3));
  border-color: rgba(64, 224, 208, 0.3);
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3),
              inset 0 1px 0 rgba(255, 255, 255, 0.2);
}

button:active {
  transform: translateY(0);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2),
              inset 0 1px 0 rgba(255, 255, 255, 0.1);
}

button:focus-visible {
  outline: 2px solid #40e0d0;
  outline-offset: 2px;
}

.confirm-modal {
  background: rgba(15, 15, 35, 0.25);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  color: #e0f6ff;
  padding: 24px;
  border-radius: 24px;
  text-align: center;
  font-size: 16px;
  min-width: 280px;
  max-width: 80vw;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 32px 64px rgba(0, 0, 0, 0.4),
              0 16px 32px rgba(0, 0, 0, 0.3),
              inset 0 1px 0 rgba(255, 255, 255, 0.05);
  animation: modalSlideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.confirm-modal.hovered {
  background: rgba(15, 15, 35, 0.4);
}

.confirm-modal p {
  margin: 0 0 20px 0;
  font-size: 14px;
  color: #e0f6ff;
}

.confirm-buttons {
  margin-top: 20px;
  display: flex;
  justify-content: space-around;
  gap: 12px;
}

@keyframes modalSlideIn {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(20px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

@media (prefers-reduced-motion: reduce) {
  .modal-content,
  .confirm-modal {
    animation: none;
  }

  button,
  .input-group button,
  .slider::-webkit-slider-thumb,
  .slider::-moz-range-thumb {
    transition: none;
  }

  button:hover,
  .input-group button:hover {
    transform: none;
  }

  .slider::-webkit-slider-thumb:hover,
  .slider::-moz-range-thumb:hover {
    transform: none;
  }
}

@media (max-width: 480px), (max-height: 320px) {
  .modal-content {
    padding: 20px;
    min-width: 300px;
    font-size: 14px;
  }

  .settings-section {
    margin-bottom: 20px;
  }

  .time-setting {
    margin-bottom: 10px;
    font-size: 13px;
  }

  label {
    width: 55px;
    font-size: 13px;
  }

  .slider {
    height: 6px;
  }

  .slider::-webkit-slider-thumb,
  .slider::-moz-range-thumb {
    width: 18px;
    height: 18px;
  }

  .number-input {
    width: 45px;
    height: 26px;
    font-size: 13px;
  }

  .input-group button {
    width: 26px;
    height: 26px;
    font-size: 14px;
  }

  button {
    padding: 10px 16px;
    font-size: 13px;
  }

  .confirm-modal {
    padding: 20px;
    min-width: 260px;
    font-size: 14px;
  }

  .confirm-modal p {
    font-size: 13px;
  }
}

@media (max-width: 320px), (max-height: 240px) {
  .modal-content {
    padding: 16px;
    min-width: 280px;
    font-size: 12px;
  }

  .settings-section {
    margin-bottom: 16px;
  }

  .time-setting {
    margin-bottom: 8px;
    font-size: 12px;
  }

  label {
    width: 50px;
    font-size: 12px;
  }

  .slider {
    height: 6px;
  }

  .slider::-webkit-slider-thumb,
  .slider::-moz-range-thumb {
    width: 16px;
    height: 16px;
  }

  .number-input {
    width: 40px;
    height: 24px;
    font-size: 12px;
  }

  .input-group button {
    width: 24px;
    height: 24px;
    font-size: 12px;
  }

  button {
    padding: 8px 12px;
    font-size: 12px;
  }

  .confirm-modal {
    padding: 16px;
    min-width: 240px;
    font-size: 12px;
  }

  .confirm-modal p {
    font-size: 12px;
  }
}
</style>