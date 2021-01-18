:: cargo build --target=armv7-unknown-linux-gnueabihf --release
cross build --target armv7-unknown-linux-gnueabihf --release

:: kill all running instance of the star_pi_runner
ssh pi@192.168.1.14 pkill star_pi_runner

:: copy the whole folder to the pi for compiling
scp C:\Users\davel\Desktop\git\star_pi_runner\config.json pi@192.168.1.14:/home/pi/config.json
scp C:\Users\davel\Desktop\git\star_pi_runner\layout.json pi@192.168.1.14:/home/pi/layout.json
scp C:\Users\davel\Desktop\git\star_pi_runner\layout.json pi@192.168.1.14:/home/pi/star_pi_runner/layout.json
scp .\target\armv7-unknown-linux-gnueabihf\release\star_pi_runner pi@192.168.1.14:/home/pi/star_pi_runner
:: scp -p .\src\gui\dist\* pi@raspberrypi:/home/pi/star_pi_runner

:: give the ?necesary? rights
ssh pi@192.168.1.14 chmod +x /home/pi/star_pi_runner/star_pi_runner

:: ssh pi@192.168.1.14 export DISPLAY=:0

:: run the program
:: ssh pi@raspberrypi /home/pi/star_pi_runner/star_pi_runner