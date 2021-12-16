import sys
sz = 1000000
fp = open("py_player_out", "w")
for i in range(sz):
    sys.stdout.write("hi\n")
    #  print("hi")
sys.stdout.flush()
#  sys.stdin.readline()
#  input()
for i in range(sz):
    fp.write(sys.stdin.readline())

#  input()
#  open("p1_out", 'w').write("p1_out")
