import rbind

i_stck = rbind.int_stack()

i_stck.push(1)
i_stck.push(2)
i_stck.push(3)

head = i_stck.head()
print(head)
i_stck.pop()
head = i_stck.head()
print(head)
i_stck.pop()
head = i_stck.head()
print(head)

