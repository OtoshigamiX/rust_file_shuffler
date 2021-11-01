# rust_file_shuffler
Project for my rust training - simple application which randomly shuffles files and gets the new order, useful for shuffling images for quizzes

It has several modes of work:
Default - it takes provided directory, excludes files with specific extensions, takes the rest, and shuffles the filenames (preserving extensions). 
Output is stored in a text file containing previous filename and new filename in each line.


Simple - this mode expects all filenames to be numbers. It generates a file containing a single column - only new filenames (old filenames are the line numbers)


Flatten - also expects all filenames to be numbers. Renames files so that there won't be any gaps in the filenames, for example if you had files named 1 2 5 7
,those files will be renamed to 1 2 3 4 in that order.
