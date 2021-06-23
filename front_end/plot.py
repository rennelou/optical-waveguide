import h5py
import math
import numpy as np
import matplotlib.pyplot as plt

filename = "main.h5"
lines = 50

origin = 'lower'

with h5py.File(filename, "r") as f:

    for key in f.keys():
        print("Key %s" % key)
    
    [ydelta, xdelta] = f['deltas'][()]
    data = f['intensity'][()]

    xlen = data[0].size
    ylen = data.size / xlen

    x = np.arange(0.0, xlen*xdelta, xdelta)
    y = np.arange(0.0, ylen*ydelta, ydelta)

    X, Y = np.meshgrid(x, y)

    cs1 = plt.contourf(X, Y, data, origin=origin)
    cbar = plt.colorbar(cs1)  #barra lateral de intensidade
    cbar.ax.set_ylabel('intensity')
    
    core = f['core'][()]
    cs3 = plt.contour(X, Y, core, origin=origin)
    
    plt.show()