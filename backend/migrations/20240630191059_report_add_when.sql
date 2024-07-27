-- Step 1: Add the new column 'when_iso' to the 'Report' table
ALTER TABLE Report ADD COLUMN when_iso TEXT;

-- Step 2: Update the existing rows to set the 'when_iso' column to the current time in ISO format
UPDATE Report SET when_iso = strftime('%Y-%m-%dT%H:%M:%SZ', 'now');

-- Step 3: Ensure the 'when_iso' column is not nullable
CREATE TABLE Report_new (
  	id UNSIGNED BIG INT PRIMARY KEY,
    title TEXT NOT NULL,
    ip_address TEXT,
  	data TEXT NOT NULL,
    when_iso TEXT NOT NULL
);

-- Copy data from the old table to the new table
INSERT INTO Report_new (id, title, ip_address, data, when_iso)
SELECT id, title, ip_address, data, when_iso
FROM Report;

-- Drop the old table
DROP TABLE Report;

-- Rename the new table to the original table name
ALTER TABLE Report_new RENAME TO Report;