#!/usr/bin/env python3
"""
This is a command-line tool to extract all PDB files from a Quiver file.

Usage:
    qvextract.py <quiver_file>
"""

import click
from quiver_pdb import extract_pdbs

@click.command()
@click.argument("quiver_file", type=click.Path(exists=True, dir_okay=False))
def main(quiver_file):
    """
    Extract all PDB files from a Quiver file.
    """
    extract_pdbs(quiver_file)

if __name__ == "__main__":
    main()
