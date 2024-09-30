import threading; import random; import time

analysis = "Wellington's coffee is better."

def coffee_report():
    time.sleep(random.random())
    print(analysis)

thread_1 = threading.Thread(target=coffee_report)
thread_1.start()
time.sleep(random.random())
del analysis
thread_1.join()

