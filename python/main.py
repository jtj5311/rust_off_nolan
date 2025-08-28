import numpy as np
from numba import njit
import time 


def setup(N:int)->np.ndarray:
    array = np.random.rand(N)
    return array

@njit(cache = True)
def hot_loop(array: np.ndarray, steps: int)->float:
    output = 0
    for i in range(steps):
        output += array[i]

    return output



def main(N:int, steps:int):
    array = setup(N)
    X = hot_loop(array, steps)


    array = setup(N)
    X = hot_loop(array, steps)

    start = time.time()
    X = hot_loop(array, steps)
    print((time.time()-start)*1000, 'ms')
    return X



if __name__ == "__main__":
    N = 10**6
    steps = 2*10**6

    main(N,steps)





