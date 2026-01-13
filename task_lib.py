import os
import sys
import subprocess
import platform
import re

RED = 91
YELLOW = 33
GREEN = 92
def colorize(s, color_code):
    return f"\x1b[{color_code}m{s}\x1b[0m"
def replacer(color, skip_begin = 0):
    return lambda match : colorize(match.group(0)[skip_begin:], color)
def mod_failed(s):
    return re.sub("(?<! 0)[ ]failed", replacer(RED), s) 
def mod_not_yet_implemented(s):
    return re.sub("not yet implemented", replacer(RED), s) 
    
def mod_passed(s):
    return re.sub("[ ]passed", replacer(GREEN), s) 

def mod_link(s):
    color = YELLOW
    full_path_r = r" [^\. ]*rainbow[\/\\]src[\/\\][^\:]+(\:[0-9]+){0,1}\:[0-9]+"
    rel_path_w_r = r"\.[\/\\]?src[\/\\][^\:]+(\:[0-9]+){0,1}\:[0-9]+"
    rel_path_l_r = r" [\/\\]?src[\/\\][^\:]+(\:[0-9]+){0,1}\:[0-9]+"
    s = re.sub(full_path_r, replacer(color), s) 
    s = re.sub(rel_path_w_r, replacer(color, 1), s) 
    s = re.sub(rel_path_l_r, replacer(color), s) 
    return s

def mod(s):
    s = mod_failed(s)
    s = mod_not_yet_implemented(s)
    s = mod_passed(s)
    s = mod_link(s)
    return s

def run_command(command, cwd, envs, mod):
    env = os.environ.copy()
    for e in envs:
        env[e[0]] = e[1]

        
    if platform.system() == "Windows":
        executable = None
    else:
        executable = '/bin/bash'

    result = subprocess.run(command, env=env, shell=True, capture_output=mod!=None, 
                            executable=executable, cwd=cwd, text=True)
    if mod != None:
        print(mod(result.stdout))
        print(mod(result.stderr))

def run_cargo(command, cwd, no_warning = False, backtrace= False, use_mod= False, target_dir = "", release = False, features = []):
    command = f"cargo {command}"
    if target_dir != "":
        command += f" --target-dir {target_dir}"
    if release :
        command += f" --release"
    if len(features) != 0:
        features_s = " ".join(features)
        command += f' --features "{features_s}"'
    envs = []
    if no_warning:
        envs.append(("RUSTFLAGS", "-Awarnings"))
    if backtrace:
        envs.append(("RUST_BACKTRACE", "1"))
    mod_f = None
    if use_mod:
        mod_f = mod
        
    print("------------------------------")
    print(command)
    print("------------------------------")
    run_command(command, cwd, envs, mod_f)
