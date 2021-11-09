import h5py
import optical_waveguide as wave
import numpy as np
import matplotlib
matplotlib.use('Qt5Agg')
import matplotlib.pyplot as plt

lines = 50
origin = 'lower'

def main():
    output_filename = "bidimensional_result.h5"
    
    x_axis = get_axis(40, 0.02)
    z_axis = get_axis(750, 0.5)
    core = get_core(3.377, 3.38, 8, 20)
    beam = get_beam(5.4636, 0.0, 4, 20)
    
    simulation = get_simulation(core, beam, z_axis, x_axis)
    wave.run(simulation, output_filename)

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

def get_simulation(core, beam, z_axis, x_axis = None, y_axis = None):
    simulation = {}

    if x_axis is not None:
        simulation["x_axis"] = x_axis
    
    if y_axis is not None:
        simulation["y_axis"] = y_axis

    simulation["z_axis"] = z_axis
    simulation["core"] = core
    simulation["beam"] = beam

    return simulation

def get_axis(width, delta):
    return { "width": width, "delta": delta }

def get_core(n0, n, width, x=None, y=None):
    core = {
        "n0": n0,
        "n": n,
        "width": width
    }

    if x is not None:
        core["x"] = x
    
    if y is not None:
        core["y"] = y
    
    return core

def get_beam(k, alpha, width, x = None, y = None):
    beam = {
        "k": k,
        "alpha": alpha,
        "width": width
    }

    if x is not None:
        beam["x"] = x
    
    if y is not None:
        beam["y"] = y

    return beam
