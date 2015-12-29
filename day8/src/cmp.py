#!/usr/bin/python

import sys


f1 = open("test.txt")
f2 = open("output.txt")

i = 0
for l1,l2 in zip(f1.readlines(),f2.readlines()):
  i+=1
  l1 = l1[:-1]
  l2 = l2[:-1]
  if l1!=l2:
    print "%d: %s is not %s" % (i,l1,l2)
