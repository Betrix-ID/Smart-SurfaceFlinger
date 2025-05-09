// It is forbidden to change without official permission from the Module maker. 

use std::{process::Command, thread::sleep, time::Duration};

fn shell(message: &str, fps: i32) {
    let command = format!(
        "cmd notification post -S bigtext -t '♨️ Automatic SurfaceFllinger' 'Tag' '{} {} Hz' > /dev/null 2>&1",
        message, fps
    );
    
    if let Err(e) = Command::new("sh").arg("-c").arg(command).status() {
        eprintln!("Failed to execute shell command: {}", e);
    }
}

fn get_duration() -> i32 {
    let output = Command::new("sh")
        .arg("-c")
        .arg("dumpsys SurfaceFlinger | awk '/refresh-rate/ { gsub(/[^0-9.]+/, \"\", $3); printf(\"%.0f\\n\", $3) }'")
        .output()
        .expect("Failed to read duration");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.trim().parse().unwrap_or(-1)
}

fn run_cmd(command: &str) {
    if let Err(e) = Command::new("sh").arg("-c").arg(command).status() {
        eprintln!("Failed to execute command: {}", e);
    }
}

fn apply_props(duration: i32) {
    let commands = [
        format!("dumpsys SurfaceFlinger --timestats -clear -disable"),
        format!("setprop debug.sf.late.sf.duration  {}", duration),
        format!("setprop debug.sf.late.app.duration {}", duration),
        format!("setprop debug.sf.early.sf.duration {}", duration),
        format!("setprop debug.sf.early.app.duration {}", duration),
        format!("setprop debug.sf.earlyGl.sf.duration {}", duration),
        format!("setprop debug.sf.earlyGl.app.duration  {}", duration),
        format!("setprop debug.sf.hwc.min.duration {}", duration),
        format!("setprop debug.vsync_event_phase_offset_ns {}", duration),
        format!("setprop debug.vsync_sf_event_phase_offset_ns  {}", duration),
        format!("setprop debug.sf.phase_offset_threshold_for_next_vsync_ns {}", duration),
        format!("setprop debug.sf.latch_unsignaled false "),
        format!("setprop debug.sf.disable_backpressure 1 "),
        format!("setprop debug.sf.use_phase_offsets_as_durations 1"),
    ];
    for cmd in commands {
        run_cmd(&cmd);
    }
}

pub fn monitor_auto() {
    println!("\nDescription:\n  Automatically set duration from SurfaceFlinger output.");
    let mut last_fps = -1;

    loop {
        let duration = get_duration();
        if duration > 0 {
            let fps = 1_000_000_000 / duration; // Kalkulasi fps

            if fps != last_fps {
                sleep(Duration::from_secs(2));
                println!("Duration: {} ns ({} Hz)", duration, fps);
                apply_props(fps);
                shell("Duration set to", duration);
                last_fps = fps;
            }
        } else {
            eprintln!("Invalid duration received: {}", duration);
        }
    }
}

pub fn surface_flinger_custom(target_hz: i32) {
    println!("\nDescription:\n  Custom duration setting for target frequency.\n  Applying a resolution preset optimized for devices with {} Hz.\n  Focused on balancing performance and power consumption.\n", target_hz);
    let duration = 1_000_000_000 / target_hz;
    println!("Calculated duration: {} ns", duration);
    apply_props(duration);
    
    sleep(Duration::from_secs(2));
    shell("Duration set to", target_hz);
}

pub fn surface_flinger120() {
    println!("\nDescription:\n  Setting SurfaceFlinger to 120 Hz.\n  Optimized for high frame rates, enhancing visual smoothness.\n  Ideal for gaming and demanding applications.");
    surface_flinger_custom(120);
}

pub fn surface_flinger90() {
    println!("\nDescription:\n  Setting SurfaceFlinger to 90 Hz.\n  A balanced option for smoother animations and battery efficiency.\n  Suitable for general usage and multimedia.");
    surface_flinger_custom(90);
}

pub fn surface_flinger60() {
    println!("\nDescription:\n  Setting SurfaceFlinger to 60 Hz.\n  Standard refresh rate for most applications, ensuring stable performance.\n  Best for users prioritizing battery life.");
    surface_flinger_custom(60);
}

pub fn kill() {
    println!("- Resetting SurfaceFlinger to default settings. This command will restore all SurfaceFlinger settings to their default state. After executing this command, it is highly recommended to restart your device to ensure that all changes are correctly applied and your settings return to default.");
    shell("Resetting SurfaceFlinger", 0); 
}