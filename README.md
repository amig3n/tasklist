(t)a(s)k (l)ist - simple cli-based tasklist for the sake of becoming a Rustacean

This project is for learning purposes. Written in Rust to practice the language basics

Commands:
- `add <task description>`: Add a new task to the list.
- `list`: Display all tasks with their status (completed or pending).
- `finish <task id>`: Mark a task as completed using its ID.
- `finish-many <task id 1> <task id 2> ...`: Mark multiple tasks as completed using their IDs.
- `delete <task id>`: Remove a task from the list using its ID.
- `help`: Show this help message.

It stores tasks in a local .json file in the home directory of current user.

## Implementation list
- [x] CRD
- [x] Persistance
- [x] Properly handle errors
- [x] Mark multiple tasks as finished in one shot
- [ ] Unit tests
- [x] Rendering component
-- [x] Table rendering
-- [x] Colored output
- [ ] Update task
-- [ ] Update description
-- [ ] Update deadline
- [ ] Delete multiple tasks in one shot
- [ ] More statuses
- [ ] Subtasks
-- [ ] Tree structure
- [ ] Tasklist encryption?
-- Symmetric encryption with password prompt
- [ ] multiple vaults support?
-- Using static configfile inside home dir?
- Something not yet defined
