use std::{process::Command, thread::sleep, time::Duration};

fn shell(message: &str, fps: i32) {
    let command = format!(
        "cmd notification post -S bigtext -t '♨️ Smart SurfaceFlinger' 'Tag' '{} {} Hz' > /dev/null 2>&1",
        message, fps
    );
    
    let _ = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status();
}

fn get_duration() -> i32 {
    let output = Command::new("sh")
        .arg("-c")
        .arg("dumpsys SurfaceFlinger | tr -d ' ' | grep -Eio 'appduration[^ns]+' | cut -f2 -d: | sort | tail -n1")
        .output()
        .expect("Failed to read duration");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.trim().parse().unwrap_or(-1)
}

fn run_cmd(command: &str) {
    let _ = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Failed to execute command");
}

fn apply_props(duration: i32) {
    let commands = [
        format!("dumpsys SurfaceFlinger --timestats -clear -disable"),
        format!("setprop debug.sf.phaseoffset_app {}", duration),
        format!("setprop debug.sf.present_offset -{}", duration),
        format!("setprop debug.sf.phaseoffset {}", duration),
        format!("setprop debug.sf.vsync_period {}", duration),
        format!("setprop debug.sf.early_phaseoffset {}", duration),
        format!("setprop debug.sf.early_phaseoffset_app {}", duration),
        format!("setprop debug.sf.early_duration {}", duration),
        format!("setprop debug.sf.early_duration_app {}", duration),
        format!("setprop debug.sf.glearly_phaseoffset {}", duration),
        format!("setprop debug.sf.glearly_phaseoffset_app {}", duration),
        format!("setprop debug.sf.glearly_duration {}", duration),
        format!("setprop debug.sf.glearly_duration_app {}", duration),
        format!("setprop debug.hwc.min_duration {}", duration),
        format!("setprop debug.sf.duration {}", duration),
        format!("setprop debug.sf.duration_app {}", duration),
    ];

    for cmd in commands {
        run_cmd(&cmd);
    }
}

pub fn monitor_auto() {
    println!("\nDescription:\n  Automatically set duration from SurfaceFlinger output.");
    let duration = get_duration();
    println!("Duration: {} ns", duration);
    apply_props(duration);
    
    // Notifikasi dengan fps
    let fps = 1_000_000_000 / duration;
    sleep(Duration::from_secs(2));
    shell("Duration set to", fps);
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