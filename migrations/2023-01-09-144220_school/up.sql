-- Your SQL goes here
CREATE TABLE teachers (
  name VARCHAR PRIMARY KEY NOT NULL,
  class INTEGER NOT NULL,
  course VARCHAR NOT NULL
);

CREATE TABLE students (
  name VARCHAR NOT NULL,
  roll_number INTEGER PRIMARY KEY NOT NULL,
  class INTEGER NOT NULL,
  english_score INTEGER NOT NULL,
  computer_score INTEGER NOT NULL,
  math_score INTEGER NOT NULL,
  sports_score INTEGER NOT NULL
);
