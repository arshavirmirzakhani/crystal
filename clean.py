import subprocess
import os

for p in ["crystal","crystal_bin","crystal_editor"]:
    
    print("\n-- Cleaning: ",p," --\n")    

    os.chdir(p)
    subprocess.run(["cargo","clean"])
    os.chdir("..")
    
