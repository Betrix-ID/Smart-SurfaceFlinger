#!/system/bin/sh
# Checking ID shell
if [ "$(id -u)" -ne 0 ] && [ "$(id -u)" -ne 2000 ]; then
	printf "[ Eror |@Yeye_nat(Yeye)]\n"
   exit 1
fi
#Chking cpu.abi
     if [ ! -f /sdcard/venlib/source/target ]; then
	    architecture=$(getprop ro.product.cpu.abi)
	  if [ "$architecture" = "arm64-v8a" ]; then
		cp /sdcard/venlib/source/target/release/arm64 /sdcard/venlib/libdl
	elif [ "$architecture" = "armeabi-v7a" ]; then
		cp /sdcard/venlib/source/target/release/arm /sdcard/venlib/libdl
	fi
  fi
#smart notifications
shell() {
    sor="$1"
    cmd notification post -S bigtext -t '♨️ Automatic SurfaceFllinger' 'Tag' "$sor" > /dev/null 2>&1
}
	
# Style display Terminal
set +x
    echo
    echo "     ☆================================☆"
    echo
    echo "       ~ Description. Automatic SurfaceFllinger...... "
    echo
    echo "       - Author                 :  @UnixeID"
    echo "       - Point                    :  2.0 "
    echo "       - Release               :  09 - Mei - 2025"
    echo "       - Name Shell         :  Automatic SurfaceFllinger"
    echo
    echo "    |_______________________________________|"
    echo "    \______________________________________/"
    echo
    echo "   Priority Smart SurfaceFlinger Custem. "
    echo
    sleep 2
     rm -rf /data/local/tmp/libdl
     cp /sdcard/venlib/libdl /data/local/tmp
     chmod +x /data/local/tmp/libdl
     if [ "$1" = "-d" ]; then
          shell "Applying SurfaceFlinger for duration 120fps 1-4 seconds..."
          /data/local/tmp/libdl -d
     elif [ "$1" = "-L" ]; then
          shell "Applying SurfaceFlinger for duration 900fps 1-4 seconds..."
          /data/local/tmp/libdl -L
     elif [ "$1" = "-O" ]; then
          shell "Applying SurfaceFlinger for duration 600fps 1-4 seconds..."
          /data/local/tmp/libdl -O
     elif [ "$1" = "-P" ]; then
          shell "Applying SurfaceFlinger for duration auto smart Fps 1-4 seconds..."
          nohup  /data/local/tmp/libdl -P > /dev/null 2>&1
     elif [ "$1" = "-R" ]; then
           shell "Please wait to description reset explanation 1-2 seconds..."
           /data/local/tmp/libdl -R
     elif [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
           /data/local/tmp/libdl --help
        else
          printf "Failed to apply requested profile. Unknown option: %s\n" "$1"
         fi
set +x

