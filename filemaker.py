
with open("data.txt","w") as outfile:
    for i in range(9000000):
        outfile.write('->' + str(i) + '\n')
