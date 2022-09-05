inputs = [[14,0,15,12,11,11,3,5,1,6,8,4,9,1,8,4]]

cycles = 0
running = True
while (running):
    currentinput = list(inputs[-1])
    maximum = 0
    index = 0
    for i in range(0,len(currentinput)):
        if (currentinput[i] > maximum):
            maximum = currentinput[i]
            index = i
    currentinput[index] = 0
    for i in range(index+1, maximum+index+1):
        j = i
        while (j >= len(currentinput)):
            j -= len(currentinput)
        currentinput[j] += 1
    cycles += 1
##    print(currentinput)
##    print(cycles)
    for inp in inputs:
        if (inp == currentinput):
            print(cycles, 'cycles')
            print(currentinput, 'currentinput')
            print(inp, 'inp')
            running = False
    inputs.append(currentinput)
