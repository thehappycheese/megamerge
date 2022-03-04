

def test_import():
    import megamerge

def test_do_chonk():
    import megamerge
    import numpy as np
    seg = np.array([[2,2,2],[2,5,6]]).astype("f8")
    dat = np.array([[1,2,3],[4,5,6]]).astype("f8")
    
    res = megamerge.do_chonk(seg, dat)
    assert res is seg