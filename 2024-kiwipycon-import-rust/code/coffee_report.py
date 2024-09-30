import threading

a = "Wellington's coffee is better."

def coffee_report():
    print(a)

thread_1 = threading.Thread(target=coffee_report)
thread_1.start()

thread_1.join()
