'''
Example of how to run the Spring.fmu using fmpy
'''

from fmpy import read_model_description, extract
from fmpy.fmi2 import FMU2Slave

import shutil
from pathlib import Path
import matplotlib.pyplot as plt

def find_fmu() -> Path:
    '''
    Returns the path to the Spring.fmu, built with 'cargo fmu-from-struct'. The release build is
    used if it exists, otherwise the debug build.
    '''
    fmu_folder = Path(__file__).resolve().parent.parent / 'target' / 'fmu'

    for profile in ['release', 'debug']:
        fmu_path = fmu_folder / profile / 'Spring.fmu'

        if fmu_path.exists():
            return fmu_path

    raise FileNotFoundError(
        f"Spring.fmu not found in {fmu_folder / 'release'} or {fmu_folder / 'debug'}. "
        "Build it first by running 'cargo fmu-from-struct --release' (or 'cargo fmu-from-struct' "
        "for a debug build) in the spring folder."
    )

if __name__ == '__main__':
    fmu_filename = str(find_fmu())

    print(f'Running {fmu_filename}')

    model_description = read_model_description(fmu_filename)

    vrs = {}
    for variable in model_description.modelVariables:
        vrs[variable.name] = variable.valueReference

    stiffness_index = vrs['stiffness']
    mass_index = vrs['mass']
    damping_index = vrs['damping']

    position_index = vrs['position']
    velocity_index = vrs['velocity']
    acceleration_index = vrs['acceleration']
    
    unzipdir = extract(fmu_filename)

    fmu = FMU2Slave(
        guid = model_description.guid,
        unzipDirectory = unzipdir,
        modelIdentifier = model_description.coSimulation.modelIdentifier,
        instanceName = 'spring1'
    )

    start_time = 0.0
    stop_time = 20.0
    step_size = 0.1

     # initialize
    fmu.instantiate()
    fmu.setupExperiment(startTime=start_time)
    fmu.enterInitializationMode()
    
    time = start_time

    time_array = []
    pos_array = []
    vel_array = []
    acc_array = []
    
    # Only sett the stiffness and damping, as mass has a default value of 1.0 in the FMU.
    # **Note**: the mass can, off-course, be overridden here if wanted
    fmu.setReal(
        [stiffness_index, damping_index], 
        [1.0, 0.1]
    )

    fmu.setReal([position_index], [1.0])

    fmu.exitInitializationMode()

    # simulation loop
    while time < stop_time:
        # perform one step
        fmu.doStep(
            currentCommunicationPoint = time, 
            communicationStepSize = step_size
        )

        # advance the time
        time += step_size

        # Extract output values
        pos, vel, acc = fmu.getReal(
            [position_index, velocity_index, acceleration_index]
        )

        time_array.append(time)
        pos_array.append(pos)
        vel_array.append(vel)
        acc_array.append(acc)

    fmu.terminate()
    fmu.freeInstance()

    # clean up
    shutil.rmtree(unzipdir, ignore_errors=True)

    # Plot results
    plt.plot(time_array, pos_array, label='Position')

    plt.show()


