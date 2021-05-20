import h5py
import numpy as np
import matplotlib.pyplot as plt

filename = "../main.h5"
lines = 50
with h5py.File(filename, "r") as f:

    for key in f.keys():
        print("Key %s" % key)

    group = f['dir']
    print("Group: %s" % group)
    
    [zdelta, xdelta] = group['deltas'][()]
    data = group['intensity'][()]

    xlen = data[0].size
    zlen = data.size / xlen

    x = np.arange(0.0, xlen*xdelta, xdelta)

    fig, (ax1, ax2) = plt.subplots(2, 1, sharex=True)
    fig.subplots_adjust(hspace=0.5)

    throttle = int(zlen * zdelta / lines)
    for i, line in enumerate(data):
        if i % throttle == 0:
            y = line + (i*zdelta)
            ax1.plot(x, y)
    
    plt.show()

    
    