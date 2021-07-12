import h5py
import numpy as np
import matplotlib.pyplot as plt

origin = 'lower'

filename = "../fdmbpm/slab3d.h5"
lines = 50
with h5py.File(filename, "r") as f:

    for key in f.keys():
        print("Key %s" % key)
    
    [zdelta, ydelta, xdelta] = f['deltas'][()]
    data = f['intensity'][()]
    core = f['core'][()]

    xdepht = data[0][0].size
    ydepht = data[0].size / xdepht
    zdepht = data.size / (ydepht * xdepht)

    y = np.arange(0., ydepht * ydelta, ydelta)
    x = np.arange(0., xdepht * xdelta, xdelta)
    X, Y = np.meshgrid(x, y)

    zstep = zdepht / 4
    fig1, axs = plt.subplots(2, 4, constrained_layout=True)
    for i in range(4):
        index = int(i * zstep)
        Z = data[index]
        ax = axs[0][i]
        cs = ax.contourf(X, Y, Z, 10, cmap=plt.cm.bone, origin=origin)
        cs1 =  ax.contour(cs, levels=cs.levels[::2], cmap='inferno', origin=origin)
        
        ax2 = axs[1][i]
        C = core[index]
        cs2 = ax2.contourf(X, Y, C, 10, cmap=plt.cm.bone, origin=origin)
        cs3 =  ax2.contour(cs2, levels=cs2.levels[::2], origin=origin)
        
        if i == 3:
            cbar = fig1.colorbar(cs, ax=ax)  #barra lateral de intensidade
            cbar.ax.set_ylabel('intensity')
            cbar.add_lines(cs1)

            cbar = fig1.colorbar(cs3, ax=ax2)  #barra lateral de intensidade
            cbar.ax.set_ylabel('refractive index')
            cbar.add_lines(cs3)
        
    plt.show()
