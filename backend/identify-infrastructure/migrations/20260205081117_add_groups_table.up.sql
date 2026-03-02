create table groups (
  id            text primary key not null,
  name          text not null unique,
  is_privileged boolean not null,
  created_at    datetime not null,
  updated_at    datetime not null
);
