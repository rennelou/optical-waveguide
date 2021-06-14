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

    group = f['dir']
    print("Group: %s" % group)
    
    [ydelta, xdelta] = group['deltas'][()]
    data = group['intensity'][()]

    xlen = data[0].size
    ylen = data.size / xlen

    x = np.arange(0.0, xlen*xdelta, xdelta)
    y = np.arange(0.0, ylen*ydelta, ydelta)

    X, Y = np.meshgrid(x, y)

    fig, (ax1, ax2) = plt.subplots(2, 1)
    #fig.subplots_adjust(hspace=0.5)

    cs1 =  ax1.contourf(X, Y, data, origin=origin)
    #cs1 =  ax1.pcolormesh(X, Y, data, shading='auto')
    cbar = fig.colorbar(cs1, ax=ax1)  #barra lateral de intensidade
    cbar.ax.set_ylabel('intensity')
    
    core = group['core'][()]
    cs2 = ax2.contourf(X, Y, core, 10, cmap=plt.cm.bone, origin=origin)
    cs3 =  ax2.contour(cs2, levels=cs2.levels[::2], origin=origin)
    cbar = fig.colorbar(cs3, ax=ax2)  #barra lateral de intensidade
    cbar.ax.set_ylabel('refractive index')
    cbar.add_lines(cs3)
    
    plt.show()

    
    