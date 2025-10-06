// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Europe;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::PhysicalPosition;
use tauri::PhysicalSize;

#[derive(Serialize, Deserialize)]
pub struct TimeResponse {
    time: String,
}

#[derive(Serialize, Deserialize)]
pub struct CountdownStatus {
    remaining_seconds: u64,
    status: String,
}
#[derive(Serialize, Deserialize)]
pub struct WindowSize {
    width: f64,
    height: f64,
}

#[derive(Clone)]
struct CountdownState {
    start_time: Option<DateTime<Utc>>,
    duration: u64,
    paused: bool,
    pause_start: Option<DateTime<Utc>>,
    paused_duration: Duration,
    tts_triggered: bool,
}

impl Default for CountdownState {
    fn default() -> Self {
        Self {
            start_time: None,
            duration: 0,
            paused: false,
            pause_start: None,
            paused_duration: Duration::zero(),
            tts_triggered: false,
        }
    }
}

fn speak_text(text: &str) -> Result<(), String> {
    use std::process::Command;

    // Use PowerShell to speak the text
    let output = Command::new("powershell")
        .args(&["-Command", &format!("Add-Type -AssemblyName System.Speech; (New-Object System.Speech.Synthesis.SpeechSynthesizer).Speak('{}');", text)])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell TTS: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "PowerShell TTS failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[tauri::command]
fn get_current_time() -> TimeResponse {
    let utc_now = Utc::now();
    let athens_time = utc_now.with_timezone(&Europe::Athens);
    TimeResponse {
        time: athens_time.format("%H:%M:%S").to_string(),
    }
}

#[tauri::command]
fn start_countdown(
    seconds: u64,
    state: tauri::State<Arc<Mutex<CountdownState>>>,
) -> Result<(), String> {
    if seconds > 10800 {
        return Err("Duration cannot exceed 3 hours (10800 seconds)".to_string());
    }
    let mut s = state.lock().unwrap();
    s.start_time = Some(Utc::now());
    s.duration = seconds;
    s.paused = false;
    s.pause_start = None;
    s.paused_duration = Duration::zero();
    s.tts_triggered = false;
    Ok(())
}

#[tauri::command]
fn pause_countdown(state: tauri::State<Arc<Mutex<CountdownState>>>) -> Result<(), String> {
    let mut s = state.lock().unwrap();
    if s.start_time.is_none() {
        return Err("Countdown not started".to_string());
    }
    if s.paused {
        // Resume
        s.paused = false;
        if let Some(pause_start) = s.pause_start {
            s.paused_duration = s.paused_duration + (Utc::now() - pause_start);
        }
        s.pause_start = None;
    } else {
        // Pause
        s.paused = true;
        s.pause_start = Some(Utc::now());
    }
    Ok(())
}

#[tauri::command]
fn reset_countdown(state: tauri::State<Arc<Mutex<CountdownState>>>) -> Result<(), String> {
    let mut s = state.lock().unwrap();
    *s = CountdownState::default();
    Ok(())
}

#[tauri::command]
fn get_countdown_status(state: tauri::State<Arc<Mutex<CountdownState>>>) -> CountdownStatus {
    let mut s = state.lock().unwrap();
    let status = if s.start_time.is_none() {
        "stopped".to_string()
    } else if s.paused {
        "paused".to_string()
    } else {
        "running".to_string()
    };
    let remaining = if let Some(start) = s.start_time {
        let now = Utc::now();
        let elapsed = if s.paused {
            if let Some(pause_start) = s.pause_start {
                (pause_start - start - s.paused_duration).num_seconds() as u64
            } else {
                0
            }
        } else {
            (now - start - s.paused_duration).num_seconds() as u64
        };
        if elapsed >= s.duration {
            0
        } else {
            s.duration - elapsed
        }
    } else {
        s.duration
    };

    // Trigger TTS when countdown reaches zero (only once)
    if remaining == 0 && s.start_time.is_some() && !s.tts_triggered {
        if let Err(e) = speak_text("countdown ended, Phantom") {
            eprintln!("Failed to speak TTS: {}", e);
        }
        s.tts_triggered = true;
    }
    CountdownStatus {
        remaining_seconds: remaining,
        status,
    }
}

#[tauri::command]
async fn set_window_position(window: tauri::Window, x: f64, y: f64) -> Result<(), String> {
    window
        .set_position(PhysicalPosition::new(x, y))
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_window_size(window: tauri::Window, width: f64, height: f64) -> Result<(), String> {
    window
        .set_size(PhysicalSize::new(width, height))
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_window_size(window: tauri::Window) -> Result<WindowSize, String> {
    let size = window.outer_size().map_err(|e| e.to_string())?;
    Ok(WindowSize {
        width: size.width as f64,
        height: size.height as f64,
    })
}

#[tauri::command]
async fn get_screen_size(window: tauri::Window) -> Result<(f64, f64), String> {
    // Try current monitor
    if let Ok(Some(monitor)) = window.current_monitor() {
        let size = monitor.size();
        return Ok((size.width as f64, size.height as f64));
    }

    // Fallback to primary monitor
    if let Ok(Some(monitor)) = window.primary_monitor() {
        let size = monitor.size();
        return Ok((size.width as f64, size.height as f64));
    }

    // Default fallback
    Ok((1920.0, 1080.0))
}

#[tauri::command]
async fn get_window_position(window: tauri::Window) -> Result<(f64, f64), String> {
    let position = window.outer_position().map_err(|e| e.to_string())?;
    Ok((position.x as f64, position.y as f64))
}

#[tauri::command]
async fn set_window_focusable(window: tauri::Window, focusable: bool) -> Result<(), String> {
    window.set_focusable(focusable).map_err(|e| e.to_string())
}

#[tauri::command]
fn resume_countdown(state: tauri::State<Arc<Mutex<CountdownState>>>) -> Result<(), String> {
    let mut s = state.lock().unwrap();
    if !s.paused {
        return Err("Countdown is not paused".to_string());
    }
    s.paused = false;
    if let Some(pause_start) = s.pause_start {
        s.paused_duration = s.paused_duration + (Utc::now() - pause_start);
    }
    s.pause_start = None;
    Ok(())
}

#[tauri::command]
fn restart_countdown(
    seconds: u64,
    state: tauri::State<Arc<Mutex<CountdownState>>>,
) -> Result<(), String> {
    let mut s = state.lock().unwrap();
    s.start_time = Some(Utc::now());
    s.duration = seconds;
    s.paused = false;
    s.pause_start = None;
    s.paused_duration = Duration::zero();
    s.tts_triggered = false;
    Ok(())
}

#[tauri::command]
async fn start_dragging(window: tauri::Window) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}

#[tauri::command]
async fn close_overlay(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let countdown_state = Arc::new(Mutex::new(CountdownState::default()));
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(countdown_state)
        .invoke_handler(tauri::generate_handler![
            get_current_time,
            start_countdown,
            pause_countdown,
            resume_countdown,
            restart_countdown,
            reset_countdown,
            get_countdown_status,
            set_window_position,
            set_window_size,
            get_window_size,
            get_screen_size,
            get_window_position,
            set_window_focusable,
            start_dragging,
            close_overlay
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
