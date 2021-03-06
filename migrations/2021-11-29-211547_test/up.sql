-- Your SQL goes here

create table commands (
       id integer primary key autoincrement,
       name text,
       value text,
       activate boolean
);

insert or replace into commands values(1, "echo", "echo (id) (value)", true);
insert or replace into commands values(2, "echo", "echo (id) (value)", false);

create table timers (
       id integer primary key autoincrement,
       name text,
       value text,
       time_th bigint,
       message_th bigint,
       activate boolean
);

insert or replace into timers values(1, "sub", "Pensez a votre Prime mais ne subbez pas ici", 600, 10, true);
insert or replace into timers values(2, "nain", "nain", 600, 10, false);

create table twitch (
       id integer primary key autoincrement,
       token text,
       refresh_token text,
       expire_in integer,
       generation_date date
);

insert or replace into twitch values(1, "eiuau", "euiaua", 604800, "1970-01-01");
