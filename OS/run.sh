#!/system/bin/sh
# Checking ID shell
if [ "$(id -u)" -ne 0 ] && [ "$(id -u)" -ne 2000 ]; then
	printf "[ Eror |@Yeye_nat(Yeye)]\n"
   exit 1
fi
#Chking cpu.abi
     if [ ! -f /sdcard/int/source/target ]; then
	    architecture=$(getprop ro.product.cpu.abi)
	  if [ "$architecture" = "arm64-v8a" ]; then
		cp /sdcard/int/source/target/debug/smart_SurfaceFlinger /sdcard/int/SSD
	elif [ "$architecture" = "armeabi-v7a" ]; then
		cp /sdcard/int/source/target/release/smart_SurfaceFlinger /sdcard/int/SSD
	fi
  fi
#smart notifications
shell() {
    sor="$1"
    cmd notification post -S bigtext -t '♨️ Smart SurfaceFlinger' 'Tag' "$sor" > /dev/null 2>&1
}
	
# Style display Terminal
set +x
    echo
    echo "     ☆================================☆"
    echo
    echo "       ~ Description. Smart SurfaceFlinger...... "
    echo
    echo "       - Author                 :  @UnixeID"
    echo "       - Point                    :  1.0 "
    echo "       - Release               :  4 - Mei - 2025"
    echo "       - Name Shell         :  Smart SurfaceFlinger"
    echo
    echo "    |_______________________________________|"
    echo "    \______________________________________/"
    echo
    echo "   Priority Smart SurfaceFlinger Custem. "
    echo
    sleep 2
     rm -rf /data/local/tmp/smart_SurfaceFlinger
     cp /sdcard/OS/SSD /data/local/tmp
     chmod +x /data/local/tmp/SSD
     if [ "$1" = "-d" ]; then
          shell "Applying SurfaceFlinger for duration 120fps 1-4 seconds..."
          /data/local/tmp/SSD -d
     elif [ "$1" = "-L" ]; then
          shell "Applying SurfaceFlinger for duration 900fps 1-4 seconds..."
          /data/local/tmp/SSD -L
     elif [ "$1" = "-P" ]; then
          shell "Applying SurfaceFlinger for duration auto smart Fps 1-4 seconds..."
          /data/local/tmp/SSD -P
     elif [ "$1" = "-R" ]; then
           shell "Please wait to description reset explanation 1-2 seconds..."
           /data/local/tmp/SSD -R
     elif [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
           /data/local/tmp/SSD --help
        else
          printf "Failed to apply requested profile. Unknown option: %s\n" "$1"
         fi
set +x