#!/bin/bash

#PID=".`readlink /proc/self`"
PID=""

rm -f /tmp/interaction$PID.txt
tee -a /tmp/interaction$PID.txt | z3 -in -smt2 | tee -a /tmp/interaction$PID.txt
