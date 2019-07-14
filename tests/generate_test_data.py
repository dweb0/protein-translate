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

    if len(sys.argv) > 1:
        try:
            num_sequences = int(sys.argv[1])
        except ValueError:
            sys.stderr.write('Please enter an integer.\n')
    else:
        sys.stderr.write('USAGE: python3 generate_test_data.py NUM_SEQUENCES\n')
        sys.exit(1)


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
        for i in range(num_sequences):
            seq = random_seq(VARIATIONS[randint(0, 3)])
            outfile.write(seq + "," + peptide(seq) + "\n")
