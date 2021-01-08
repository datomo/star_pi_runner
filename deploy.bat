:: cargo build --target=armv7-unknown-linux-gnueabihf --release

:: copy the whole folder to the pi for compiling
scp C:\Users\davel\Desktop\git\star_pi_runner\config.json pi@raspberrypi:/home/pi/config.json
scp .\target\armv7-unknown-linux-gnueabihf\release\star_pi_runner pi@raspberrypi:/home/pi/star_pi_runner

:: give the ?necesary? rights
ssh pi@raspberrypi chmod +x /home/pi/star_pi_runner/star_pi_runner

:: run the program
ssh pi@raspberrypi /home/pi/star_pi_runner/star_pi_runner