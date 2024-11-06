#!/bin/bash

cargo build
wasm-pack build --target web

npx http-server -c-1 & 
SERVER_PID=$!

sleep 2

case "$(uname -s)" in
   Darwin)  # macOS
     open "http://127.0.0.1:8080/test2.html"
     ;;
   Linux)   # Linux
     xdg-open "http://127.0.0.1:8080/test2.html"
     ;;
   CYGWIN*|MINGW*|MSYS*)  # Windows
     start "http://127.0.0.1:8080/test2.html"
     ;;
   *)
     echo "Unsupported operating system"
     ;;
esac

wait $SERVER_PID
