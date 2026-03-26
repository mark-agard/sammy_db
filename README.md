# SammyDB

SammyDB (named after my cat) is an attempt to create a simple, fast, and reliable database engine in Rust with no (or at least minimal) AI assistance. I do not intend to create a full-featured database system; I'd like to see how close I can get to a production-grade database writing everything from scratch. I'm also using this as an opportunity to learn Rust- I am a Python and JavaScript developer by trade, though I have some past C++ experience. My impression is that a database engine is a great way to learn a systems programming language like Rust.

## Scope

What constitutes a "database" for this project? The truth is: I don't know. I am certainly not a database expert, nor do I have a particularly robust understanding of database internals. That said, I use databases in my work, so I can list some features this project will probably need:

- Ability to efficiently store/retrieve data of varied types in some sort of binary file system.
- Some sort of index structure to enable efficient querying.
- SQL parser/query support (I have zero idea how this works)
- Concurrency system (simultaneous reads and writes)
- CLI and/or GUI for interacting with the database

## Resources

As I consume resources to make this somewhat viable, I'll list them here for reference.

- https://www.deepintodev.com/blog/how-databases-store-your-tables-on-disk
    - I happened upon this blog. I don't yet have the ability/knowledge to evaluate it, but turning "store data on disk" into "store rows of data in pages" seems helpful.

## Milestones

I can only imagine building a database in Rust is a monumental undertaking, especially since this is my first Rust project. I certainly can't get into the technically demanding portions of the work yet. Let's split this into more approachable asks that I'm pretty sure will contribute to the whole:

- I need a system to read data from memory, write it into a file, and then read it back into memory.
    - [ ] If I have 4 rows of data in memory, can I write them to a file?
    - [ ] Can I read those 4 rows of data back into memory accurately?
    - [ ] If I have more data than can be stored in one page/file, can I write to multiple pages/files, and read them back in?

- I need a way to index data, so that partial reads don't read the entire set of files.
    - [ ] If I want to select one row from a table with 1000 rows, can I use an index to find that row without reading all 1000 rows?
    - [ ] Does that index update as the table changes?

- I need a way to modify data in the database.
    - [ ] Can I insert a row into a table?
    - [ ] Can I update a row in a table?
    - [ ] Can I delete a row from a table?

- I need a way to query data from the database.
    - [ ] Can I select all rows from a table?
    - [ ] Can I select specific rows from a table?
    - [ ] Can I select specific columns from a table?

I think checking these boxes will get me to the first real step towards something that meaningfully resembles a database. If something can store data, access that data (including specific portions of it), and update it, then I'm making real progress.


    