-- Add migration script here
CREATE TABLE ministry_events (
  id INTEGER PRIMARY KEY NOT NULL,
  assignee_name TEXT DEFAULT "" NOT NULL,
  assignee_id INTEGER,
  scheduled_time TEXT NOT NULL,
  place TEXT DEFAULT "" NOT NULL,
  extra_info TEXT DEFAULT "" NOT NULL,

  FOREIGN KEY(assignee_id) REFERENCES persons(id)
);
