-- Your SQL goes here
CREATE TABLE teachers (
  teacher_id INTEGER PRIMARY KEY NOT NULL,
  teacher_name VARCHAR NOT NULL,
  class_name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  FOREIGN KEY (class_name) REFERENCES Subs(class_name)
);

CREATE TABLE students (
  student_id INTEGER PRIMARY KEY NOT NULL,
  student_name VARCHAR NOT NULL,
  contact_info VARCHAR NOT NULL,
  email VARCHAR NOT NULL
);

CREATE TABLE subs (
  class_id INTEGER PRIMARY KEY NOT NULL,
  class_name VARCHAR NOT NULL,
  teacher_id INTEGER,
  FOREIGN KEY (teacher_id) REFERENCES Teachers(teacher_id)
);

CREATE TABLE grades (
  grade_id INTEGER PRIMARY KEY NOT NULL,
  student_id INTEGER NOT NULL,
  class_name VARCHAR NOT NULL,
  assignment_score INTEGER NOT NULL,
  test_score INTEGER NOT NULL,
  FOREIGN KEY (student_id) REFERENCES Students(student_id),
  FOREIGN KEY (class_name) REFERENCES Classes(class_name)
);
