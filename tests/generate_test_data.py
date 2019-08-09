""" Generate random nucleotide sequences and translate
using Biopython

Output
------
2 column csv: DNA/RNA sequence and translated peptide 
"""

from Bio.Seq import Seq
from random import choice, randint


def random_seq(candidates: str):
    """ Generate a random sequence between 0 and 1000 nucleotides

    Parameters
    ----------
    candidates: str
        nucleotides to chose from (use ATCG for dna, AUCG for rna)
    
    Returns
    -------
    str
        random nucleotide sequence
    """
    return "".join(choice(candidates) for _ in range(randint(0, 1000)))


def peptide(seq: str):
    """ Translate the sequence to peptide """
    return str(Seq(seq).translate())


if __name__ == "__main__":

    import os
    import sys
    import argparse

    parser = argparse.ArgumentParser(prog='generate_test_data')
    parser.add_argument(
        'num_sequences',
        type=int,
        help='The number of sequences to randomly generate'
    )
    args = parser.parse_args()

    # Type of sequence
    # We will use this to randomly select a type of sequence to generate
    VARIATIONS = {
        0: 'ATCG',   # dna
        1: 'atcg',   # lowercase dna
        2: 'AUCG',   # rna
        3: 'aucg',   # lowercase rna
    }

    # Write sequences to named test file
    script_dir = os.path.dirname(os.path.realpath(__file__))
    output = os.path.join(script_dir, 'test_data.csv')

    with open(output, 'wt') as outfile:
        for i in range(args.num_sequences):
            seq = random_seq(VARIATIONS[randint(0, 3)])
            outfile.write(seq + "," + peptide(seq) + "\n")
