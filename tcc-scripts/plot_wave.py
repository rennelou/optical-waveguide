import h5py
import numpy as np
import matplotlib.pyplot as plt

filename = "main.h5"
lines = 50
with h5py.File(filename, "r") as f:

    for key in f.keys():
        print("Key %s" % key)

    [zdelta, xdelta] = f['deltas'][()]
    data = f['intensity'][()]

    xlen = data[0].size
    zlen = data.size / xlen

    x = np.arange(0.0, xlen*xdelta, xdelta)

    throttle = int(zlen * zdelta / lines)
    for i, line in enumerate(data):
        if i % throttle == 0:
            y = line + (i*zdelta)
            plt.plot(x, y)

    #plt.plot(x,data[500])
    plt.title("Muito Fácil")
    plt.show()



