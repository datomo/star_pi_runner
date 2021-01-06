:: copy the whole folder to the pi for compiling
scp -pr C:\Users\davel\Desktop\git\star_pi_runner pi@raspberrypi:/home/pi
ssh pi@raspberrypi rm -rf /home/pi/star_pi_runner/src/*
scp -pr C:\Users\davel\Desktop\git\star_pi_runner/src pi@raspberrypi:/home/pi/star_pi_runner
scp C:\Users\davel\Desktop\git\star_pi_runner\config.json pi@raspberrypi:/home/pi/star_pi_runner/config.json

:: give the ?necesary? rights
:: ssh pi@raspberrypi chmod +x /home/pi/star_pi_runner

:: run the program
 ssh pi@raspberrypi cargo run