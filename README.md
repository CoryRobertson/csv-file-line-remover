# csv-file-line-remover
This program removes lines based on a divisor, its really simple but I wanted to make it to go with my temperature sensor logging software.
It also has a deduplication feature, that looks at a csv line, skips the first cell, then deduplicates based on the second two cells. It decuplicates only up until a non-duplicate data point. E.g. [10,20,20,30,20,10,5,5] -> [10,20,30,20,10,5]
