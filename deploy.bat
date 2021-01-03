// cargo build --target=armv7-unknown-linux-gnueabihf --release

scp .\target\armv7-unknown-linux-gnueabihf\release\star_pi_runner pi@raspberrypi:/home/pi
scp .\config.json pi@raspberrypi:/home/pi
ssh pi@raspberrypi chmod +x /home/pi/star_pi_runner
ssh pi@raspberrypi /home/pi/star_pi_runner