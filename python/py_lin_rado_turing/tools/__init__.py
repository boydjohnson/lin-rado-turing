from .. import tm

def run_bb(prog: str, tape=[], check_rec=None, x_limit=100_000_000, check_blanks=False, samples=[]):
    return tm.run_bb(prog, tape, x_limit, check_rec, check_blanks, samples)
