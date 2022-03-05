

def test_import():
    import megamerge

def test_do_chonk():
    import megamerge
    import numpy as np
    seg = np.array([
        [0,1,6],
        [1,5,8]
    ]).astype("f8")
    
    dat = np.array([
        [0,3,6],
        [3,6,9]
    ]).astype("f8")
    
    res = megamerge.do_chonk(
        seg,
        dat,
    )
    print(res)