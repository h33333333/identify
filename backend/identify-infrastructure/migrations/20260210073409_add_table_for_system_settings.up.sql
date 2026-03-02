create table system_settings (
  id             integer primary key not null check(id = 1),
  is_initialized boolean not null,
  created_at     datetime not null,
  updated_at     datetime not null
);

-- Insert the record to ensure it always exists.
insert into system_settings (
  is_initialized,
  created_at,
  updated_at
) values (
  false,
  datetime('now'),
  datetime('now')
);
