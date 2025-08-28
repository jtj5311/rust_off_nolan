import numpy as np
from numba import njit


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

   
    hot_loop(array, steps)



if __name__ == "__main__":
    N = 10**6
    steps = 10**6

    main(N,steps)





