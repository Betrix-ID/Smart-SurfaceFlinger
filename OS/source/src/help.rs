pub fn usage() {
    let script_version = "1.0.1"; 
    println!(r#"♨️ Smart SurfaceFlinger {} - Automatic SurfaceFlinger Utility

Usage:
  SmartSurfaceFlinger [OPTION]

Options:
  -d           Activate SurfaceFlinger duration to 120 ns for smoother animations.
  -L           Activate SurfaceFlinger duration to 90 ns for improved performance.
  -O           Activate SurfaceFlinger duration to 60 ns for standard refresh rate.
  -P           Activate smart SurfaceFlinger to automatically adjust based on current device settings.
  -R           Show Reset SurfaceFlinger profile to default settings, requires reboot for changes to take effect.
  -h, --help   Show this help message and exit.

Description:
  Smart SurfaceFlinger is a utility designed to optimize the SurfaceFlinger settings on Android devices.
  This tool allows users to adjust the display refresh rate, improving the smoothness of animations and overall user experience.
  By activating specific durations, users can cater to their device's capabilities, ensuring smoother graphics rendering.
  The utility also includes an option to reset all modifications and return to default settings, which may be necessary
  after extensive testing or if issues arise. 

Examples:
  Activate 120 ns duration for enhanced performance:
      SmartSurfaceFlinger -d

  Activate 90 ns duration for balanced performance:
      SmartSurfaceFlinger -L

  Activate 60 ns duration for a standard refresh rate:
      SmartSurfaceFlinger -O

  Activate smart dumpsys to configure settings automatically:
      SmartSurfaceFlinger -P

  Reset all SurfaceFlinger settings to default:
      SmartSurfaceFlinger -R

  Display help information:
      SmartSurfaceFlinger -h --help

Requirements:
  - Root access may be required to modify SurfaceFlinger properties.
  - Device must support 'adb shell' and the necessary shell utilities to execute commands.
  
More info:
  - Amdroid developer   : https://developer.android.com/topic/performance/vitals/slow-session?hl=id#what_is_swappy
  - Document kode       :  https://android.googlesource.com/platform/frameworks/native/+/refs/heads/master/services/surfaceflinger/SurfaceFlinger.cpp
  - Documen kode two  : https://github.com/Adivennataly/HWUI-SurfaceFlinger
  "#,
    script_version);
}