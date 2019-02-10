#!/usr/bin/python3

from websocket import create_connection

#ws = create_connection("ws://echo.websocket.org/")
ws = create_connection("ws://localhost:3030/echo")

print("Sending 'Hello, World'...")
ws.send("Hello, World")
print("Sent")

print("Receiving...")
result =  ws.recv()
print("Received '%s'" % result)

ws.close()
