

sudo rm /tmp/.X4-lock
sudo X :4 -ac -terminate &
sleep 1
DISPLAY=:4 $VARIABLE