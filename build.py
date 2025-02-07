import subprocess
import os
import sys

for p in ["crystal","crystal_bin","crystal_editor"]:
    
    print("\n-- Building: ",p," --\n")    

    os.chdir(p)
    subprocess.run(["cargo","build"]+sys.argv[1:])
    os.chdir("..")