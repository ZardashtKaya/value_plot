import requests
import time
import sys
import subprocess
process = subprocess.Popen(["./target/debug/value_plot"], stdin=subprocess.PIPE)

PP_ADDRESS = "http://192.168.1.95"

def test1():
    PP_CHANNELS = ["acc"]
    # PP_CHANNELS = ["pressure"]
    PP_CHANNELS_COUNT = len(PP_CHANNELS)
    M_CONTROLS = [70]
    M_CHANNEL = 0

    starturl = PP_ADDRESS + "/control?cmd=start"
    requests.get(starturl)
    index = 0

    while True:
        url = PP_ADDRESS + "/get?" + ("&".join(PP_CHANNELS))
        data = requests.get(url=url).json()
        
        for i, control in enumerate(M_CONTROLS):
            value = data["buffer"][PP_CHANNELS[i]]["buffer"][0]
            index+=1
            buff = str(index) + " " + str(value)
            print (buff)
            time.sleep(.1)
            process.stdin.write(str.encode(buff+"\n"))
def test2():
    starturl = PP_ADDRESS + "/control?cmd=start"
    requests.get(starturl)
    while True:
        pass

test1()