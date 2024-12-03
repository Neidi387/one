# Read input Split new lines
# Split new lines
# Split space
# Join first and second element to array
# Sort lists
# iterate and calculate distance
# Sum of list

import re

input = open("input.txt").read()

locationsFlat = re.split(r'\s{3}|\n', input)
left = locationsFlat[0::2]
right = locationsFlat[1::2]

left.sort()
right.sort()

distances = []
for i in range(len(left)):
    distance = abs(int(left[i]) - int(right[i]))
    distances.append(distance)

distance = sum(distances)
print(distance)
