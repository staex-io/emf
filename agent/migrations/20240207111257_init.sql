create table entities (account_id text primary key, created_at integer);

create table sub_entities (
  entity text not not references entities (account_id),
  account_id text primary key,
  location text,
  created_at integer
);

create table spikes (
  sub_entity text not null references sub_entities (account_id),
  value text,
  created_at integer,
);

create table too_much_spikes (
  sub_entity text not null references sub_entities (account_id),
  created_at integer,
);

create table certificate_ready (
  sub_entity text not null references sub_entities (account_id),
  created_at integer
);
