# Project Devlog for XJB-DB

## Project Goal
the Goal of XJB-DB is to build a:
* Disk Based RDBMS
* with rich support of query execution

## Overall architecture
    the XJB DB uses a traditional 3-tier architecture: inbound-exec-storage
        inbound deals with client interaction with server
        exec deals with query execution
        storage deals with persistence and storage management
    Transaction, Concurrency Control and Disaster Recovery will be consider as aspects that goes through all 3 tiers

## Milestone 1: Get a in memory, naive CRUD only, single-threaded system up & running 
* [ ] initial CLI inbound
* [ ] initial query processing
    * [ ] CREATE TABLE
    * [ ] INSERT INTO
    * [ ] SELECT WHERE
    * [ ] DELETE
    * [ ] DROP TABLE
* [ ] in-memory storage and overall interface
