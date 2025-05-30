#!/usr/bin/env python3
"""
This tool combines multiple PDB files into a Quiver-compatible stream.

Usage:
    qvfrompdbs.py <pdb1> <pdb2> ... <pdbN> > output.qv
"""

import sys
import click
from quiver_pdb import qvfrompdbs

@click.command()
@click.argument("pdb_files", nargs=-1, required=True)
def main(pdb_files):
    """
    Converts one or more PDB files into a Quiver-formatted stream.
    Output is printed to stdout.
    """
    sys.stdout.write(qvfrompdbs(list(pdb_files)))

if __name__ == "__main__":
    main()
