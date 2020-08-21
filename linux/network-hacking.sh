# NETWORKING

# list ethernet adaptors
ifconfig en0 | grep ether

# spoof new mac address
sudo ifconfig en0 ether 00:e2:e3:e4:e5:e6
