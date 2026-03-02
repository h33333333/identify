create table users_groups (
  id         integer primary key autoincrement not null,
  user_id    text references users(id) on delete cascade not null,
  group_id   text references groups(id) on delete cascade not null,
  created_at datetime not null default (datetime('now','utc')),

  constraint users_groups_uni unique (user_id, group_id)
);

