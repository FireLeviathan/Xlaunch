

sudo rm /tmp/.X4-lock
export WINEPREFIX="/home/fire/.wine"
sudo X :4 -ac -terminate &
sleep 1
DISPLAY=:4 wine $VARIABLE