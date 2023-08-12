-- Add migration script here
ALTER TABLE ministry_events RENAME TO _ministry_events;

CREATE TABLE ministry_events (
  id INTEGER PRIMARY KEY NOT NULL,
  assignee_name TEXT DEFAULT "" NOT NULL,
  assignee_id INTEGER,
  date TEXT NOT NULL,
  time TEXT,
  place TEXT DEFAULT "" NOT NULL,
  extra_info TEXT DEFAULT "" NOT NULL,

  FOREIGN KEY(assignee_id) REFERENCES persons(id)
);

INSERT INTO ministry_events (
    id
    , assignee_name
    , assignee_id
    , date
    , time
    , place
    , extra_info
  ) SELECT 
    id
    , assignee_name
    , assignee_id
    , date(scheduled_time , 'localtime')
    , strftime('%H:%M', scheduled_time, 'localtime')
    , place
    , extra_info
  FROM _ministry_events;

  DROP TABLE _ministry_events;

-- ALTER TABLE ministry_events ADD COLUMN date TEXT;
-- ALTER TABLE ministry_events ADD COLUMN time TEXT;
-- 
-- UPDATE ministry_events 
-- SET 
--   date = date(scheduled_time, 'localtime'),
--   time = time(scheduled_time, 'localtime');
-- 
-- ALTER TABLE ministry_events DROP COLUMN scheduled_time;
