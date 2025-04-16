'''
Example of how to run the Spring.fmu using fmpy
'''

from fmpy import read_model_description, extract
from fmpy.fmi2 import FMU2Slave

import shutil
import matplotlib.pyplot as plt

if __name__ == '__main__':
    fmu_filename = '../Spring.fmu'
    
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

    fmu.setReal(
        [mass_index, stiffness_index, damping_index], 
        [1.0, 1.0, 0.1]
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
        pos, vel, acc = fmu.getReal([position_index, velocity_index, acceleration_index])

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


