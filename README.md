## Introduction
RedHareDB is my toy implementation of a relational database, built in Rust. I have two primary goals for this project:

- Learn Rust! I typically work in Python and JS. In my mind, Python is most suited to domains like data analysis and JS is most suited to web development. While my opinion is not yet well-founded, a database feels close to a quintessential Rust project. 

- Build a better understanding of database internals. I use database tools like PostGIS and DuckDB pretty consistently, but I don't yet have a good understanding of _how_ a database works. It's my hope that by attempting to build one myself, I'll come to understand them a bit more.

## Current Scope/Features
As it stands, this is not yet a complete database. table.rs features most of the actual logic, which is limited to basic CRUD functionality. However, main.rs isn't really written like the entrypoint for a database. Instead, it features something like a series of integration tests demonstrating that my table.rs primatives work. Putting that aside, I think I've made some progress. RedHare has support for multiple tables via "database" abstraction. Additionally data values are encoded with their type and (if appropriate) length, thereby not relying on null/terminating chars. 

## Lessons Learned
An issue I'm running into is that I want to make sustainable/scalable choices, but I think I can only really come to understand those by keeping at this and noting what choices necessitate really annoying refactors six months later. I haven't done this yet, but I should probably look into open-source databases and see what I can learn.

I'm also struggling a bit to rebuild my lower-level language muscles. It's been ~5 years since I studied C++ in my undergrad, and my professional work since then has been more closely related to designing effective SQL queries. I'm hopeful that intentionally uncomfortable personal projects will make me a much more competent programmer. 

## Planned Changes
1. Integration/Unit Tests & Error Handling
2. Simple CLI resembling psql
3. CSV support
4. Basic SQL support
5. Indexes


