#!/usr/bin/python

import sys



for line in sys.stdin:
  print eval(line[:-1])
