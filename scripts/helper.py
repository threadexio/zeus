#!/usr/bin/python3
import argparse
import os
import sys
import subprocess
from glob import glob

from colorama import Fore


def info(*m: str):
    print("{} => {}{}".format(Fore.CYAN, Fore.RESET, *m))
    pass


def error(*m: str):
    print("{} => {}{}".format(Fore.RED, Fore.RESET, *m))


def resolve_path(path: str) -> str:
    p = os.path.expandvars(path)

    if p.find("$") >= 0:
        error(f"Unset variable at: {path}")
        sys.exit(1)

    if p.startswith("/"):
        return p[1:]
    else:
        return p


def join_path(*path: str) -> str:
    final_path = os.path.sep.join(path)

    while final_path.find("//") >= 0:
        final_path = final_path.replace("//", "/")

    return final_path


def install(src: str, dst: str, mode: int = 644, strip: bool = False, extra_args: str = ""):
    if strip:
        extra_args += "-s"

    cmd = f"install -D -T -m {str(mode)} {extra_args} -- '{src}' '{dst}'"

    info(f"install: {src} -> {dst}")
    os.system(cmd)


def mkdir(dst: str):
    cmd = f"mkdir -p -- '{dst}'"
    os.system(cmd)


def install_overlay(src: str, dst: str):
    info(f"install_overlay: {src} -> {dst}")

    # this must NOT return hidden paths
    paths = glob("**", root_dir=src, recursive=True)

    for path in paths:
        src_path = join_path(src, path)
        dst_path = join_path(dst, resolve_path(path))

        try:
            if os.path.isdir(src_path):
                mkdir(dst_path)
            else:
                install(src_path, dst_path)
        except:
            continue

    # hooks
    hook_dir = join_path(src, join_path(".install_hooks.d"))
    if os.path.isdir(hook_dir):
        for hook in glob("*", root_dir=hook_dir):
            info(f"install_hook: {hook}")
            h = subprocess.run(os.path.abspath(join_path(
                hook_dir, hook)), cwd=dst)

            if h.returncode != 0:
                error(f"{hook}: exited with {h.returncode}")
                sys.exit(1)


if __name__ == "__main__":
    try:
        parser = argparse.ArgumentParser()
        subparsers = parser.add_subparsers(dest="subcommand")

        p = subparsers.add_parser('install_overlay')
        p.add_argument('src', type=str)
        p.add_argument('dst', type=str)

        p = subparsers.add_parser('install')
        p.add_argument('src', type=str)
        p.add_argument('dst', type=str)
        p.add_argument('--mode', type=int, default=644)
        p.add_argument('--strip', default=False, action="store_true")
        p.add_argument('--extra_args', type=str, default="")

        args = parser.parse_args()

        if args.subcommand == "install":
            install(src=args.src,
                    dst=args.dst,
                    mode=args.mode,
                    strip=args.strip,
                    extra_args=args.extra_args)
        elif args.subcommand == "install_overlay":
            install_overlay(src=args.src,
                            dst=args.dst)
    except KeyError as e:
        error(f"Unset env var: {e}")
        sys.exit(1)
