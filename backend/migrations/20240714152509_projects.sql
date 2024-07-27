-- NOTE: This migration will clear reports since reports now will require projects to be associated to it

-- Create a new table to define projects
CREATE TABLE Project (
    id TEXT PRIMARY KEY,
    owner_id TEXT NOT NULL,
    name TEXT NOT NULL,

    FOREIGN KEY (owner_id) REFERENCES User(id)
      ON UPDATE CASCADE
      ON DELETE CASCADE
);

-- Create the updated Report table
CREATE TABLE Report_new (
    id UNSIGNED BIG INT NOT NULL,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL,
    ip_address TEXT,
    data TEXT NOT NULL,
    when_iso TEXT NOT NULL,
    resolve_status TEXT NOT NULL DEFAULT 'unresolved',

    PRIMARY KEY (project_id, id),
    FOREIGN KEY (project_id) REFERENCES Project(id)
      ON UPDATE CASCADE
      ON DELETE CASCADE
);

-- Drop the existing Report table
-- NOTE: This clears existing reports
DROP TABLE Report;

-- Rename the new report table back to `Report`
ALTER TABLE Report_new RENAME TO Report;
