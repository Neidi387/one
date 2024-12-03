# Read input Split new lines
# Split new lines
# Split space
# Join first and second element to array
# Sort lists
# iterate and calculate distance
# Sum of list

input = open("src/input.txt").read()
lines = input.split("\n")
left = []
right = []
for line in lines:
    locations = line.split("   ")
    left.append(int(locations[0]))
    right.append(int(locations[1]))

left.sort()
right.sort()

distances = [None] * len(lines)

for i in range(len(lines)):
    distances[i] = abs(left[i] - right[i])

distance = sum(distances)
print(distance)
