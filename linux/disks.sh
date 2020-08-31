# benchmark disk performance
dd bs=1M count=1024 if=/dev/zero of=test conv=fdatasync
