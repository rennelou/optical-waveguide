import json
import subprocess

def run(simulation_path, simulation, output_name):
    with open('simulation_temp.json', 'w') as f:
        json.dump(simulation, f, sort_keys=True)    

    subprocess.run([simulation_path, 'simulation_temp.json', output_name])

    return

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

def get_beam(k, width, x = None, y = None):
    beam = {
        "k": k,
        "width": width
    }

    if x is not None:
        beam["x"] = x
    
    if y is not None:
        beam["y"] = y

    return beam
