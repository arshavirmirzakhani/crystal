import subprocess
import os

for p in ["crystal","crystal_bin","crystal_editor"]:
    
    print("\n-- Building: ",p," --\n")    

    os.chdir(p)
    subprocess.run(["cargo","build"])
    os.chdir("..")
    
