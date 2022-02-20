#!/usr/bin/bash

openssl genpkey -algorithm RSA -out cam_bot.key -pkeyopt rsa_keygen_bits:2048

openssl req -x509 -new -nodes -key cam_bot.key -sha256 -days 1825 -out cam_bot.crt
