import h5py
import json
import subprocess
import numpy as np
import matplotlib
matplotlib.use('Qt5Agg')
import matplotlib.pyplot as plt

lines = 50
origin = 'lower'

output_filename = "bidimensional_result.h5"
input_filename = "bidimensional_simulation.json"
simulation_param = {
        "x_axis": {
            "width": 40,
            "delta": 0.02 
        },
        "z_axis": {
            "width": 750,
            "delta": 0.5
        },
        "core": {
            "n0": 3.377,
            "n": 3.38,
            "width": 8,
            "x": 20
        },
        "beam": {
            "k": 5.4636,
            "x": 20,
            "width": 4
        }
    }

with open(input_filename, 'w') as f:
    json.dump(simulation_param, f, sort_keys=True)

subprocess.run(["../release/optical_waveguide", input_filename, output_filename])

with h5py.File(output_filename, "r") as f:
        
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
