create table users (
  id              text primary key not null,
  email           text not null unique,
  first_name      text not null,
  last_name       text null,
  password_hash   text null,
  created_at      datetime not null,
  updated_at      datetime not null
);
