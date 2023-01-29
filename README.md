# Mind organizer

Mind organizer is a simple command line tool for people who like to organize themselves using to-do lists and Markdown files, but don't want to reorganize all the entries very often.

## For example:
We start with an unorganized file like this:
```
[] task one
- Idea 1
[x] completed task
[] to-do

- awesome idea
- [] and another one
```

After running the `cargo run` command, this is what we get:

```
Completed tasks:
[x] completed task

To-do's:
[] task one
[] to-do
[] and another one

Ideas:
- Idea 1
- awesome idea
```

Much better, right?