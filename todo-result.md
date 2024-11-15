```
[geronimo@archlinux ~]$ cd /home/geronimo/Downloads/release/
[geronimo@archlinux release]$ ./todo
       ||    COMMAND LINE  TO DO    ||
||   - Rust Cli Todo Program -   ||
  -------------------------------
Loading task list...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
.
 List is empty, add a new task lil bro.
.
Choose action - Add task/Remove task/Quit (a/r/q) :
a
Type task to input:
A taks
   -- Updated List --
||   - Rust Cli Todo Program -   ||
  -------------------------------
Loading task list...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
1 | A taks
Choose action - Add task/Remove task/Quit (a/r/q) :
a
Type task to input:
task 2
   -- Updated List --
||   - Rust Cli Todo Program -   ||
  -------------------------------
Loading task list...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
1 | A taks
2 | task 2
Choose action - Add task/Remove task/Quit (a/r/q) :
a
Type task to input:
get a job
   -- Updated List --
||   - Rust Cli Todo Program -   ||
  -------------------------------
Loading task list...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
1 | A taks
2 | task 2
3 | get a job
Choose action - Add task/Remove task/Quit (a/r/q) :
q
Quitting program...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
1 | A taks
2 | task 2
3 | get a job
[geronimo@archlinux release]$ 2
bash: 2: command not found
[geronimo@archlinux release]$ ./todo
       ||    COMMAND LINE  TO DO    ||
||   - Rust Cli Todo Program -   ||
  -------------------------------
Loading task list...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
1 | A taks
2 | task 2
3 | get a job
Choose action - Add task/Remove task/Quit (a/r/q) :
r
Type index of task to remove:
2
   -- Updated List --
||   - Rust Cli Todo Program -   ||
  -------------------------------
Loading task list...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
1 | A taks
2 | get a job
Choose action - Add task/Remove task/Quit (a/r/q) :
r
Type index of task to remove:
1
   -- Updated List --
||   - Rust Cli Todo Program -   ||
  -------------------------------
Loading task list...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
1 | get a job
Choose action - Add task/Remove task/Quit (a/r/q) :
r
Type index of task to remove:
1
   -- Updated List --
||   - Rust Cli Todo Program -   ||
  -------------------------------
Loading task list...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
.
 List is empty, add a new task lil bro.
.
Choose action - Add task/Remove task/Quit (a/r/q) :
q
Quitting program...
Loading task list...
n | ---  TASKS ----------------
--|-----------------------------
.
 List is empty, add a new task lil bro.
.
[geronimo@archlinux release]$
```
