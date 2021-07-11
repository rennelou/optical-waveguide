import h5py
import math
import numpy as np
import matplotlib
matplotlib.use('Qt5Agg')
import matplotlib.pyplot as plt

filename = "../tools/target/release/compared2.h5"

with h5py.File(filename, "r") as f:

    for key in f.keys():
        print("Key %s" % key)
    
    diff = f['diff'][()]

    xlen = diff[0].size
    ylen = diff.size / xlen

    x = np.arange(0.0, xlen, 1.0)
    y = np.arange(0.0, ylen, 1.0)

    X, Y = np.meshgrid(x, y)

    cs1 = plt.contourf(X, Y, diff, origin='lower')
    cbar = plt.colorbar(cs1)  #barra lateral de intensidade
    cbar.ax.set_ylabel('diff')

    average_error = f['avarege_error'][()]
    average_error_len = average_error.size
    average_error_x = np.arange(0.0, average_error_len, 1.0)

    fig1, cs2 = plt.subplots()
    cs2.plot(average_error_x, average_error)
    cs2.set_ylabel('error')
    
    plt.show()
