
with open("data.txt","w") as outfile:
    for i in range(4000000):
        outfile.write('->' + str(i) + '\n')
