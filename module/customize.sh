SKIPUNZIP=1

if [ "$BOOTMODE" != true ]; then
    abort "This module must be installed via Magisk Manager"
fi

ui_print "- Extracting module files"
unzip -o "$ZIPFILE" -d "$MODPATH" >&2

ui_print "- Removing placeholder files"
rm -f "$MODPATH/zygisk/placeholder"

ui_print "- Zygisk Spoofer installed"
ui_print "- Edit /data/adb/modules/zygisk_spoof/config.json to configure"
