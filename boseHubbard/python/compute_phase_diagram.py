#!/usr/bin/env python3
# -*- coding: iso-8859-1 -*-

""" Use to compute the phase diagram of various bosonic Hubbard Model """

import numpy as np
from scipy.optimize import minimize_scalar
from joblib import Parallel, delayed
import itertools  
import matplotlib.pyplot as plt 

class Models:
    def __init__(self, n_max):
        self.n_max = n_max
        self.create_local_operators()

    def create_local_operators(self):
        self.a = np.diag(np.sqrt(np.arange(self.n_max)+1), k=1)
        self.a_dag = self.a.T 
        self.n = self.a_dag @ self.a
        self.I = np.identity(self.n_max+1)

    def gs_energy(self, ham):
        w, v = np.linalg.eigh(ham) 
        return w[0]
    
    def full_ground_state(self, ham):
        w, v = np.linalg.eigh(ham) 
        return w[0], v[:,0]
    
    def find_psi(self, t, mu): 
        def e_min(psi):
            return self.gs_energy(self.get_BHMF_ham(t, mu, psi)) 
        res=minimize_scalar(e_min) 
        return t, mu, res.x
    
    def find_psi_variational(self, t, mu, initial_guess):
        w, v = self.full_ground_state(self.get_BHMF_ham(t, mu, initial_guess))

        psi = 0
        for k in range(0, self.n_max):
            psi += v[k]*v[k+1]*np.sqrt(k+1)
        # psi *= 2
        
        while np.abs(psi - initial_guess) > 0.0001:
            # print(psi)
            initial_guess = psi
            w, v = self.full_ground_state(self.get_BHMF_ham(t, mu, initial_guess))

            psi = 0
            for k in range(0, self.n_max):    
                psi += v[k]*v[k+1]*np.sqrt(k+1)
            # psi *= 2

        return psi

    def get_BHMF_ham(self, t, mu, psi):
        ham = -6*t * (psi * (self.a + self.a_dag) - psi**2 * self.I) + 0.5 * self.n @ (self.n - self.I) - mu * self.n
        return ham

def get_blocks(iterator, num_cores, step=1):
    """Function to slice iterator"""
    n_frames = len(iterator)
    n_blocks = num_cores
    n_frames_per_block = n_frames // n_blocks
    blocks = [
        range(i * n_frames_per_block, (i + 1) * n_frames_per_block, step)
        for i in range(n_blocks - 1)
    ]
    blocks.append(range((n_blocks - 1) * n_frames_per_block, n_frames, step))
    return blocks

def analyze_block(iterator, blockslice, func, *args, **kwargs):
    """Function to analyse slices"""
    result = [
        func(_[0], _[1], *args, **kwargs)
        for _ in iterator[blockslice.start : blockslice.stop : blockslice.step]
    ]
    return result

def flatten(lst):
    """Flatten array from parallel analysis"""
    return np.array(list(itertools.chain.from_iterable(lst)))


BHMF = Models(50)
size = 500
init = 1
ti = np.linspace(0, 0.05, size) 
mui = np.linspace(0, 3, size)
pairs = list(itertools.product(ti, mui))


# blocks = get_blocks(pairs, 8)
# res = Parallel(n_jobs=8, verbose=10)(
#     delayed(analyze_block)(
#         pairs, block, BHMF.find_psi
#     )
#     for block in blocks
# )
# res = np.absolute(flatten(res))

# res_reshaped = res.reshape((500, 500, 3))
# x = res_reshaped[:, :, 0]
# y = res_reshaped[:, :, 1]
# z = res_reshaped[:, :, 2]

blocks = get_blocks(pairs, 8)
res = Parallel(n_jobs=8, verbose=10)(
    delayed(analyze_block)(
        pairs, block, BHMF.find_psi_variational, init
    )
    for block in blocks
)
res = np.absolute(flatten(res))

# res_reshaped = res.reshape((500, 500))

# fig, ax = plt.subplots(1, 2)
# ax[0].imshow(z.T, origin='lower', extent= [0, 0.4, 0, 3], aspect='auto')
# ax[1].imshow(res_reshaped.T, origin='lower', extent= [0, 0.4, 0, 3], aspect='auto')
# plt.show()