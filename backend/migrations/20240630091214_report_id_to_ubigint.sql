-- Step 1: Create a new table with the desired schema
CREATE TABLE Report_new (
    id UNSIGNED BIG INT PRIMARY KEY,
    title TEXT NOT NULL,
    ip_address TEXT,
    data TEXT NOT NULL
);

-- Step 2: Copy the data from the old table to the new table
INSERT INTO Report_new (id, title, ip_address, data)
SELECT id, title, ip_address, data FROM Report;

-- Step 3: Drop the old table
DROP TABLE Report;

-- Step 4: Rename the new table to the original table name
ALTER TABLE Report_new RENAME TO Report;
