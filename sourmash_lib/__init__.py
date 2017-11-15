import sys

import sourmash

sys.modules[__name__] = sys.modules['sourmash']
#sys.modules[__name__] = __import__('sourmash')
