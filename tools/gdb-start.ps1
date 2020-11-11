$gdbserver = $args[0]

Start-Process powershell -ArgumentList "-Command $gdbserver", "127.0.0.1:2159", "./target/debug/realestatebg-ml.exe"