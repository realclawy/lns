# lns
Random project to count how many lines directories + files have, with the ability to exclude nested directories/files

From `lns help`:
lns (pronounced "lines" ig?) is a utility for getting a line-count for your projects

By default, lns will get the line count of all files found in src/, but it will check for a .lns file
 on the use directory, which determines its behaviour.

In the .lns file you can use:
    + to declare files to count towards line count (+\<file>)
    ++ for directories (++\<directory>)
    - to ignore a specific file (-\<file>)
    -- to ignore a specific directory.
    ignore_spaces_comments=true|false|both to make lns decide whether empty lines and comments count 
torwards the total (if you choose both it will display both, showing "\<lines> (\<lines_no_spaces> loc)
", like the cat platform does!)
    verbose=true|false to print all of the files' line counts. I personally enjoy this a lot

You can also use presets located in ~/.config/lns/ by putting their name.
To make a preset, you have to make a file called \<name>.conf, where \<name> is the name you want to gi
ve to it.
Inside the preset, put everything as you would on a .lns

What is the purpose of this project?
Well, mostly convenience and fun. It's so satisfactory to see how many lines I've written, so I decid
ed to take a more sophisticated manner of getting the lines.

Why is this project's name just "lns"?
Think of how many times I'll write this command on the terminal in hopes I have reached a bigger mile
stone.
Writing something like "getcodelines" eventually gets tiring, saying this out of experience.
